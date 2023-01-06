use crate::*;

#[salsa::tracked(jar = ExprJar)]
pub struct ExprSheet {
    #[return_ref]
    pub expr_arena: ExprArena,
    #[return_ref]
    pub entity_path_expr_arena: EntityPathExprArena,
    #[return_ref]
    pub pattern_expr_arena: PatternExprSheet,
    #[return_ref]
    pub stmt_arena: StmtArena,
}
