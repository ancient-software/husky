use crate::*;

use crate::path::menu::{vfs_path_menu, VfsPathMenu};
use husky_fs_specs::FsSpecsError;
use maybe_result::MaybeResult::JustOk;
use salsa::Db;
use vec_like::VecSet;

pub trait VfsDb {
    fn vfs_path_menu(&self, toolchain: Toolchain) -> &VfsPathMenu;
    /// live packages are those whose information have been queried or modified through this db
    fn live_packages(
        &self,
    ) -> std::sync::LockResult<std::sync::RwLockReadGuard<'_, VecSet<PackagePath>>>;
    fn collect_probable_modules(&self, package_path: PackagePath) -> Vec<ModulePath>;
    fn resolve_module_path_and_update_live_packages(&self, path: &Path) -> VfsResult<ModulePath>;
    fn published_toolchain_library_path(&self, toolchain: PublishedToolchain) -> &Path;
}

// don't leak this outside the crate
pub trait VfsDbInner {
    fn file_from_virtual_path(&self, path: VirtualPath) -> VfsResult<File>;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn vfs_db_mut(&mut self) -> &mut ::salsa::Db;
    fn vfs_cache(&self) -> &VfsCache;
    fn set_content(&mut self, path: &Path, content: FileContent) -> VfsResult<()>;
    fn refresh_file_from_disk(&mut self, path: &Path) -> VfsResult<()>
    where
        Self: 'static;
    fn corgi_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.vfs_jar().cache().corgi_install_path()
    }
    fn huskyup_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.vfs_jar().cache().huskyup_install_path()
    }
    fn is_inside_installed_corgi_or_huskyup(&self, path: &Path) -> VfsResult<bool> {
        Ok(path.starts_with(self.corgi_install_path()?)
            || path.starts_with(self.huskyup_install_path()?))
    }
    fn calc_durability(&self, path: &Path) -> VfsResult<salsa::Durability>;
}

impl VfsDbInner for Db {
    fn file_from_virtual_path(&self, abs_path: VirtualPath) -> VfsResult<File> {
        Ok(
            match self
                .vfs_jar()
                .cache()
                .files()
                .entry(abs_path.data().to_owned())
            {
                // If the file already exists in our cache then just return it.
                Entry::Occupied(entry) => *entry.get(),
                // If we haven't read this file yet set up the watch, read the
                // contents, store it in the cache, and return it.
                Entry::Vacant(entry) => {
                    let path = abs_path.data();
                    //  &path.path(self);
                    // ad hoc
                    // if let Some(watcher) = self.watcher() {
                    //     let watcher = &mut watcher.0.lock().unwrap();
                    //     watcher
                    //         .watcher()
                    //         .watch(path, RecursiveMode::NonRecursive)
                    //         .unwrap();
                    // }
                    let content = read_file_content(path);
                    *entry.insert(File::new(
                        self,
                        abs_path.clone(),
                        content,
                        self.calc_durability(path)?,
                    ))
                }
            },
        )
    }

    fn vfs_jar(&self) -> &VfsJar {
        self.jar::<VfsJar>().0
    }

    fn vfs_jar_mut(&mut self) -> &mut VfsJar {
        self.jar_mut::<VfsJar>().0
    }

    fn vfs_cache(&self) -> &VfsCache {
        self.vfs_jar().cache()
    }

    // todo: test this
    fn refresh_file_from_disk(&mut self, path: &Path) -> VfsResult<()>
    where
        Db: 'static,
    {
        let content = read_file_content(&path);
        self.set_content(path, content)
    }

    fn set_content(&mut self, path: &Path, content: FileContent) -> VfsResult<()> {
        let virtual_path = VirtualPath::try_new(self, path)?;
        let path = virtual_path.data();
        let durability = self.calc_durability(path)?;
        let file = match self
            .vfs_jar()
            .cache()
            .files()
            .entry(virtual_path.data().to_owned())
        {
            // If the file already exists in our cache then just return it.
            Entry::Occupied(entry) => *entry.get(),
            // If we haven't read this file yet set up the watch, read the
            // contents, store it in the cache, and return it.
            Entry::Vacant(entry) => {
                //  &path.path(self);
                // ad hoc
                // if let Some(watcher) = self.watcher() {
                //     let watcher = &mut watcher.0.lock().unwrap();
                //     watcher
                //         .watcher()
                //         .watch(path, RecursiveMode::NonRecursive)
                //         .unwrap();
                // }
                let content = read_file_content(path);
                *entry.insert(File::new(self, virtual_path.clone(), content, durability))
            }
        };
        file.set_content(self)?.to(content);
        Ok(())
    }

    fn calc_durability(&self, path: &Path) -> VfsResult<salsa::Durability> {
        Ok(if self.is_inside_installed_corgi_or_huskyup(path)? {
            salsa::Durability::HIGH
        } else {
            salsa::Durability::LOW
        })
    }

    fn vfs_db_mut(&mut self) -> &mut ::salsa::Db {
        self
    }
}

impl VfsDb for Db {
    fn vfs_path_menu(&self, toolchain: Toolchain) -> &VfsPathMenu {
        vfs_path_menu(self, toolchain)
    }

