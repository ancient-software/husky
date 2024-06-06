use crate::{
    defn::module_defn_rust_transpilation,
    linkage::package_linkages_transpilation,
    manifest::{package_linkages_rust_package_manifest, package_source_rust_package_manifest},
    *,
};
use ::relative_path::RelativePathBuf;
use husky_corgi_config::transpilation_setup::TranspilationSetup;
use husky_entity_tree::helpers::paths::crate_module_paths;
use husky_io_utils::error::IOResult;
use husky_manifest::HasManifest;
use husky_vfs::{
    path::{
        crate_path::CrateKind,
        linktime_target_path::{LinktimeTargetPath, LinktimeTargetPathData},
        module_path::ModulePathData,
    },
    *,
};
use pathdiff::diff_paths;
use std::path::Path;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RustTranspilationPackage {
    pub(crate) target_path: LinktimeTargetPath,
    pub(crate) package_path: PackagePath,
    pub(crate) kind: RustTranspilationPackageKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustTranspilationPackageKind {
    Source,
    Linkages,
}

impl RustTranspilationPackage {
    pub(crate) fn name(self, db: &::salsa::Db) -> String {
        match self.kind {
            RustTranspilationPackageKind::Source => self.package_path.name(db).data(db).to_string(),
            RustTranspilationPackageKind::Linkages => {
                format!("{}-linkages", self.package_path.name(db).data(db))
            }
        }
    }

    pub(crate) fn path_in_workspace(
        self,
        rust_workspace_abs_dir: &Path,
        db: &::salsa::Db,
    ) -> String {
        match self.kind {
            RustTranspilationPackageKind::Source => {
                if self.package_path.is_virtual(db) {
                    diff_paths(
                        self.package_path.dir(db).unwrap().abs_path(db).unwrap(),
                        rust_workspace_abs_dir,
                    )
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string()
                } else {
                    self.package_path.name(db).data(db).to_string()
                }
            }
            RustTranspilationPackageKind::Linkages => {
                format!("{}/linkages", self.package_path.name(db).data(db))
            }
        }
    }
}

#[salsa::tracked(return_ref)]
pub(crate) fn rust_transpilation_packages(
    db: &::salsa::Db,
    target_path: LinktimeTargetPath,
) -> Vec<RustTranspilationPackage> {
    match target_path.data(db) {
        LinktimeTargetPathData::Package(package_path) => {
            let mut packages = vec![
                RustTranspilationPackage {
                    target_path,
                    package_path,
                    kind: RustTranspilationPackageKind::Source,
                },
                RustTranspilationPackage {
                    target_path,
                    package_path,
                    kind: RustTranspilationPackageKind::Linkages,
                },
            ];
            packages.extend(
                package_path
                    .dependencies(db)
                    .expect("no error at this stage")
                    .iter()
                    .flat_map(|dep| {
                        [
                            RustTranspilationPackage {
                                target_path,
                                package_path: dep.package_path(),
                                kind: RustTranspilationPackageKind::Source,
                            },
                            RustTranspilationPackage {
                                target_path,
                                package_path: dep.package_path(),
                                kind: RustTranspilationPackageKind::Linkages,
                            },
                        ]
                    }),
            );
            packages
        }
        LinktimeTargetPathData::Workspace(_) => todo!(),
    }
}

#[test]
fn rust_transpilation_packages_works() {
    DB::ast_rich_test_debug_with_db(
        |db, package_path: PackagePath| {
            let linktime_target_path = LinktimeTargetPath::new_package(package_path, db);
            rust_transpilation_packages(db, linktime_target_path)
        },
        &AstTestConfig::new(
            "rust_transpilation_packages",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::COMPTIME,
        ),
    )
}

impl RustTranspilationPackage {
    pub(crate) fn transpile_to_fs(
        &self,
        setup: TranspilationSetup,
        db: &::salsa::Db,
    ) -> IOResult<()> {
        let workspace_dir = self.target_path.rust_workspace_abs_dir(db);
        match self.kind {
            package::RustTranspilationPackageKind::Source => {
                transpile_package_source_to_fs(setup, workspace_dir, self.package_path, db)
            }
            package::RustTranspilationPackageKind::Linkages => {
                transpile_package_linkages_to_fs(setup, workspace_dir, self.package_path, db)
            }
        }
    }
}

fn transpile_package_source_to_fs(
    setup: TranspilationSetup,
    rust_workspace_dir: &std::path::Path,
    package_path: PackagePath,
    db: &::salsa::Db,
) -> IOResult<()> {
    if package_path.is_virtual(db) {
        return Ok(());
    }
    let package_dir = rust_workspace_dir.join(package_path.name(db).data(db));
    let cargo_toml_path = package_dir.join("Cargo.toml");
    husky_io_utils::diff_write(
        &cargo_toml_path,
        package_source_rust_package_manifest(db, package_path, setup),
        true,
    );
    for &crate_path in package_path
        .crate_paths(db)
        .expect("no vfs error at this stage")
    {
        match crate_path.kind(db) {
            CrateKind::Lib | CrateKind::Main => {
                for &module_path in crate_module_paths(db, crate_path) {
                    husky_io_utils::diff_write(
                        &module_relative_path_for_transpilation(db, module_path)
                            .to_path(&package_dir),
                        module_defn_rust_transpilation(db, module_path, setup),
                        true,
                    );
                }
            }
            CrateKind::Requirements | CrateKind::Task => (),
            CrateKind::Bin(_) => todo!(),
            CrateKind::IntegratedTest(_) => todo!(),
            CrateKind::Example => todo!(),
        }
    }
    Ok(())
}

#[salsa::tracked(return_ref)]
fn module_relative_path_for_transpilation(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> RelativePathBuf {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => match crate_path.kind(db) {
            CrateKind::Lib | CrateKind::Main => RelativePathBuf::from_path("lib.rs").unwrap(),
            CrateKind::Requirements | CrateKind::Task => todo!(),
            CrateKind::Bin(_) => todo!(),
            CrateKind::IntegratedTest(_) => todo!(),
            CrateKind::Example => todo!(),
        },
        ModulePathData::Child { .. } => module_path
            .relative_dir_for_submodules(db)
            .with_extension("rs"),
        ModulePathData::Script { .. } => unreachable!(),
    }
}

fn transpile_package_linkages_to_fs(
    setup: TranspilationSetup,
    rust_workspace_dir: &std::path::Path,
    package_path: PackagePath,
    db: &::salsa::Db,
) -> IOResult<()> {
    let package_dir = rust_workspace_dir
        .join(package_path.name(db).data(db))
        .join("linkages");
    let src_dir = package_dir.join("src");
    let cargo_toml_path = package_dir.join("Cargo.toml");
    husky_io_utils::diff_write(
        &cargo_toml_path,
        package_linkages_rust_package_manifest(db, package_path, setup),
        true,
    );
    husky_io_utils::diff_write(
        &src_dir.join("lib.rs"),
        package_linkages_transpilation(db, package_path, setup),
        true,
    );
    Ok(())
}
