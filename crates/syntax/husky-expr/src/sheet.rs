use crate::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ExprSheet {
    expr_arena: ExprArena,
    expr_base_entity_paths: Vec<BaseEntityPath>,
    entity_path_expr_arena: EntityPathExprArena,
    pattern_expr_arena: PatternExprArena,
}

impl ExprSheet {
    pub fn expr_arena(&self) -> &ExprArena {
        &self.expr_arena
    }

    pub fn entity_path_expr_arena(&self) -> &EntityPathExprArena {
        &self.entity_path_expr_arena
    }

    pub fn pattern_expr_arena(&self) -> &PatternExprArena {
        &self.pattern_expr_arena
    }

    pub(crate) fn alloc_expr_batch(
        &mut self,
        opds: Vec<Expr>,
        base_entity_path_results: Vec<BaseEntityPath>,
    ) -> ArenaIdxRange<Expr> {
        self.expr_base_entity_paths.extend(base_entity_path_results);
        self.expr_arena.alloc_batch(opds)
    }
}

impl std::ops::Index<ExprIdx> for ExprSheet {
    type Output = Expr;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_arena[index]
    }
}

impl std::ops::Index<PatternExprIdx> for ExprSheet {
    type Output = PatternExpr;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_arena[index]
    }
}

impl std::ops::Index<EntityPathExprIdx> for ExprSheet {
    type Output = EntityPathExpr;

    fn index(&self, index: EntityPathExprIdx) -> &Self::Output {
        &self.entity_path_expr_arena[index]
    }
}
