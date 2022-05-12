mod allocate_unique;
mod error;
mod line_map;
mod query;
mod utils;

pub use allocate_unique::{
    new_file_unique_allocator, AllocateUniqueFile, FilePtr, UniqueFileAllocator,
};
pub use error::*;
pub use query::{FileContentQuery, FileQueryGroup, FileQueryStorage, FileSalsaQuery, LiveFiles};
pub use utils::*;
pub type URange = std::ops::Range<usize>;

use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    Main,
    Lib,
    Module,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileContent {
    OnDisk(Arc<String>),
    Live(Arc<String>),
    Deleted,
    NonExistent,
}

impl FileContent {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            FileContent::OnDisk(s) => Some(&s),
            FileContent::Live(s) => Some(&s),
            FileContent::Deleted | FileContent::NonExistent => None,
        }
    }
}
