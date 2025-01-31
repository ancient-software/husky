pub mod assoc_item;
pub mod attr;
#[path = "decl/crate.rs"]
pub mod crate_;
pub mod impl_block;
pub mod major_item;
pub mod submodule;
pub mod ty_variant;

pub use self::assoc_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::{jar::*, *};
use husky_entity_path::path::ItemPath;
use husky_hir_eager_expr::helpers::region::hir_eager_expr_region;
use husky_syn_decl::decl::HasSynDecl;

pub trait HasHirDecl {
    type HirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl>;
}

impl HasHirDecl for ItemPath {
    type HirDecl = HirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        Some(match self {
            ItemPath::Submodule(_, path) => path.hir_decl(db)?.into(),
            ItemPath::MajorItem(path) => path.hir_decl(db)?.into(),
            ItemPath::AssocItem(path) => path.hir_decl(db)?.into(),
            ItemPath::TypeVariant(_, path) => path.hir_decl(db)?.into(),
            ItemPath::ImplBlock(path) => path.hir_decl(db)?.into(),
            ItemPath::Attr(_, path) => path.hir_decl(db)?.into(),
            ItemPath::Script(_, _) => todo!(),
        })
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum HirDecl {
    Submodule(SubmoduleHirDecl),
    MajorItem(MajorItemHirDecl),
    ImplBlock(ImplBlockHirDecl),
    AssocItem(AssocItemHirDecl),
    TypeVariant(TypeVariantHirDecl),
    Attr(AttrHirDecl),
}

impl HirDecl {
    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> Option<&'a HirTemplateParameters> {
        match self {
            HirDecl::Submodule(_) => None,
            HirDecl::MajorItem(decl) => decl.template_parameters(db),
            HirDecl::ImplBlock(decl) => Some(decl.template_parameters(db)),
            HirDecl::AssocItem(decl) => decl.template_parameters(db),
            HirDecl::TypeVariant(_) => None,
            HirDecl::Attr(_) => None,
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            HirDecl::Submodule(decl) => None,
            HirDecl::MajorItem(decl) => Some(decl.hir_expr_region(db)),
            HirDecl::ImplBlock(decl) => Some(decl.hir_eager_expr_region(db).into()),
            HirDecl::AssocItem(decl) => Some(decl.hir_expr_region(db)),
            HirDecl::TypeVariant(decl) => Some(decl.hir_expr_region(db)),
            HirDecl::Attr(decl) => None,
        }
    }

    pub fn path(self, db: &::salsa::Db) -> ItemPath {
        match self {
            HirDecl::Submodule(_) => todo!(),
            HirDecl::MajorItem(decl) => decl.path(db).into(),
            HirDecl::ImplBlock(decl) => decl.path(db).into(),
            HirDecl::AssocItem(decl) => decl.path(db).into(),
            HirDecl::TypeVariant(decl) => decl.path(db).into(),
            HirDecl::Attr(decl) => decl.path(db).into(),
        }
    }
}
