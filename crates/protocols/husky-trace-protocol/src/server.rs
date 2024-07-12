use self::accompany::AccompanyingTraceIdsExceptFollowed;
use crate::{
    message::{TraceRequest, TraceResponse},
    synchrotron::action::{
        TraceSynchrotronNewTrace, TraceSynchrotronSetSubtraces, TraceSynchrotronToggleExpansion,
    },
    view::{action::TraceViewAction, TraceViewData},
    *,
};
use husky_devsoul_interface::{ki_repr::KiReprInterface, pedestal::IsPedestalFull};
use husky_value_protocol::presentation::{
    synchrotron::ValuePresentationSynchrotron, ValuePresenterCache,
};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use husky_websocket_utils::easy_server::IsEasyWebsocketServer;
use rustc_hash::FxHashMap;
use std::net::ToSocketAddrs;

pub struct TraceServer<Tracetime: IsTracetime> {
    trace_synchrotron: Option<TraceSynchrotron<Tracetime::TraceProtocol>>,
    value_presenter_cache: ValuePresenterCache,
    visual_cache: ValVisualCache<<Tracetime::TraceProtocol as IsTraceProtocol>::Pedestal>,
    tracetime: Tracetime,
}

/// this struct storages mapping from trace id and pedestal to visual,
///
/// useful for calculating figure,
///
/// but the client doesn't need to know about this
pub struct ValVisualCache<Pedestal: IsPedestalFull> {
    visuals: FxHashMap<(KiReprInterface, Pedestal), Visual>,
}

impl<Pedestal: IsPedestalFull> ValVisualCache<Pedestal> {
    pub fn get_visual(
        &mut self,
        ki_repr: KiReprInterface,
        pedestal: Pedestal,
        f: impl FnOnce() -> Visual,
    ) -> Visual {
        *self.visuals.entry((ki_repr, pedestal)).or_insert_with(f)
    }
}

impl<Pedestal: IsPedestalFull> Default for ValVisualCache<Pedestal> {
    fn default() -> Self {
        Self {
            visuals: Default::default(),
        }
    }
}

impl<Tracetime: IsTracetime> Default for TraceServer<Tracetime>
where
    Tracetime: Default,
{
    fn default() -> Self {
        Self {
            trace_synchrotron: Default::default(),
            value_presenter_cache: Default::default(),
            tracetime: Default::default(),
            visual_cache: Default::default(),
        }
    }
}

impl<Tracetime: IsTracetime> TraceServer<Tracetime> {
    fn new(tracetime: Tracetime) -> Self {
        Self {
            trace_synchrotron: Default::default(),
            tracetime,
            value_presenter_cache: Default::default(),
            visual_cache: Default::default(),
        }
    }

    fn init(&mut self) {
        if self.trace_synchrotron.is_some() {
            return;
        }
        let trace_bundles = self.tracetime.get_trace_bundles();
        self.trace_synchrotron = Some(TraceSynchrotron::new(trace_bundles, |trace| {
            self.tracetime.get_trace_view_data(trace).clone()
        }));
        self.cache_periphery()
    }

    #[track_caller]
    fn trace_synchrotron(&self) -> &TraceSynchrotron<Tracetime::TraceProtocol> {
        self.trace_synchrotron.as_ref().unwrap()
    }

    #[track_caller]
    fn trace_synchrotron_mut(&mut self) -> &mut TraceSynchrotron<Tracetime::TraceProtocol> {
        self.trace_synchrotron.as_mut().unwrap()
    }
}

impl<Tracetime> IsEasyWebsocketServer for TraceServer<Tracetime>
where
    Tracetime: IsTracetime,
    Tracetime::TraceProtocol: Serialize + for<'a> Deserialize<'a>,
{
    type Response = TraceResponse<Tracetime::TraceProtocol>;

    type Request = TraceRequest<Tracetime::TraceProtocol>;

    type SerdeImpl = Tracetime::SerdeImpl;

    fn handle(&mut self, request: Self::Request) -> Option<Self::Response> {
        match request {
            TraceRequest::Init {
                trace_protocol_type_name,
            } => {
                if trace_protocol_type_name != std::any::type_name::<Tracetime::TraceProtocol>() {
                    // todo: make this a type
                    return Some(TraceResponse::Err(format!(
                        r#"server's trace protocol is of type `{},
but client's trace protocol is of type `{trace_protocol_type_name}`."#,
                        std::any::type_name::<Tracetime::TraceProtocol>()
                    )));
                }
                self.init();
                let Some(trace_synchrotron) = self.trace_synchrotron.clone() else {
                    unreachable!()
                };
                Some(TraceResponse::Init { trace_synchrotron })
            }
            TraceRequest::TakeViewAction {
                view_action,
                trace_synchrotron_status,
            } => {
                let Some(ref mut _cache) = self.trace_synchrotron else {
                    unreachable!()
                };
                assert_eq!(self.trace_synchrotron().status(), trace_synchrotron_status);
                self.take_view_action(view_action);
                let trace_synchrotron_actions_diff = self
                    .trace_synchrotron()
                    .actions_diff(trace_synchrotron_status);
                Some(TraceResponse::TakeTraceSynchrotronActionsDiff {
                    trace_synchrotron_actions_diff,
                })
            }
            TraceRequest::NotifyViewAction {
                trace_synchrotron_action,
                ..
            } => {
                self.trace_synchrotron_mut()
                    .take_action(trace_synchrotron_action);
                None
            }
        }
    }
}

