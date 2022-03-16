use semantics_feature::*;
use vm::{History, InstructionSheet, LoopFrameSnapshot, StackValueSnapshot, VMControl};

use crate::*;

#[derive(Debug, Clone)]
pub enum TraceKind {
    Main(Arc<FeatureBlock>),
    FeatureStmt(Arc<FeatureStmt>),
    FeatureBranch(Arc<FeatureBranch>),
    FeatureExpr(Arc<FeatureExpr>),
    Input(Arc<FeatureExpr>),
    StrictDeclStmt {
        stmt: Arc<DeclStmt>,
        history: Arc<History>,
    },
    ImprStmt {
        stmt: Arc<ImprStmt>,
        history: Arc<History>,
    },
    LoopFrame {
        loop_stmt: Arc<ImprStmt>,
        body_instruction_sheet: Arc<InstructionSheet>,
        body_stmts: Arc<Vec<Arc<ImprStmt>>>,
        loop_frame_snapshot: LoopFrameSnapshot,
    },
    EagerExpr {
        expr: Arc<EagerExpr>,
        history: Arc<History>,
    },
    CallHead {
        entity: Arc<Entity>,
        tokens: Vec<TokenProps>,
    },
}

impl TraceKind {
    pub fn file_and_range(&self) -> (FilePtr, TextRange) {
        match self {
            TraceKind::Main(ref block) => (block.file, block.range),
            TraceKind::FeatureStmt(ref stmt) => (stmt.file, stmt.range),
            TraceKind::FeatureExpr(ref expr) => (expr.file, expr.range),
            TraceKind::FeatureBranch(ref branch) => (branch.block.file, branch.block.range),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt { ref stmt, .. } => (stmt.file, stmt.range),
            TraceKind::EagerExpr { ref expr, .. } => (expr.file, expr.range),
            TraceKind::CallHead { ref entity, .. } => (entity.file, entity.range),
            TraceKind::ImprStmt { stmt, .. } => (stmt.file, stmt.range),
            TraceKind::LoopFrame { loop_stmt, .. } => (loop_stmt.file, loop_stmt.range),
        }
    }
}

impl Serialize for TraceKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            TraceKind::Main(_) => "Main",
            TraceKind::FeatureStmt(_) => "FeatureStmt",
            TraceKind::FeatureBranch(_) => "FeatureBranch",
            TraceKind::FeatureExpr(_) => "FeatureExpr",
            TraceKind::Input(_) => "Input",
            TraceKind::StrictDeclStmt { .. } => "StrictDeclStmt",
            TraceKind::ImprStmt { .. } => "ImprStmt",
            TraceKind::EagerExpr { .. } => "StrictExpr",
            TraceKind::CallHead { .. } => "CallHead",
            TraceKind::LoopFrame { .. } => "LoopFrame",
        })
    }
}
