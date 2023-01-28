use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RegularStructFieldPattern {
    ident_token: IdentifierToken,
    colon: ColonToken,
    ty: ExprIdx,
}

impl RegularStructFieldPattern {
    pub fn ident(&self) -> Identifier {
        self.ident_token.ident()
    }

    pub fn colon(&self) -> ColonToken {
        self.colon
    }

    pub fn ty(&self) -> ExprIdx {
        self.ty
    }
}

impl<'a, 'b> parsec::ParseFrom<ExprParseContext<'a, 'b>> for RegularStructFieldPattern {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(ident_token) = ctx.parse::<IdentifierToken>()? else {
                return Ok(None)
            };
        let colon: ColonToken = ctx.parse_expected()?;
        let ty = ctx.parse_expr_expected2(ExprParseEnvironment::None, ExprError::MissingFieldType);
        let variables = ctx.add_expr_root(ExprRoot::new(ExprRootKind::FieldType, ty));
        Ok(Some(RegularStructFieldPattern {
            ident_token,
            colon,
            ty,
        }))
    }
}
