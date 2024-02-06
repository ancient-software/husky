use super::*;
use husky_hir_decl::decl::TraitAssocValHirDecl;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssocValHirDefn {
    pub path: TraitItemPath,
    pub hir_decl: TraitAssocValHirDecl,
    pub hir_expr_region: Option<HirExprRegion>,
}

impl From<TraitAssocValHirDefn> for AssocItemHirDefn {
    fn from(hir_defn: TraitAssocValHirDefn) -> Self {
        AssocItemHirDefn::TraitItem(hir_defn.into())
    }
}

impl From<TraitAssocValHirDefn> for HirDefn {
    fn from(hir_defn: TraitAssocValHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TraitAssocValHirDefn {
    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        trai_associated_val_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        trai_associated_val_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_associated_val_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: TraitAssocValHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_item_path(hir_decl.path(db).trai_path(db));
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_expr_region) = hir_defn.hir_expr_region(db) {
        builder.add_hir_expr_region(hir_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_associated_val_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TraitAssocValHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
