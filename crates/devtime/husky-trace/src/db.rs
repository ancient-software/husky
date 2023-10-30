use husky_entity_syn_tree::EntitySynTreeDb;
use husky_hir_expr::db::HirExprDb;
use husky_text::db::TextDb;
use husky_token_info::TokenInfoDb;

use crate::*;

pub trait TraceDb: salsa::DbWithJar<TraceJar> + TokenInfoDb + TextDb + HirExprDb {
    fn root_traces(&self, crate_path: CratePath) -> &[Trace];
}

impl<Db> TraceDb for Db
where
    Db: salsa::DbWithJar<TraceJar> + TokenInfoDb + TextDb + HirExprDb,
{
    fn root_traces(&self, crate_path: CratePath) -> &[Trace] {
        crate::helpers::root_traces(self, crate_path).as_ref()
    }
}

#[salsa::jar(db = TraceDb)]
pub struct TraceJar(
    submodule_view_tokens,
    submodule_contains_val_item,
    submodule_trace_subtraces,
    ValItemTracePath,
    ValItemTrace,
    val_item_trace_view_tokens,
    val_item_trace_subtraces,
    LazyCallTracePath,
    LazyCallTrace,
    LazyExprTracePath,
    LazyExprTrace,
    LazyStmtTracePath,
    LazyStmtTrace,
    lazy_stmt_trace_view_tokens,
    // lazy_stmt_associated_expr_traces,
    EagerCallTracePath,
    EagerCallTrace,
    EagerExprTracePath,
    EagerExprTrace,
    EagerStmtTracePath,
    EagerStmtTrace,
    eager_stmt_associated_expr_traces,
    eager_stmt_trace_view_tokens,
    LoopGroupTracePath,
    LoopGroupTrace,
    // helpers
    crate::helpers::root_traces,
);
