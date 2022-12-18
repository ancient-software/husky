pub(crate) use husky_vfs_test_utils::*;

use crate::*;
use expect_test::expect_file;
use husky_absolute_path::AbsolutePath;
use husky_ast::AstJar;
use husky_entity_path::{EntityPathData, EntityPathDb, EntityPathJar};
use husky_package_path::{CrateKind, PackagePathData, PackagePathDb, PackagePathJar};
use husky_token::TokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::*;
use husky_word::{WordDb, WordJar};
use salsa::{Database, DebugWithDb, ParallelDatabase, Snapshot};
use std::{borrow::Cow, sync::Arc};

#[salsa::db(
    WordJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    VfsJar,
    TokenJar,
    AstJar,
    EntityTreeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
    source_path_config: VfsConfigMimic,
}

impl HasVfsConfig for DB {
    fn vfs_config(&self) -> &VfsConfig {
        &self.source_path_config
    }
}

impl salsa::Database for DB {}
