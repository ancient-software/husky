mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemHirDefn {
    AssociatedFn(TraitForTypeAssociatedFnHirDefn),
    MethodFn(TraitForTypeMethodFnHirDefn),
    AssociatedType(TraitForTypeAssociatedTypeHirDefn),
    AssociatedVal(TraitForTypeAssociatedValHirDefn),
}

impl TraitForTypeItemHirDefn {
    pub fn hir_decl(self, db: &dyn HirDefnDb) -> TraitForTypeItemHirDecl {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn path(self, _db: &dyn HirDefnDb) -> TraitForTypeItemPath {
        todo!()
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> HirExprRegion {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.hir_expr_region(db).into(),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.hir_expr_region(db).into(),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => {
                hir_defn.hir_expr_region(db).into()
            }
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.hir_expr_region(db).into(),
        }
    }
}

impl HasHirDefn for TraitForTypeItemPath {
    type HirDefn = TraitForTypeItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        trai_for_ty_item_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_for_ty_item_hir_defn(
    db: &dyn HirDefnDb,
    path: TraitForTypeItemPath,
) -> TraitForTypeItemHirDefn {
    match path.hir_decl(db) {
        TraitForTypeItemHirDecl::AssociatedFn(_) => todo!(),
        TraitForTypeItemHirDecl::MethodFn(hir_decl) => {
            TraitForTypeMethodFnHirDefn::new(db, path, hir_decl).into()
        }
        TraitForTypeItemHirDecl::AssociatedType(hir_decl) => {
            TraitForTypeAssociatedTypeHirDefn::new(db, path, hir_decl).into()
        }
        TraitForTypeItemHirDecl::AssociatedVal(hir_decl) => {
            TraitForTypeAssociatedValHirDefn::new(db, path, hir_decl).into()
        }
    }
}
