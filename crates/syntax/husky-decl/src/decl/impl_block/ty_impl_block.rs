use super::*;
use husky_token::EolColonToken;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeImplBlockDecl {
    pub ast_idx: AstIdx,
    pub impl_block: ImplBlock,
    pub impl_token: ImplToken,
    #[return_ref]
    pub implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub ty: TypeExpr,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolColonToken>,
    pub expr_region: ExprRegion,
}

impl TypeImplBlockDecl {
    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        Ok(self
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|l| -> &[ImplicitParameterDecl] { &l })
            .unwrap_or(&[]))
    }
}
