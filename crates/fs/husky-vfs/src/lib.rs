#![feature(iter_advance_by)]
#![allow(incomplete_features)]
#![feature(absolute_path)]
#![feature(let_chains)]
mod cache;
pub mod db;
pub mod error;
mod file;
#[cfg(feature = "lsp_support")]
mod lsp_support;
pub mod path;
pub mod snippet;
#[cfg(feature = "test_utils")]
pub mod test_utils;
#[cfg(test)]
mod tests;
mod toolchain;
pub mod toolchain_config;
mod watch;

pub use self::cache::VfsCache;
#[cfg(feature = "lsp_support")]
pub use self::lsp_support::*;
pub use self::path::*;
#[cfg(feature = "test_utils")]
pub use self::test_utils::*;
pub use self::toolchain::*;
// pub use self::watch::{VfsWatcher, WatchableVfsDb, WatchedVfs};

use self::error::*;
use self::file::*;
use dashmap::{mapref::entry::Entry, DashMap};
use husky_coword::*;
use std::path::{Path, PathBuf};
#[cfg(test)]
use tests::*;
