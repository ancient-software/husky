use super::*;
use husky_syn_decl::decl::major_item::ty::r#extern::ExternSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar, constructor = new_inner)]
pub struct ExternTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl ExternTypeHirDecl {
    pub(super) fn from_syn(path: TypePath, _syn_decl: ExternSynDecl, db: &::salsa::Db) -> Self {
        let TypeSynDecl::Extern(syn_decl) = path.syn_decl(db).expect("hir stage ok") else {
            unreachable!()
        };
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        Self::new_inner(
            db,
            path,
            template_parameters,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}