    fn live_packages(
        &self,
    ) -> std::sync::LockResult<std::sync::RwLockReadGuard<'_, VecSet<PackagePath>>> {
        self.vfs_cache().live_packages()
    }

    /// todo: should return not only ModulePath but also files with extension "hsy" but not included in any tree
    /// so the type should be ProbableModulePath maybe
    fn collect_probable_modules(&self, package: PackagePath) -> Vec<ModulePath> {
        fn collect_probable_modules_aux(
            db: &::salsa::Db,
            parent: ModulePath,
            dir: &Path,
            modules: &mut Vec<ModulePath>,
        ) -> VfsResult<()> {
            let read_dir =
                std::fs::read_dir(dir).map_err(|e| VfsError::new_io_error(dir.to_owned(), e))?;
            let mut paths: Vec<PathBuf> = read_dir
                .map(|entry| -> VfsResult<PathBuf> {
                    let entry = entry.map_err(|e| VfsError::new_io_error(dir.to_owned(), e))?;
                    Ok(entry.path())
                })
                .collect::<VfsResult<_>>()?;
            // sort is important for testing
            paths.sort();
            for path in paths {
                if path.is_dir() {
                    if let Some(ident) = path
                        .file_name()
                        .and_then(|filename| filename.to_str())
                        .and_then(|filename| Ident::from_ref(db, filename))
                    {
                        if let JustOk(child) = ModulePath::new_child(db, parent, ident) {
                            collect_probable_modules_aux(db, child.inner(), &path, modules)?
                        }
                    }
                } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hsy")
                {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let push_flag = match file_stem {
                            "main" | "lib" => match parent.data(db) {
                                ModulePathData::Root(_) => false,
                                ModulePathData::Child { .. } => true,
                                ModulePathData::Script { .. } => unreachable!(),
                            },
                            _ => true,
                        };
                        if push_flag {
                            if let Some(ident) = Ident::from_ref(db, file_stem) {
                                if let JustOk(new_child) = ModulePath::new_child(db, parent, ident)
                                {
                                    modules.push(new_child.into())
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        let mut module_paths = vec![];
        let Ok(diff_path) = package.dir(self) else {
            return vec![];
        };
        let Ok(crate_paths) = package.crate_paths(self) else {
            return vec![];
        };
        for crate_path in crate_paths {
            let root_module_path = crate_path.root_module_path(self);
            module_paths.push(root_module_path);
            if let Ok(crate_dir) = crate_path.dir(self)
                && crate_dir.exists(self) == Ok(true)
            {
                collect_probable_modules_aux(
                    self,
                    root_module_path,
                    crate_dir.data(),
                    &mut module_paths,
                )
                .expect("what to do here?")
            }
        }
        module_paths
    }

    /// toolchain is
    /// - equal to the first live package's toolchain if live packages are not empty
    /// - equal to the toolchain found by iterating through config files under path's ancestry
    ///
    /// this will also update live packages
    fn resolve_module_path_and_update_live_packages(&self, path: &Path) -> VfsResult<ModulePath> {
        let toolchain = match self.live_packages().unwrap().first() {
            Some(package_path) => package_path.toolchain(self),
            None => crate::toolchain_config::toolchain_config(path, self).toolchain(),
        };
        let module_path = resolve_module_path(self, toolchain, path)?;
        let package_path = module_path.package_path(self);
        self.vfs_cache().add_live_package(package_path);
        Ok(module_path)
    }

    fn published_toolchain_library_path(&self, toolchain: PublishedToolchain) -> &Path {
        published_toolchain_library_path(self, toolchain)
    }
}

fn read_file_content(path: &Path) -> FileContent {
    if !path.exists() {
        FileContent::NotExists
    } else if path.is_file() {
        match std::fs::read_to_string(path) {
            Ok(text) => FileContent::OnDisk(text),
            Err(e) => FileContent::Err(VfsError::new_io_error(path.to_owned(), e)),
        }
    } else if path.is_dir() {
        // ad hoc
        FileContent::Directory(vec![])
    } else {
        todo!()
    }
}

#[salsa::jar]
pub struct VfsJar(
    VfsCache,
    crate::path::workspace_path::WorkspacePath,
    crate::linktime_target_path::LinktimeTargetPath,
    crate::linktime_target_path::linktime_target_rust_abs_dir,
    crate::path::package_path::PackagePath,
    crate::path::package_path::is_package_path_virtual,
    crate::path::package_path::package_task_crate_path,
    crate::path::crate_path::package_crate_paths,
    crate::path::module_path::relative_path::module_relative_dir_for_submodules,
    crate::path::module_path::relative_path::module_relative_stem,
    crate::path::crate_path::relative_path::crate_relative_dir_for_submodules,
    crate::path::linktime_target_path::linktime_target_rust_workspace_rustfmt_toml_path,
    crate::path::linktime_target_path::linktime_target_rust_workspace_manifest_path,
    CratePath,
    ModulePath,
    module_ancestry,
    vfs_path_menu,
    File,
    package_dir,
    package_manifest_path,
    module_virtual_path,
    crate::toolchain::Toolchain,
    crate::toolchain::PublishedToolchain,
    crate::toolchain::published_toolchain_library_path,
    crate::script::Script,
    crate::script::chunk_toolchain,
);

impl salsa::storage::IngredientsFor for VfsCache {
    type Jar = VfsJar;

    type Ingredients = Self;

    fn create_ingredients(_routes: &mut salsa::routes::Routes) -> Self::Ingredients {
        Default::default()
    }
}

impl VfsJar {
    pub(crate) fn cache(&self) -> &VfsCache {
        &self.0
    }

    // pub(crate) fn set_watcher(&mut self, watcher: VfsWatcher) {
    //     self.0.set_watcher(watcher)
    // }
}
