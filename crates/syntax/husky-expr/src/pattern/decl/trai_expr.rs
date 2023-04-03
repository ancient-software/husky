use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitExpr {
    expr: ExprIdx,
}

impl TraitExpr {
    pub fn expr(&self) -> ArenaIdx<Expr> {
        self.expr
    }
}

impl<'a, 'b> ParseFromStream<ExprParseContext<'a, 'b>> for TraitExpr {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr(None) {
            ctx.add_expr_root(ExprRoot::new(ExprRootKind::Trait, expr));
            Ok(Some(TraitExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
