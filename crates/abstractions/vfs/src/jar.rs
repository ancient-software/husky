use crate::*;

use maybe_result::MaybeResult::JustOk;
use salsa::{Db, Durability};
use vec_like::VecSet;

pub trait VfsDb {}

// don't leak this outside the crate
pub trait VfsDbInner {
    fn file_from_virtual_path(&self, path: VirtualPath, durability: Durability) -> VfsResult<File>;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn vfs_db_mut(&mut self) -> &mut ::salsa::Db;
    fn vfs_cache(&self) -> &VfsCache;
    fn set_content(
        &mut self,
        path: &Path,
        content: FileContent,
        durability: Durability,
    ) -> VfsResult<()>;
    fn refresh_file_from_disk(&mut self, path: &Path, durability: Durability) -> VfsResult<()>
    where
        Self: 'static;
}

impl VfsDbInner for Db {
    fn file_from_virtual_path(
        &self,
        abs_path: VirtualPath,
        durability: Durability,
    ) -> VfsResult<File> {
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
                    *entry.insert(File::new(self, abs_path.clone(), content, durability))
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
    fn refresh_file_from_disk(&mut self, path: &Path, durability: Durability) -> VfsResult<()>
    where
        Db: 'static,
    {
        let content = read_file_content(&path);
        self.set_content(path, content, durability)
    }

    fn set_content(
        &mut self,
        path: &Path,
        content: FileContent,
        durability: Durability,
    ) -> VfsResult<()> {
        let virtual_path = VirtualPath::try_new(self, path)?;
        let path = virtual_path.data();
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
        file.set_content(self, durability)?.to(content);
        Ok(())
    }

    fn vfs_db_mut(&mut self) -> &mut ::salsa::Db {
        self
    }
}

impl VfsDb for Db {}

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
pub struct VfsJar(VfsCache, File);

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
