mod r#enum;
mod r#extern;
mod props_struct;
mod tuple_struct;
mod r#union;
mod unit_struct;

pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::r#union::*;
pub use self::tuple_struct::*;
pub use self::unit_struct::*;

use super::*;
use husky_entity_path::path::major_item::ty::TypePath;
use husky_eth_signature::signature::major_item::ty::props_struct::PropsStructFieldEthSignature;
use husky_syn_decl::decl::major_item::ty::TypeSynDecl;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeHirDecl {
    Enum(EnumHirDecl),
    PropsStruct(PropsStructHirDecl),
    UnitStruct(UnitStructHirDecl),
    TupleStruct(TupleStructHirDecl),
    Extern(ExternTypeHirDecl),
    Union(UnionHirDecl),
}

impl TypeHirDecl {
    pub fn path(self, db: &::salsa::Db) -> TypePath {
        match self {
            TypeHirDecl::Enum(decl) => decl.path(db),
            TypeHirDecl::UnitStruct(decl) => decl.path(db),
            TypeHirDecl::PropsStruct(decl) => decl.path(db),
            TypeHirDecl::TupleStruct(decl) => decl.path(db),
            TypeHirDecl::Extern(decl) => decl.path(db),
            TypeHirDecl::Union(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a HirTemplateParameters {
        match self {
            TypeHirDecl::Enum(decl) => decl.template_parameters(db),
            TypeHirDecl::UnitStruct(decl) => decl.template_parameters(db),
            TypeHirDecl::TupleStruct(decl) => decl.template_parameters(db),
            TypeHirDecl::PropsStruct(decl) => decl.template_parameters(db),
            TypeHirDecl::Extern(decl) => decl.template_parameters(db),
            TypeHirDecl::Union(decl) => decl.template_parameters(db),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            TypeHirDecl::Enum(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::UnitStruct(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::TupleStruct(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::PropsStruct(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::Extern(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::Union(decl) => decl.hir_eager_expr_region(db).into(),
        }
    }
}

impl HasHirDecl for TypePath {
    type HirDecl = TypeHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        ty_hir_decl(db, self)
    }
}

#[salsa::tracked]
fn ty_hir_decl(db: &::salsa::Db, path: TypePath) -> Option<TypeHirDecl> {
    match path.syn_decl(db).expect("no errors for hir stage") {
        TypeSynDecl::Enum(syn_decl) => Some(EnumHirDecl::from_syn(path, syn_decl, db).into()),
        TypeSynDecl::PropsStruct(syn_decl) => {
            Some(PropsStructHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeSynDecl::UnitStruct(syn_decl) => {
            Some(UnitStructHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeSynDecl::TupleStruct(syn_decl) => {
            Some(TupleStructHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeSynDecl::Inductive(_syn_decl) => None,
        TypeSynDecl::Structure(_syn_decl) => None,
        TypeSynDecl::Extern(syn_decl) => {
            Some(ExternTypeHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeSynDecl::Union(syn_decl) => Some(UnionHirDecl::from_syn(path, syn_decl, db).into()),
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum PropsFieldEthSignature {
    PropsStruct(PropsStructFieldEthSignature),
}
