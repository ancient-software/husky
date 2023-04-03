use parsec::parse_consecutive_list;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct FieldDeclPattern {
    decorators: Vec<FieldDecorator>,
    visibility: Option<FieldVisibilityExpr>,
    ident_token: IdentToken,
    colon: ColonToken,
    ty: ExprIdx,
}

impl FieldDeclPattern {
    pub fn ident(&self) -> Ident {
        self.ident_token.ident()
    }

    pub fn colon(&self) -> ColonToken {
        self.colon
    }

    pub fn ty(&self) -> ExprIdx {
        self.ty
    }
}

impl<'a, 'b> parsec::ParseFromStreamWithError<ExprParseContext<'a, 'b>> for FieldDeclPattern {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        let decorators = parse_consecutive_list(ctx)?;
        let visibility = ctx.parse()?;
        let Some(ident_token) = ctx.parse::<IdentToken>()? else {
                return Ok(None)
            };
        let colon: ColonToken = ctx.parse_expected(OriginalExprError::ExpectColon)?;
        let ty = ctx.parse_expr_expected2(None, OriginalExprError::ExpectedFieldType);
        let variables = ctx.add_expr_root(ExprRoot::new(ExprRootKind::FieldType, ty));
        Ok(Some(FieldDeclPattern {
            decorators,
            visibility,
            ident_token,
            colon,
            ty,
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct FieldDecorator {}

impl<'a, 'b> parsec::ParseFromStreamWithError<ExprParseContext<'a, 'b>> for FieldDecorator {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(at_token) = ctx.parse::<AtToken>()? else {
            return Ok(None)
        };
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum FieldVisibilityExpr {
    Pub,
}

impl<'a, 'b> parsec::ParseFromStreamWithError<ExprParseContext<'a, 'b>> for FieldVisibilityExpr {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(pub_token) = ctx.parse::<PubToken>()? else {
            return Ok(None)
        };
        let Some(lpar_token) = ctx.parse::<LeftParenthesisToken>()? else {
            return Ok(Some(FieldVisibilityExpr::Pub))
        };
        todo!()
    }
}
