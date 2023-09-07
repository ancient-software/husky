use super::*;
use husky_syn_expr::SynExprIdx;
use parsec::{PunctuatedSmallList, TryParseFromStream};

// todo: GADT
#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TupleTypeVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub ast_idx: AstIdx,
    lpar_token_idx: TokenIdx,
    #[return_ref]
    field_comma_list:
        SynNodeDeclResult<PunctuatedSmallList<TupleFieldObelisk, CommaToken, 4, SynNodeDeclError>>,
    #[return_ref]
    rpar: SynNodeDeclResult<TupleTypeVariantRparToken>,
    pub syn_expr_region: SynExprRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleTypeVariantRparToken(RparToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for TupleTypeVariantRparToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_parenthesis
        let rpar = sp.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList,
        )?;
        Ok(Self(rpar))
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TupleTypeVariantSynDecl {
    #[id]
    pub path: TypeVariantPath,
    pub fields: SmallVec<[TupleFieldObelisk; 4]>,
    pub syn_expr_region: SynExprRegion,
}

impl TupleTypeVariantSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypeVariantPath,
        syn_node_decl: TupleTypeVariantSynNodeDecl,
    ) -> DeclResult<Self> {
        let fields = SmallVec::from(syn_node_decl.field_comma_list(db).as_ref()?.elements());
        Ok(Self::new(
            db,
            path,
            fields,
            syn_node_decl.syn_expr_region(db),
        ))
    }
}
