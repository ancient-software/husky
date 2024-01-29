use super::*;
use crate::helpers::paths::HasItemPaths;
use husky_entity_kind::*;
use husky_entity_path::{ItemPath, ItemPathId};
use husky_task_interface::TaskIngredientIndex;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IngredientPath {
    item_path: ItemPath,
}

pub trait HasIngredientPaths: is::Is<CratePath> {
    fn ingredient_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [IngredientPath];
    fn has_ingredients(self, db: &::salsa::Db) -> bool {
        !self.ingredient_paths(db).is_empty()
    }
}

impl HasIngredientPaths for CratePath {
    fn ingredient_paths<'a>(self, db: &'a salsa::Db) -> &'a [IngredientPath] {
        crate_ingredient_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn crate_ingredient_paths(
    db: &::salsa::Db,
    crate_path: CratePath,
) -> Vec<IngredientPath> {
    crate_path
        .item_paths(db)
        .iter()
        .filter_map(|&item_path| IngredientPath::from_item_path(item_path, db))
        .collect()
}

impl IngredientPath {
    fn from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        Self::is_ingredient(item_path, db).then_some(Self { item_path })
    }

    fn is_ingredient(path: ItemPath, db: &::salsa::Db) -> bool {
        match path.item_kind(db) {
            EntityKind::Module => false,
            EntityKind::MajorItem {
                module_item_kind,
                connection: _,
            } => match module_item_kind {
                MajorItemKind::Type(_) => false,
                MajorItemKind::Fugitive(fugitive_kind) => match fugitive_kind {
                    FugitiveKind::FunctionFn => false,
                    // gn doesn't directly needs jars
                    FugitiveKind::FunctionGn => false,
                    FugitiveKind::AliasType => false,
                    FugitiveKind::Val => true,
                    FugitiveKind::Formal => false,
                },
                MajorItemKind::Trait => false,
            },
            EntityKind::AssociatedItem {
                associated_item_kind,
            } => match associated_item_kind {
                AssociatedItemKind::TraitItem(TraitItemKind::AssociatedVal)
                | AssociatedItemKind::TypeItem(
                    TypeItemKind::MemoizedField | TypeItemKind::AssociatedVal,
                )
                | AssociatedItemKind::TraitForTypeItem(
                    TraitItemKind::MemoizedField | TraitItemKind::AssociatedVal,
                ) => true,
                _ => false,
            },
            EntityKind::TypeVariant => false,
            EntityKind::ImplBlock => false,
            EntityKind::Attr => false,
        }
    }

    pub fn item_path(self) -> ItemPath {
        self.item_path
    }
}

pub trait HasIngredientIndex: Into<ItemPath> {
    fn ingredient_index(self, db: &::salsa::Db) -> Option<TaskIngredientIndex>;
}

impl<P> HasIngredientIndex for P
where
    P: Into<ItemPath>,
{
    fn ingredient_index(self, db: &::salsa::Db) -> Option<TaskIngredientIndex> {
        item_path_ingredient_index(db, *self.into())
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn item_path_ingredient_index(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> Option<TaskIngredientIndex> {
    item_path_id
        .crate_path(db)
        .ingredient_paths(db)
        .iter()
        .position(|ingredient_path| *ingredient_path.item_path == item_path_id)
        .map(|raw| TaskIngredientIndex::from_index(raw))
}
