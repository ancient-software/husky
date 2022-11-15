use husky_dev_utils::DevSource;
use husky_text::TextRange;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LexError {
    pub message: String,
    pub range: TextRange,
    pub dev_src: DevSource,
}

pub type LexResult<T> = Result<T, LexError>;

pub type LexResultArc<T> = Result<Arc<T>, LexError>;
