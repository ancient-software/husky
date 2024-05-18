use super::*;
use husky_syn_decl::decl::{HasSynDecl, TypeAssocRitchieSynDecl, TypeItemSynDecl};

#[salsa::interned]
pub struct TypeAssocRitchieHirDecl {
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub parenate_parameters: HirEagerParenateParameters,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TypeAssocRitchieHirDecl {
    pub(super) fn from_syn(
        path: TypeItemPath,
        _syn_decl: TypeAssocRitchieSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let TypeItemSynDecl::AssocRitchie(syn_decl) = path.syn_decl(db).expect("ok") else {
            unreachable!()
        };
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let parenate_parameters =
            HirEagerParenateParameters::from_syn(syn_decl.parenate_parameters(db), &builder);
        let return_ty = builder.return_ty_before_colon(syn_decl.return_ty(db));
        Self::new(
            db,
            path,
            template_parameters,
            parenate_parameters,
            return_ty,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}
