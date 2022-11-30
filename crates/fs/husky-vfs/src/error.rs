use std::path::{Path, PathBuf};

use husky_print_utils::p;
use husky_source_path::SourcePathError;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum VfsError {
    #[error("file {0:?} not found")]
    FileNotFound(PathBuf),
    #[error("IO Error: ???")]
    IO {
        path: PathBuf,
        error_message: String,
    },
    #[error("source path error {0}")]
    SourcePath(#[from] SourcePathError),
}

pub type VfsResult<T> = Result<T, VfsError>;

// impl From<std::io::Error> for VfsError {
//     fn from(value: std::io::Error) -> Self {
//         p!(value);
//         todo!()
//     }
// }

impl VfsError {
    pub(crate) fn new_io_error(path: PathBuf, e: std::io::Error) -> VfsError {
        VfsError::IO {
            path,
            error_message: e.to_string(),
        }
    }
}
