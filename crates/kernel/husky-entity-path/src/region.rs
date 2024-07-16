use crate::{path::ItemPath, *};
use husky_vfs::{
    path::{
        crate_path::CratePath,
        module_path::{ChunkModulePath, ModulePath},
    },
    toolchain::Toolchain,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
pub enum RegionPath {
    CrateDecl(CratePath),
    ItemDecl(ItemPath),
    ItemDefn(ItemPath),
    Chunk(ChunkModulePath),
}

impl RegionPath {
    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            RegionPath::CrateDecl(slf) => slf.root_module_path(db),
            RegionPath::ItemDecl(slf) => slf.module_path(db),
            RegionPath::ItemDefn(slf) => slf.module_path(db),
            RegionPath::Chunk(slf) => slf.module_path(),
        }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        match self {
            RegionPath::CrateDecl(slf) => slf.toolchain(db),
            RegionPath::ItemDecl(slf) => slf.toolchain(db),
            RegionPath::ItemDefn(slf) => slf.toolchain(db),
            RegionPath::Chunk(slf) => slf.toolchain(db),
        }
    }
}
