use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LazyExprTracePath {
    #[return_ref]
    pub data: EagerExprTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyExprTracePathData {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LazyExprTrace {
    #[id]
    pub path: LazyExprTracePath,
    pub biological_parent: LazyExprTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyExprTraceBiologicalParent {
    LazyStmt(LazyStmtTrace),
}
