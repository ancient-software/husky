use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum DiskCacheError {
    #[error("io error: {0}")]
    Io(PathBuf, io::Error),
    #[error("cache file is locked by another process")]
    CacheFileLockedByAnotherProcess,
}

pub type DiskCacheResult<T> = Result<T, DiskCacheError>;
