use super::*;
use husky_hir_decl::{
    decl::FunctionGnFormHirDecl, parameter::parenate::lazy::HirLazyParenateParameter,
};
use husky_hir_lazy_expr::helpers::hir_lazy_body_with_expr_region;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct FunctionGnHirDefn {
    pub path: MajorFormPath,
    pub hir_decl: FunctionGnFormHirDecl,
    pub lazy_body_with_hir_lazy_expr_region: Option<(HirLazyExprIdx, HirLazyExprRegion)>,
}

impl From<FunctionGnHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: FunctionGnHirDefn) -> Self {
        MajorItemHirDefn::Form(hir_defn.into())
    }
}

impl From<FunctionGnHirDefn> for HirDefn {
    fn from(hir_defn: FunctionGnHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl FunctionGnHirDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: MajorFormPath,
        hir_decl: FunctionGnFormHirDecl,
    ) -> Self {
        FunctionGnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_lazy_body_with_expr_region(path.into(), db),
        )
    }

    pub fn hir_lazy_expr_region(self, db: &::salsa::Db) -> Option<HirLazyExprRegion> {
        Some(self.lazy_body_with_hir_lazy_expr_region(db)?.1)
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        function_gn_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        function_gn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn function_gn_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: FunctionGnHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_lazy_expr_region(hir_decl.hir_lazy_expr_region(db));
    for param in hir_decl.parenate_parameters(db).iter() {
        match *param {
            HirLazyParenateParameter::SelfValue => unreachable!(),
            HirLazyParenateParameter::Simple { ty, .. } => builder.add_hir_ty(ty),
            HirLazyParenateParameter::Keyed { ty, .. } => builder.add_hir_ty(ty),
            HirLazyParenateParameter::Variadic { ty, .. } => builder.add_hir_ty(ty),
        }
    }
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_lazy_expr_region) = hir_defn.hir_lazy_expr_region(db) {
        builder.add_hir_lazy_expr_region(hir_lazy_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn function_gn_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: FunctionGnHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