impl<Tracetime: IsTracetime> TraceServer<Tracetime> {
    fn take_view_action(&mut self, view_action: TraceViewAction<Tracetime::TraceProtocol>) {
        match view_action {
            TraceViewAction::ToggleExpansion { trace_id } => {
                assert!(!self.trace_synchrotron()[trace_id].expanded());
                // todo: handle more cases like subtraces with channels
                if self.trace_synchrotron()[trace_id].subtrace_ids().is_some() {
                    return;
                }
                let subtraces = self.tracetime.get_subtraces(trace_id.into()).to_vec();
                let subtrace_ids = subtraces
                    .into_iter()
                    .map(|subtrace| {
                        let subtrace_id = subtrace.into();
                        self.cache_trace_if_new(subtrace_id);
                        subtrace_id
                    })
                    .collect();
                self.trace_synchrotron_mut()
                    .take_action(TraceSynchrotronSetSubtraces::new(trace_id, subtrace_ids));
                self.trace_synchrotron_mut()
                    .take_action(TraceSynchrotronToggleExpansion::new(trace_id))
            }
            TraceViewAction::Marker { _marker } => todo!(),
            TraceViewAction::ToggleAssocTrace {
                trace_id,
                assoc_trace_id,
            } => {
                self.cache_trace_if_new(assoc_trace_id);
                self.trace_synchrotron_mut()
                    .take_action(TraceSynchrotronAction::ToggleAssocTrace {
                        trace_id,
                        assoc_trace_id,
                    })
            }
            TraceViewAction::FollowTrace { trace_id } => self
                .trace_synchrotron_mut()
                .take_action(TraceSynchrotronAction::FollowTrace { trace_id }),
            TraceViewAction::ToggleAccompany { trace_id } => self
                .trace_synchrotron_mut()
                .take_action(TraceSynchrotronAction::ToggleAccompany { trace_id }),
            TraceViewAction::SetPedestal { pedestal } => self
                .trace_synchrotron_mut()
                .take_action(TraceSynchrotronAction::SetPedestal { pedestal }),
        }
        self.cache_periphery()
    }

    fn cache_trace_if_new(&mut self, trace_id: TraceId) {
        if !self.trace_synchrotron().is_trace_cached(trace_id) {
            let view_data = self.tracetime.get_trace_view_data(trace_id.into());
            self.trace_synchrotron_mut()
                .take_action(TraceSynchrotronNewTrace::new(trace_id, view_data))
        }
    }

    fn cache_periphery(&mut self) {
        self.cache_stalks();
        self.cache_figure()
    }

    fn cache_stalks(&mut self) {
        let trace_synchrotron = &self.trace_synchrotron();
        let trace_listing = trace_synchrotron.trace_listing();
        let pedestal = trace_synchrotron.pedestal();
        for trace_id in trace_listing {
            self.cache_stalk(trace_id, pedestal)
        }
    }

    fn cache_stalk(
        &mut self,
        trace_id: TraceId,
        pedestal: <<Tracetime as IsTracetime>::TraceProtocol as IsTraceProtocol>::Pedestal,
    ) {
        if !self.trace_synchrotron()[trace_id].has_stalk(pedestal) {
            let trace_synchrotron = self.trace_synchrotron.as_mut().unwrap();
            let stalk = self.tracetime.get_trace_stalk(
                trace_id.into(),
                pedestal,
                &mut self.value_presenter_cache,
                trace_synchrotron.value_presentation_synchrotron_mut(),
            );
            trace_synchrotron.take_action(TraceSynchrotronAction::CacheStalk {
                pedestal,
                trace_id,
                stalk,
            });
        }
    }

    fn cache_figure(&mut self) {
        let trace_synchrotron = self.trace_synchrotron.as_mut().unwrap();
        let pedestal = trace_synchrotron.pedestal();
        let accompanying_trace_ids_except_followed = trace_synchrotron
            .accompanying_trace_ids_except_followed()
            .clone();
        let followed_trace_id = trace_synchrotron.followed_trace_id();
        let (has_figure, accompanying_trace_ids_except_followed) = trace_synchrotron.has_figure(
            followed_trace_id,
            pedestal,
            accompanying_trace_ids_except_followed,
        );
        if !has_figure {
            let figure = self.tracetime.get_figure(
                followed_trace_id.map(Into::into),
                &accompanying_trace_ids_except_followed,
                pedestal,
                trace_synchrotron.visual_synchrotron_mut(),
                &mut self.visual_cache,
            );
            trace_synchrotron.take_action(TraceSynchrotronAction::CacheFigure {
                pedestal,
                followed_trace_id,
                accompanying_trace_ids_except_followed,
                figure,
            })
        }
    }
}

pub trait IsTracetime: Send + 'static + Sized {
    type Trace: IsTrace;
    //  Send + Eq + std::hash::Hash + Copy;

    type TraceProtocol: IsTraceProtocolFull;

    type SerdeImpl: serde_impl::IsSerdeImpl;

    /// final
    fn serve_traces(self, addr: impl ToSocketAddrs) {
        TraceServer::new(self).easy_serve(addr)
    }

    fn get_trace_bundles(&self) -> &[TraceBundle<Self::Trace>];

    fn get_subtraces(&self, trace: Self::Trace) -> &[Self::Trace];

    fn get_trace_view_data(&self, trace: Self::Trace) -> TraceViewData;

    fn get_trace_stalk(
        &self,
        trace: Self::Trace,
        pedestal: <Self::TraceProtocol as IsTraceProtocol>::Pedestal,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> TraceStalk;

    fn get_figure(
        &self,
        followed_trace: Option<Self::Trace>,
        accompanying_trace_ids: &AccompanyingTraceIdsExceptFollowed,
        pedestal: <Self::TraceProtocol as IsTraceProtocol>::Pedestal,
        visual_synchrotron: &mut VisualSynchrotron,
        val_visual_cache: &mut ValVisualCache<<Self::TraceProtocol as IsTraceProtocol>::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure;
}
