use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FigureControlKey {
    LoopFrame { parent: TraceId },
    Other { this: TraceId },
}

impl FigureControlKey {
    pub fn from_trace_raw_data(trace_raw_data: &TraceRawData, focus: &Focus) -> FigureControlKey {
        Self::new(
            trace_raw_data.opt_parent_id,
            trace_raw_data.kind,
            trace_raw_data.id,
            focus,
        )
    }

    pub fn new(
        opt_parent_id: Option<TraceId>,
        trace_kind: TraceKind,
        trace_id: TraceId,
        focus: &Focus,
    ) -> FigureControlKey {
        match trace_kind {
            TraceKind::LoopFrame => FigureControlKey::LoopFrame {
                parent: opt_parent_id.unwrap(),
            },
            _ => FigureControlKey::Other { this: trace_id },
        }
    }
}
