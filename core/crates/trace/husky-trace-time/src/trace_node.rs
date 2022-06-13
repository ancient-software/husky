use super::*;

#[derive(Debug)]
pub struct TraceNode {
    pub(crate) trace: Trace,
    pub(crate) expansion: bool,
    pub(crate) shown: bool,
}

impl TraceNode {
    pub fn to_data(&self) -> husky_tracer_protocol::TraceNodeData {
        TraceNodeData {
            trace: self.trace.props.clone(),
            expansion: self.expansion,
            shown: self.shown,
        }
    }
}
