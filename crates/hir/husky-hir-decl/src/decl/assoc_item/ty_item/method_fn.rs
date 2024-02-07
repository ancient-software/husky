use super::*;

use husky_syn_decl::TypeMethodFnSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMethodFnHirDecl {
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub self_value_parameter: HirEagerSelfValueParameter,
    #[return_ref]
    pub parenate_parameters: HirEagerParenateParameters,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TypeMethodFnHirDecl {
    pub(super) fn from_syn(
        path: TypeItemPath,
        syn_decl: TypeMethodFnSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let self_ty =
            HirType::from_eth(path.eth_template(db).unwrap().self_ty(db).unwrap(), db).unwrap();
        let self_value_parameter =
            HirEagerSelfValueParameter::from_syn(self_ty, syn_decl.self_value_parameter(db));
        let parenate_parameters =
            HirEagerParenateParameters::from_syn(syn_decl.parenate_parameters(db), &builder);
        let return_ty = builder.return_ty_before_colon(syn_decl.return_ty(db));
        TypeMethodFnHirDecl::new(
            db,
            path,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
            builder.finish().eager(),
        )
    }
}
