use super::*;

#[derive(Debug, Clone)]
pub struct SymbolContext<'a> {
    module_prelude: ModulePrelude<'a>,
    expr_arena: &'a ExprArena,
    entity_path_expr_arena: &'a EntityPathExprArena,
    pattern_expr_sheet: &'a PatternExprSheet,
    stmt_arena: &'a StmtArena,
    symbol_sheet: &'a SymbolSheet,
}

impl<'a> SymbolContext<'a> {
    pub fn new(
        db: &'a dyn ExprDb,
        module_prelude: ModulePrelude<'a>,
        expr_sheet: ExprSheet,
    ) -> Self {
        Self {
            module_prelude,
            expr_arena: expr_sheet.expr_arena(db),
            entity_path_expr_arena: expr_sheet.entity_path_expr_arena(db),
            pattern_expr_sheet: expr_sheet.pattern_expr_sheet(db),
            stmt_arena: expr_sheet.stmt_arena(db),
            symbol_sheet: expr_sheet.symbol_sheet(db),
        }
    }

    pub fn exprs(&self) -> &'a [Expr] {
        self.expr_arena.data()
    }

    pub fn indexed_local_symbol_iter(
        &self,
    ) -> impl Iterator<Item = (LocalSymbolIdx, &'a LocalSymbol)> + 'a {
        self.symbol_sheet.indexed_local_symbol_iter()
    }
}

impl<'a> std::ops::Index<PatternSymbolIdx> for SymbolContext<'a> {
    type Output = PatternSymbol;

    fn index(&self, index: PatternSymbolIdx) -> &Self::Output {
        &self.pattern_expr_sheet[index]
    }
}

impl<'a> std::ops::Index<PatternExprIdx> for SymbolContext<'a> {
    type Output = PatternExpr;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_sheet[index]
    }
}
