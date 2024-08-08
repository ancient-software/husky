//! meant for immediate mode gui
use futures_util::{SinkExt, StreamExt};
use husky_print_utils::p;
use notify_change::NotifyChange;
use std::sync::Arc;
use thiserror::Error;
use tokio_tungstenite::tungstenite::{self, Message};
use tracing::*;

const ORDERING: core::sync::atomic::Ordering = core::sync::atomic::Ordering::SeqCst;

/// non-blocking
///
/// all apis are sync
pub struct ImmediateWebsocketClientConnection<Request, Response, Notifier>
where
    Notifier: NotifyChange,
{
    tokio_runtime: Arc<tokio::runtime::Runtime>,
    creation_status: CreationStatus<Request, Response, Notifier>,
    request_tx: tokio::sync::mpsc::Sender<Request>,
    response_rx: tokio::sync::mpsc::Receiver<Response>,
    communication_status: Arc<AtomicCommunicationStatus>,
    /// must use std JoinHandle
    launch_join_handle: Option<tokio::task::JoinHandle<()>>,
}

#[atomic_enum::atomic_enum]
pub enum CommunicationStatus {
    Creation,
    AwaitingRequest,
    DeserializingRequest,
    AwaitingResponse,
    SerializingResponse,
    ResponseReady,
}

pub enum CreationStatus<Request, Response, Notifier>
where
    Notifier: NotifyChange,
{
    Await(Arc<std::sync::Mutex<CreationAwaitStatus<Request, Response, Notifier>>>),
    Ok,
    Err(WebsocketClientConnectionError),
}

pub enum CreationAwaitStatus<Request, Response, Notifier>
where
    Notifier: NotifyChange,
{
    Await,
    Ok {
        stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        response: tungstenite::handshake::client::Response,
        request_rx: tokio::sync::mpsc::Receiver<Request>,
        response_tx: tokio::sync::mpsc::Sender<Response>,
        notifier: Notifier,
    },
    Err(WebsocketClientConnectionError),
}

#[derive(Debug, Error)]
pub enum WebsocketClientConnectionError {
    #[error("Send request while creation")]
    SendRequestWhileCreation,
    #[error("Send request while deserializing request")]
    SendRequestWhileDeserializingRequest,
    #[error("Send request while awaiting response")]
    SendRequestWhileAwaitingResponse,
    #[error("Send request while serializing response")]
    SendRequestWhileSerializingResponse,
    #[error("Send request while response not processed")]
    SendRequestWhileResponseNotProcessed,
}

impl<Request, Response, Notifier> ImmediateWebsocketClientConnection<Request, Response, Notifier>
where
    Request: Send + 'static,
    Response: Send + 'static,
    Notifier: NotifyChange + 'static,
{
    pub fn new(
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        server_address: String,
        notifier: Notifier,
    ) -> Self {
        let await_status = Arc::new(std::sync::Mutex::new(CreationAwaitStatus::Await));
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(1);
        let (response_tx, response_rx) = tokio::sync::mpsc::channel(1);
        tokio_runtime.spawn({
            let await_status = await_status.clone();
            async move {
                tracing::info!("server_address = {server_address}");
                match tokio_tungstenite::connect_async(server_address).await {
                    Ok((stream, response)) => {
                        *await_status.lock().unwrap() = CreationAwaitStatus::Ok {
                            stream,
                            response,
                            request_rx,
                            response_tx,
                            notifier,
                        }
                    }
                    Err(e) => {
                        p!(e);
                        todo!()
                    }
                }
            }
        });
        Self {
            tokio_runtime,
            creation_status: CreationStatus::Await(await_status),
            request_tx,
            response_rx,
            communication_status: Arc::new(AtomicCommunicationStatus::new(
                CommunicationStatus::Creation,
            )),
            launch_join_handle: None,
        }
    }
}

impl<Request, Response, Notifier> ImmediateWebsocketClientConnection<Request, Response, Notifier>
where
    Request: Send + 'static,
    Response: Send + 'static,
    Notifier: NotifyChange + 'static,
{
    pub fn creation_status(&self) -> &CreationStatus<Request, Response, Notifier> {
        &self.creation_status
    }

    pub fn error(&self) -> Option<&WebsocketClientConnectionError> {
        match self.creation_status {
            CreationStatus::Err(ref e) => Some(e),
            _ => None,
        }
    }
}

pub trait NeedResponse {
    fn need_response(&self) -> bool;
}

