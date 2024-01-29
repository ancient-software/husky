use parsec::parse_consecutive_list;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TupleFieldSyndicate {
    decorators: Vec<TupleFieldAttr>,
    visibility: Option<FieldVisibilityExpr>,
    ty: SynExprIdx,
}

impl TupleFieldSyndicate {
    pub fn ty(&self) -> SynExprIdx {
        self.ty
    }
}

impl<'a> parsec::TryParseOptionFromStream<SynDeclExprParser<'a>> for TupleFieldSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        let decorators = parse_consecutive_list(ctx)?;
        let visibility = ctx.try_parse_option()?;
        let ty = ctx.parse_expr_expected2(
            None,
            SynExprRootKind::TupleStructFieldType,
            OriginalSynExprError::ExpectedFieldType,
        );
        Ok(Some(TupleFieldSyndicate {
            decorators,
            visibility,
            ty,
        }))
    }
}

// todo: merge this with PropsFieldAttr?
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TupleFieldAttr {}

impl<'a, 'b> parsec::TryParseOptionFromStream<SynDeclExprParser<'a>> for TupleFieldAttr {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(_pound_token) = ctx.try_parse_option::<PoundRegionalToken>()? else {
            return Ok(None);
        };
        todo!()
    }
}

// todo: repetitive
// merge with struct field?
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldVisibilityExpr {
    Pub,
}

impl<'a, 'b> parsec::TryParseOptionFromStream<SynDeclExprParser<'a>> for FieldVisibilityExpr {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(_pub_token) = ctx.try_parse_option::<PubRegionalToken>()? else {
            return Ok(None);
        };
        let Some(_lpar_token) = ctx.try_parse_option::<LparRegionalToken>()? else {
            return Ok(Some(FieldVisibilityExpr::Pub));
        };
        todo!()
    }
}
