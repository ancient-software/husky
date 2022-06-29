use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub struct HuskyTracerGuiMessage {
    pub opt_request_id: Option<usize>,
    pub variant: HuskyTracerGuiMessageVariant,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum HuskyTracerGuiMessageVariant {
    InitDataRequest,
    Activate {
        trace_id: TraceId,
        need_figure_canvas_data: bool,
        need_figure_control_data: bool,
    },
    ToggleExpansion {
        trace_id: TraceId,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    Trace {
        id: TraceId,
    },
    LockAttention {
        attention: Attention,
        need_figure_canvas_data: bool,
        need_figure_control_data: bool,
        need_stalk: bool,
    },
    TraceStalk {
        trace_id: TraceId,
    },
    UpdateFigureControlData {
        trace_id: TraceId,
        attention: Attention,
        figure_control_props: FigureControlData,
    },
}