#[cfg(feature = "serde_json")]
impl<Request, Response, Notifier> ImmediateWebsocketClientConnection<Request, Response, Notifier>
where
    Request: serde::Serialize + Send + 'static + NeedResponse + Default,
    Response: for<'a> serde::Deserialize<'a> + Send + 'static,
    Notifier: NotifyChange + 'static,
{
    pub fn try_send_request(
        &mut self,
        request: Request,
    ) -> Result<(), WebsocketClientConnectionError> {
        self.refresh();
        match self.communication_status.load(ORDERING) {
            CommunicationStatus::Creation => {
                Err(WebsocketClientConnectionError::SendRequestWhileCreation)
            }
            CommunicationStatus::AwaitingRequest => {
                self.request_tx.blocking_send(request).map_err(|_e| todo!())
            }
            CommunicationStatus::DeserializingRequest => {
                Err(WebsocketClientConnectionError::SendRequestWhileDeserializingRequest)
            }
            CommunicationStatus::AwaitingResponse => {
                Err(WebsocketClientConnectionError::SendRequestWhileAwaitingResponse)
            }
            CommunicationStatus::SerializingResponse => {
                Err(WebsocketClientConnectionError::SendRequestWhileSerializingResponse)
            }
            CommunicationStatus::ResponseReady => {
                Err(WebsocketClientConnectionError::SendRequestWhileResponseNotProcessed)
            }
        }
    }

    pub fn try_recv(&mut self) -> Option<Response> {
        loop {
            match self.refresh() {
                StatusChanged::True => continue,
                StatusChanged::False => break,
            }
        }
        match self.communication_status.load(ORDERING) {
            CommunicationStatus::Creation
            | CommunicationStatus::AwaitingRequest
            | CommunicationStatus::DeserializingRequest
            | CommunicationStatus::AwaitingResponse
            | CommunicationStatus::SerializingResponse => None,
            CommunicationStatus::ResponseReady => {
                let response = self.response_rx.blocking_recv();
                self.communication_status
                    .store(CommunicationStatus::AwaitingRequest, ORDERING);
                response
            }
        }
    }

    fn refresh(&mut self) -> StatusChanged {
        let await_result = match self.creation_status {
            CreationStatus::Await(ref await_status) => match std::mem::replace(
                &mut *await_status.lock().unwrap(),
                CreationAwaitStatus::Await,
            ) {
                CreationAwaitStatus::Await => return StatusChanged::False,
                CreationAwaitStatus::Ok {
                    stream,
                    response,
                    request_rx,
                    response_tx,
                    notifier,
                } => Ok((stream, response, request_rx, response_tx, notifier)),
                CreationAwaitStatus::Err(e) => Err(e),
            },
            CreationStatus::Ok { .. } | CreationStatus::Err(_) => return StatusChanged::False,
        };
        event!(
            Level::TRACE,
            "await_result.is_ok() = {}",
            await_result.is_ok()
        );
        match await_result {
            Ok((stream, _response, request_rx, response_tx, notifier)) => self.launch(
                stream,
                request_rx,
                response_tx,
                self.communication_status.clone(),
                notifier,
            ),
            Err(e) => self.creation_status = CreationStatus::Err(e),
        }
        StatusChanged::True
    }

    fn launch(
        &mut self,
        mut stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        mut request_rx: tokio::sync::mpsc::Receiver<Request>,
        response_tx: tokio::sync::mpsc::Sender<Response>,
        communication_status: Arc<AtomicCommunicationStatus>,
        notifier: Notifier,
    ) {
        println!("launch called");
        debug_assert!(self.launch_join_handle.is_none());
        self.launch_join_handle = Some(self.tokio_runtime.spawn(async move {
            communication_status.store(CommunicationStatus::AwaitingRequest, ORDERING);
            while let Some(request) = request_rx.recv().await {
                communication_status.store(CommunicationStatus::DeserializingRequest, ORDERING);
                match serde_json::to_string(&request) {
                    Ok(request) => {
                        if let Err(_) = stream.send(Message::Text(request)).await {
                            todo!()
                        }
                    }
                    Err(_) => todo!(),
                }
                if request.need_response() {
                    communication_status.store(CommunicationStatus::AwaitingResponse, ORDERING);
                    match stream.next().await {
                        Some(response) => match response {
                            Ok(response) => match response {
                                Message::Text(response) => match serde_json::from_str(&response) {
                                    Ok(response) => {
                                        if let Err(_) = response_tx.send(response).await {
                                            todo!()
                                        }
                                    }
                                    Err(e) => {
                                        p!(e, response);
                                        println!("response = {response}");
                                        todo!()
                                    }
                                },
                                Message::Binary(_) => todo!(),
                                Message::Ping(_) => todo!(),
                                Message::Pong(_) => todo!(),
                                Message::Close(_) => todo!(),
                                Message::Frame(_) => todo!(),
                            },
                            Err(e) => todo!("e = {e}"),
                        },
                        None => todo!(),
                    }
                    communication_status.store(CommunicationStatus::ResponseReady, ORDERING);
                    notifier.notify_change()
                } else {
                    communication_status.store(CommunicationStatus::AwaitingRequest, ORDERING);
                }
            }
        }));
        match self.request_tx.blocking_send(Request::default()) {
            Ok(_) => (),
            Err(e) => todo!("e = {e}"),
        };
        self.creation_status = CreationStatus::Ok
    }
}

pub enum StatusChanged {
    True,
    False,
}
