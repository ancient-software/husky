use super::*;
use husky_expr::ExprIdx;
use husky_word::Identifier;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
}

impl UnitStructTypeDecl {
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
