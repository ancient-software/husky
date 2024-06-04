use super::*;
use husky_manifest::helpers::upstream::HasAllUpstreamPackages;
use husky_task_interface::TaskJarIndex;
use husky_vfs::path::{linktime_target_path::LinktimeTargetPath, package_path::PackagePath};

pub fn package_path_from_jar_index(
    target_path: LinktimeTargetPath,
    jar_index: TaskJarIndex,
    db: &::salsa::Db,
) -> PackagePath {
    target_path.all_upstream_packages(db).unwrap()[jar_index.index()]
}
