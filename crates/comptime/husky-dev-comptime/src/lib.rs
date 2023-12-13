pub mod db;

use self::db::DevComptimeDb;
use husky_coword::Kebab;
use husky_task::{helpers::TaskDevLinkTime, linktime::IsLinktime, IsTask};
use husky_toolchain_config::toolchain_config;
use husky_vfs::{
    error::VfsResult, linktime_target_path::LinktimeTargetPath, CrateKind, CratePath, PackagePath,
    VfsDb, VfsJar,
};
use std::path::Path;

pub struct DevComptime<Task: IsTask> {
    db: DevComptimeDb,
    target: DevComptimeTarget,
    linktime: TaskDevLinkTime<Task>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = VfsDb, jar = VfsJar)]
pub enum DevComptimeTarget {
    None,
    SingleCrate(CratePath),
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn new(target_crate_path: &Path) -> VfsResult<Self> {
        let db = DevComptimeDb::default();
        let toolchain = toolchain_config(target_crate_path, &*db).toolchain();
        let target_package_path = match PackagePath::new_local_or_toolchain_package(
            &db,
            toolchain,
            Kebab::from_ref(&db, "mnist-classifier").unwrap(),
            target_crate_path,
        ) {
            Ok(package_path) => package_path,
            Err(_e) => todo!(),
        };
        let target_crate_path = CratePath::new(target_package_path, CrateKind::Main, &db)?;
        Ok(Self {
            linktime: IsLinktime::new_linktime(
                /* ad hoc */
                LinktimeTargetPath::new_package(target_crate_path.package_path(&db), &db),
                &db,
            ),
            target: DevComptimeTarget::SingleCrate(target_crate_path),
            db,
        })
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.target
    }
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
            db: todo!(),
            linktime: Default::default(),
        }
    }
}
