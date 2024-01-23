mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use trai_for_ty_item::*;
pub use trai_item::*;
pub use ty_item::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum AssociatedItemPath {
    TypeItem(TypeItemPath),
    TraitItem(TraitItemPath),
    TraitForTypeItem(TraitForTypeItemPath),
}

impl std::ops::Deref for AssociatedItemPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum AssociatedItemPathData {
    TypeItem(TypeItemPathData),
    TraitItem(TraitItemPathData),
    TraitForTypeItem(TraitForTypeItemPathData),
}

impl From<TraitItemPath> for ItemPath {
    fn from(v: TraitItemPath) -> Self {
        ItemPath::AssociatedItem(v.into())
    }
}

impl From<TypeItemPath> for ItemPath {
    fn from(v: TypeItemPath) -> Self {
        ItemPath::AssociatedItem(v.into())
    }
}

impl AssociatedItemPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> AssociatedItemPath {
        match self {
            AssociatedItemPathData::TypeItem(slf) => slf.item_path(id).into(),
            AssociatedItemPathData::TraitItem(slf) => slf.item_path(id).into(),
            AssociatedItemPathData::TraitForTypeItem(slf) => slf.item_path(id).into(),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            AssociatedItemPathData::TypeItem(data) => data.module_path(db),
            AssociatedItemPathData::TraitItem(data) => data.module_path(db),
            AssociatedItemPathData::TraitForTypeItem(data) => data.module_path(db),
        }
    }

    pub fn ident(self, _db: &::salsa::Db) -> Ident {
        match self {
            AssociatedItemPathData::TypeItem(slf) => slf.ident,
            AssociatedItemPathData::TraitItem(slf) => slf.ident,
            AssociatedItemPathData::TraitForTypeItem(slf) => slf.ident,
        }
    }

    pub(crate) fn entity_kind(self, db: &::salsa::Db) -> EntityKind {
        match self {
            AssociatedItemPathData::TypeItem(slf) => slf.entity_kind(db),
            AssociatedItemPathData::TraitItem(slf) => slf.entity_kind(db),
            AssociatedItemPathData::TraitForTypeItem(slf) => slf.entity_kind(db),
        }
    }
}

impl salsa::DisplayWithDb for AssociatedItemPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        match self {
            AssociatedItemPath::TypeItem(path) => path.display_with_db_fmt(f, db),
            AssociatedItemPath::TraitItem(path) => path.display_with_db_fmt(f, db),
            AssociatedItemPath::TraitForTypeItem(path) => path.display_with_db_fmt(f, db),
        }
    }
}
