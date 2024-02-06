mod associated_item;
mod attr;
mod impl_block;
mod major_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use husky_iter_utils::chain_as_ref_err_collect;
use husky_regional_token::*;
use husky_token_data::{TokenData, TokenDataResult};

type SmallVecImpl<T> = smallvec::SmallVec<[T; 2]>;

/// A `NodeDecl` is a tolerant information-preserving declaration
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum ItemSynNodeDecl {
    Submodule(SubmoduleSynNodeDecl),
    MajorItem(MajorItemSynNodeDecl),
    ImplBlock(ImplBlockSynNodeDecl),
    AssocItem(AssocItemSynNodeDecl),
    TypeVariant(TypeVariantSynNodeDecl),
    Attr(AttrSynNodeDecl),
}

impl ItemSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> Option<SynExprRegion> {
        match self {
            ItemSynNodeDecl::Submodule(_) => None,
            ItemSynNodeDecl::MajorItem(slf) => slf.syn_expr_region(db).into(),
            ItemSynNodeDecl::ImplBlock(slf) => Some(slf.syn_expr_region(db)),
            ItemSynNodeDecl::AssocItem(slf) => Some(slf.syn_expr_region(db)),
            ItemSynNodeDecl::TypeVariant(slf) => Some(slf.syn_expr_region(db)),
            ItemSynNodeDecl::Attr(slf) => Some(slf.syn_expr_region(db)),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            ItemSynNodeDecl::Submodule(slf) => slf.errors(db),
            ItemSynNodeDecl::MajorItem(slf) => slf.errors(db),
            ItemSynNodeDecl::ImplBlock(slf) => slf.errors(db),
            ItemSynNodeDecl::AssocItem(slf) => slf.errors(db),
            ItemSynNodeDecl::TypeVariant(slf) => slf.errors(db),
            ItemSynNodeDecl::Attr(slf) => slf.errors(db),
        }
    }
}

/// A `Decl` is a strict version, handy for subsequent processing
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum SynDecl {
    Submodule(SubmoduleSynDecl),
    MajorItem(MajorItemSynDecl),
    ImplBlock(ImplBlockSynDecl),
    AssocItem(AssocItemSynDecl),
    TypeVariant(TypeVariantSynDecl),
    Attr(AttrSynDecl),
}

impl SynDecl {
    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            SynDecl::MajorItem(slf) => slf.template_parameters(db),
            SynDecl::ImplBlock(slf) => slf.template_parameters(db),
            SynDecl::AssocItem(slf) => slf.template_parameters(db),
            SynDecl::Submodule(_) | SynDecl::TypeVariant(_) | SynDecl::Attr(_) => &[],
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> Option<SynExprRegion> {
        match self {
            SynDecl::Submodule(_) => None,
            SynDecl::MajorItem(slf) => slf.syn_expr_region(db).into(),
            SynDecl::ImplBlock(slf) => slf.syn_expr_region(db).into(),
            SynDecl::AssocItem(slf) => slf.syn_expr_region(db).into(),
            SynDecl::TypeVariant(slf) => Some(slf.syn_expr_region(db)),
            SynDecl::Attr(slf) => Some(slf.syn_expr_region(db)),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> ItemPath {
        match self {
            SynDecl::Submodule(slf) => slf.path(db).into(),
            SynDecl::MajorItem(slf) => slf.path(db).into(),
            SynDecl::ImplBlock(slf) => slf.path(db).into(),
            SynDecl::AssocItem(slf) => slf.path(db).into(),
            SynDecl::TypeVariant(slf) => slf.path(db).into(),
            SynDecl::Attr(slf) => slf.path(db).into(),
        }
    }
}

pub trait HasSynNodeDecl: Copy {
    type NodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl;
}

impl HasSynNodeDecl for ItemSynNodePath {
    type NodeDecl = ItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        match self {
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::TypeVariant(_, syn_node_path) => {
                syn_node_path.syn_node_decl(db).into()
            }
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::AssocItem(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::Submodule(_, syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::Attr(_, syn_node_path) => syn_node_path.syn_node_decl(db).into(),
        }
    }
}

pub trait HasSynDecl: Copy {
    type Decl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl>;
}

impl HasSynDecl for ItemPath {
    type Decl = SynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        match self {
            ItemPath::Submodule(_, path) => path.syn_decl(db).map(Into::into),
            ItemPath::MajorItem(path) => path.syn_decl(db).map(Into::into),
            ItemPath::AssocItem(path) => path.syn_decl(db).map(Into::into),
            ItemPath::TypeVariant(_, path) => path.syn_decl(db).map(Into::into),
            ItemPath::ImplBlock(path) => path.syn_decl(db).map(Into::into),
            ItemPath::Attr(_, path) => path.syn_decl(db).map(Into::into),
        }
    }
}
