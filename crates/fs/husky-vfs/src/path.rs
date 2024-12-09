pub mod crate_path;
pub mod linktime_target_path;
pub mod menu;
pub mod module_path;
pub mod package_path;
pub mod virtual_path;
pub mod workspace_path;

pub(crate) use self::crate_path::*;
pub(crate) use self::module_path::*;
pub(crate) use self::package_path::*;
pub(crate) use self::virtual_path::*;

use crate::*;

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn module_virtual_path(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> VfsResult<Option<VirtualPath>> {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => VirtualPath::try_new(
            db,
            &package_dir(db, crate_path.package_path(db))
                .as_ref()?
                .data()
                .join(crate_path.relative_path(db).as_ref()),
        )
        .map(Some)
        .map_err(|e| e.into()),
        ModulePathData::Child { parent, ident } => {
            let parent_module_path = module_virtual_path(db, parent)?.unwrap();
            let dir = match parent.data(db) {
                ModulePathData::Root(_) => parent_module_path.data().parent().unwrap().to_owned(),
                ModulePathData::Child {
                    parent: _,
                    ident: _,
                } => parent_module_path.data().with_extension(""),
                ModulePathData::Script { .. } => unreachable!(),
            };
            VirtualPath::try_new(db, &dir.join(ident.data()).with_extension("hsy")).map(Some)
        }
        ModulePathData::Script { .. } => Ok(None),
    }
}

// this shouldn't be tracked
// todo: add tests for this
pub(crate) fn resolve_module_path(
    db: &::salsa::Db,
    toolchain: Toolchain,
    path: impl AsRef<Path>,
) -> VfsResult<ModulePath> {
    let path = path.as_ref();
    if !path.exists() {
        todo!()
    }
    if path.extension().and_then(|s| s.to_str()) != Some("hsy") {
        todo!()
    }
    let parent = path.parent().ok_or(VfsError::ModulePathResolveFailure)?;
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(VfsError::ModulePathResolveFailure)?;
    Ok(if parent.join("Corgi.toml").exists() {
        match file_stem {
            "requirements" => CratePath::new(
                PackagePath::new_local_or_toolchain_package(db, toolchain, parent)?,
                CrateKind::Requirements,
                db,
            )
            .expect("should be guaranteed to exist because path exists!")
            .root_module_path(db),
            "task" => CratePath::new(
                PackagePath::new_local_or_toolchain_package(db, toolchain, parent)?,
                CrateKind::Task,
                db,
            )
            .expect("should be guaranteed to exist because path exists!")
            .root_module_path(db),
            _ => todo!(),
        }
    } else if parent.ends_with("src") {
        match file_stem {
            "lib" => CratePath::new(
                PackagePath::new_local_or_toolchain_package(
                    db,
                    toolchain,
                    parent.parent().ok_or(VfsError::ModulePathResolveFailure)?,
                )?,
                CrateKind::Lib,
                db,
            )
            .expect("should be guaranteed to exist because path exists!")
            .root_module_path(db),
            "main" => CratePath::new(
                PackagePath::new_local_or_toolchain_package(
                    db,
                    toolchain,
                    parent.parent().ok_or(VfsError::ModulePathResolveFailure)?,
                )?,
                CrateKind::Main,
                db,
            )
            .expect("should be guaranteed to exist because path exists!")
            .root_module_path(db),
            _ => {
                let lib_path = parent.join("lib.hsy");
                if lib_path.exists() {
                    ModulePath::new_child(
                        db,
                        resolve_module_path(db, toolchain, lib_path)?,
                        Ident::from_ref(db, file_stem).ok_or(VfsError::ModulePathResolveFailure)?,
                    )
                    .expect("should be guaranteed to exist because path exists!")
                    .into()
                } else {
                    let main_path = parent.join("main.hsy");
                    if main_path.exists() {
                        ModulePath::new_child(
                            db,
                            resolve_module_path(db, toolchain, main_path)?,
                            Ident::from_ref(db, file_stem)
                                .ok_or(VfsError::ModulePathResolveFailure)?,
                        )
                        .expect("should be guaranteed to exist because path exists!")
                        .into()
                    } else {
                        todo!()
                    }
                }
            }
        }
    } else {
        let parent_module_path = parent.with_extension("hsy");
        if !parent_module_path.exists() {
            todo!()
        }
        ModulePath::new_child(
            db,
            resolve_module_path(db, toolchain, parent_module_path)?,
            Ident::from_ref(db, file_stem).ok_or(VfsError::ModulePathResolveFailure)?,
        )
        .expect("should be guaranteed to exist because path exists!")
        .into()
    })
}

#[test]
fn resolve_module_path_works() {
    DB::vfs_plain_test(
        |db, module_path| {
            if let Some(virtual_path) = module_virtual_path(db, module_path).unwrap() {
                let item_path_resolved = db
                    .resolve_module_path_and_update_live_packages(virtual_path.data())
                    .unwrap();
                assert_eq!(module_path, item_path_resolved);
            }
        },
        &VfsTestConfig::new(
            "resolve-module-path",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::FS,
        ),
    )
}
