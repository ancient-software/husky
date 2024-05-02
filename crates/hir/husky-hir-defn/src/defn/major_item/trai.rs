use super::*;
use husky_hir_decl::decl::TraitHirDecl;

#[salsa::interned]
pub struct TraitHirDefn {
    pub path: TraitPath,
    pub hir_decl: TraitHirDecl,
}

impl From<TraitHirDefn> for HirDefn {
    fn from(hir_defn: TraitHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl HasHirDefn for TraitPath {
    type HirDefn = TraitHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        trai_hir_defn(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn trai_hir_defn(db: &::salsa::Db, path: TraitPath) -> Option<TraitHirDefn> {
    let hir_decl = path.hir_decl(db)?;
    Some(TraitHirDefn::new(db, path, hir_decl))
}

impl TraitHirDefn {
    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        trai_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        trai_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn trai_hir_defn_dependencies(db: &::salsa::Db, hir_defn: TraitHirDefn) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    // todo: add traits that this trait depends on
    // comment out temporarilly
    // for &(_, path) in hir_defn.path(db).assoc_item_paths(db) {
    //     builder.add_item_path(path)
    // }
    builder.finish()
}

#[salsa::tracked]
fn trai_hir_defn_version_stamp(db: &::salsa::Db, hir_defn: TraitHirDefn) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
