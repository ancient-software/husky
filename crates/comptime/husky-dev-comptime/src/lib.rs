pub mod db;

use self::db::DevComptimeDb;

use husky_entity_kind::{FugitiveKind, TraitItemKind, TypeItemKind};
use husky_entity_path::{AssocItemPath, ItemPath, MajorItemPath};
use husky_entity_tree::helpers::ingredient::{HasIngredientPaths, IngredientPath};
use husky_linkage::linkage::Linkage;
use husky_manifest::HasAllPackages;
use husky_task::{
    helpers::{TaskDevLinkTime, TaskDevLinkageImpl},
    linktime::IsLinktime,
    IsTask,
};
use husky_task_interface::TaskIngredientIndex;
use husky_task_interface::TaskJarIndex;
use husky_toolchain_config::toolchain_config;
use husky_val::Val;
use husky_val_repr::repr::ValRepr;
use husky_vfs::{
    error::VfsResult, linktime_target_path::LinktimeTargetPath, CrateKind, CratePath, PackagePath,
};
use std::path::Path;

pub struct DevComptime<Task: IsTask> {
    db: DevComptimeDb,
    target: DevComptimeTarget,
    target_path: Option<LinktimeTargetPath>,
    linktime: TaskDevLinkTime<Task>,
    ingredient_vals: Vec<(
        PackagePath,
        Vec<(IngredientPath, Option<ValRepr>, Option<Val>)>,
    )>,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DevComptimeTarget {
    None,
    SingleCrate(CratePath),
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn new(target_crate_path: impl AsRef<Path>) -> VfsResult<Self> {
        let target_crate_path = target_crate_path.as_ref();
        let db = DevComptimeDb::default();
        let toolchain = toolchain_config(target_crate_path, &*db).toolchain();
        let target_package_path =
            match PackagePath::new_local_or_toolchain_package(&db, toolchain, target_crate_path) {
                Ok(package_path) => package_path,
                Err(_e) => todo!(),
            };
        let target_crate_path = CratePath::new(target_package_path, CrateKind::Main, &db)?;
        let target = DevComptimeTarget::SingleCrate(target_crate_path);
        let target_path = match target {
            DevComptimeTarget::None => None,
            DevComptimeTarget::SingleCrate(crate_path) => Some(LinktimeTargetPath::new_package(
                crate_path.package_path(&db),
                &db,
            )),
        };
        let ingredient_vals = target_path
            .map(|target_path| ingredient_vals(target_path, &db))
            .unwrap_or_default();
        Ok(Self {
            linktime: IsLinktime::new_linktime(
                /* ad hoc */
                LinktimeTargetPath::new_package(target_crate_path.package_path(&db), &db),
                &db,
            ),
            target,
            target_path,
            db,
            ingredient_vals,
        })
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.target
    }

    pub fn linktime_target_path(&self) -> Option<LinktimeTargetPath> {
        self.target_path
    }

    pub fn linkage_impl(&self, linkage: Linkage) -> TaskDevLinkageImpl<Task> {
        self.linktime.linkage_impl(linkage, self.db())
    }

    pub fn ingredient_val(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
    ) -> Val {
        self.ingredient_vals[jar_index.index()].1[ingredient_index.index()]
            .2
            .unwrap()
    }

    pub fn ingredient_val_repr(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
    ) -> ValRepr {
        self.ingredient_vals[jar_index.index()].1[ingredient_index.index()]
            .1
            .unwrap()
    }
}

fn ingredient_vals(
    target_path: LinktimeTargetPath,
    db: &::salsa::Db,
) -> Vec<(
    PackagePath,
    Vec<(IngredientPath, Option<ValRepr>, Option<Val>)>,
)> {
    target_path
        .all_packages(db)
        .unwrap()
        .iter()
        .map(|&package_path| {
            let crate_path = package_path
                .lib_crate_path(db)
                .or(package_path.main_crate_path(db))
                .unwrap();
            (
                package_path,
                crate_path
                    .ingredient_paths(db)
                    .iter()
                    .map(|&ingredient_path| {
                        let val_repr = match ingredient_path.item_path() {
                            ItemPath::MajorItem(MajorItemPath::Fugitive(path))
                                if path.fugitive_kind(db) == FugitiveKind::Val =>
                            {
                                Some(ValRepr::new_val_item(path, db))
                            }
                            ItemPath::AssocItem(path) => match path {
                                AssocItemPath::TypeItem(path) => match path.item_kind(db) {
                                    TypeItemKind::AssocVal => todo!(),
                                    _ => None,
                                },
                                AssocItemPath::TraitItem(path) => match path.item_kind(db) {
                                    TraitItemKind::AssocVal => todo!(),
                                    _ => None,
                                },
                                AssocItemPath::TraitForTypeItem(path) => match path.item_kind(db) {
                                    TraitItemKind::AssocVal => todo!(),
                                    _ => None,
                                },
                            },
                            _ => None,
                        };
                        let val = val_repr.map(|val_repr| val_repr.val(db));
                        (ingredient_path, val_repr, val)
                    })
                    .collect(),
            )
        })
        .collect()
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn db(&self) -> &::salsa::Db {
        &self.db
    }
}

impl<Task: IsTask> Default for DevComptime<Task>
where
    TaskDevLinkTime<Task>: Default,
{
    fn default() -> Self {
        Self {
            target: DevComptimeTarget::None,
            db: Default::default(),
            linktime: Default::default(),
            target_path: None,
            ingredient_vals: vec![],
        }
    }
}
