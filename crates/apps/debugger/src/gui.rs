mod handle;
mod init;
mod query;
mod response;
mod tests;

use crate::{
    gui::{handle::handle_message, init::init_gui},
    *,
};

use futures::{task::SpawnExt, FutureExt, StreamExt};
use init::InitData;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedSender};
use trace::{FigureProps, Trace, TraceId, TraceStalk};
use warp::ws::{Message, WebSocket};

use query::Query;
use response::Response;

pub(crate) async fn handle_query(
    socket: warp::ws::Ws,
    server: Arc<Debugger>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(socket.on_upgrade(move |ws| handle_query_upgraded(ws, server)))
}

pub(crate) async fn handle_query_upgraded(websocket: WebSocket, debugger: Arc<Debugger>) {
    let (tx, mut rx) = websocket.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();
    tokio::task::spawn(client_rcv.forward(tx).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));
    println!(
        "{}husky:{} query connection established.",
        print_utils::CYAN,
        print_utils::RESET
    );
    init_gui(&debugger, client_sender.clone());
    while let Some(result) = rx.next().await {
        let msg = result.expect("error receiving ws message: {}");
        match msg.to_str() {
            Ok(text) => handle_message(debugger.clone(), text, client_sender.clone()),
            Err(_) => {
                if msg.is_close() {
                    println!(
                        "{}husky:{} query connection closed.",
                        print_utils::CYAN,
                        print_utils::RESET
                    );
                } else {
                    eprintln!("nontext message received: {:?}", msg);
                }
            }
        };
    }
}
