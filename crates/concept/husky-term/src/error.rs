use std::sync::Arc;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum TermError {
    #[error("term is not type")]
    TermIsNotTy,
    #[error("universe overflows")]
    UniverseOverflow,
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}

pub type TermResult<T> = Result<T, TermError>;
pub type TermResultArc<T> = Result<Arc<T>, TermError>;
