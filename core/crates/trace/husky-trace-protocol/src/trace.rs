mod id;
mod node;
mod stalk;
mod token;

pub use id::*;
pub use node::*;
pub use stalk::*;
pub use token::*;

use super::*;

pub type Indent = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceData {
    pub opt_parent_id: Option<TraceId>,
    pub id: TraceId,
    pub kind: TraceKind,
    pub indent: Indent,
    pub lines: Vec<TraceLineData>,
    pub compile_time_version: usize,
    pub can_have_subtraces: bool,
    pub reachable: bool,
}

impl TraceData {
    pub fn associated_trace_ids(&self) -> Vec<TraceId> {
        let mut associated_trace_ids = vec![];
        for line in &self.lines {
            for token in &line.tokens {
                if token.value == "]" || token.value == ")" || token.value == "}" {
                    continue;
                }
                if let Some(associated_trace_id) = token.opt_associated_trace_id {
                    associated_trace_ids.push(associated_trace_id)
                }
            }
        }
        associated_trace_ids
    }

    pub fn has_subtraces(&self, has_sample_id: bool) -> bool {
        match self.kind {
            TraceKind::Main | TraceKind::FeatureBranch | TraceKind::LoopFrame => true,
            TraceKind::CallHead
            | TraceKind::FeatureCallArgument
            | TraceKind::EagerCallArgument
            | TraceKind::FeatureStmt => false,
            TraceKind::FuncStmt
            | TraceKind::EagerExpr
            | TraceKind::ProcStmt
            | TraceKind::FuncBranch
            | TraceKind::ProcBranch => self.can_have_subtraces,
            TraceKind::FeatureExpr => has_sample_id && self.can_have_subtraces,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TraceKind {
    Main,
    FeatureStmt,
    FeatureBranch,
    FeatureExpr,
    FeatureCallArgument,
    FuncStmt,
    ProcStmt,
    ProcBranch,
    FuncBranch,
    LoopFrame,
    EagerExpr,
    EagerCallArgument,
    CallHead,
}

impl TraceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TraceKind::Main => "Main",
            TraceKind::FeatureStmt => "FeatureStmt",
            TraceKind::FeatureBranch => "FeatureBranch",
            TraceKind::FeatureExpr => "FeatureExpr",
            TraceKind::FeatureCallArgument => "FeatureCallArgument",
            TraceKind::FuncStmt => "FuncStmt",
            TraceKind::ProcStmt => "ProcStmt",
            TraceKind::ProcBranch => "ProcBranch",
            TraceKind::FuncBranch => "FuncBranch",
            TraceKind::LoopFrame => "LoopFrame",
            TraceKind::EagerExpr => "EagerExpr",
            TraceKind::EagerCallArgument => "EagerCallArgument",
            TraceKind::CallHead => "CallHead",
        }
    }
    pub fn can_have_stalk(self) -> bool {
        match self {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr => true,
            TraceKind::FeatureCallArgument
            | TraceKind::EagerCallArgument
            | TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::FuncBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr
            | TraceKind::CallHead => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceLineData {
    pub indent: Indent,
    pub tokens: Vec<TraceTokenData>,
    pub idx: usize,
}
