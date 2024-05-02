use super::*;
use husky_entity_path::path::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockPath;
use husky_hir_decl::decl::TraitForTypeImplBlockHirDecl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[salsa::as_id(jar = HirDefnJar)]
pub struct TraitForTypeImplBlockHirDefn {
    hir_decl: TraitForTypeImplBlockHirDecl,
}

impl From<TraitForTypeImplBlockHirDefn> for HirDefn {
    fn from(hir_defn: TraitForTypeImplBlockHirDefn) -> Self {
        HirDefn::ImplBlock(hir_defn.into())
    }
}

impl HasHirDefn for TraitForTypeImplBlockPath {
    type HirDefn = TraitForTypeImplBlockHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        Some(TraitForTypeImplBlockHirDefn {
            hir_decl: self.hir_decl(db)?,
        })
    }
}

impl TraitForTypeImplBlockHirDefn {
    pub fn path(self, db: &::salsa::Db) -> TraitForTypeImplBlockPath {
        self.hir_decl.path(db)
    }

    pub fn hir_decl(self) -> TraitForTypeImplBlockHirDecl {
        self.hir_decl
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        trai_for_ty_impl_block_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        trai_for_ty_impl_block_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn trai_for_ty_impl_block_dependencies(
    db: &::salsa::Db,
    hir_defn: TraitForTypeImplBlockHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl();
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_trai(hir_decl.trai(db));
    builder.add_hir_ty(hir_decl.self_ty(db));
    builder.finish()
}

#[salsa::tracked]
fn trai_for_ty_impl_block_version_stamp(
    db: &::salsa::Db,
    hir_defn: TraitForTypeImplBlockHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
