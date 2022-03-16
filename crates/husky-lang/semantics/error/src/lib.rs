use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
}

pub type SemanticResult<T> = Result<T, SemanticError>;

pub type SemanticResultArc<T> = Result<Arc<T>, SemanticError>;

impl From<ScopeError> for SemanticError {
    fn from(error: ScopeError) -> Self {
        Self {
            message: format!("ScopeError {:?}", error),
        }
    }
}

impl From<&ast::AstError> for SemanticError {
    fn from(error: &ast::AstError) -> Self {
        Self {
            message: format!("AstError {:?}", error),
        }
    }
}

impl From<VMError> for SemanticError {
    fn from(_: VMError) -> Self {
        todo!()
    }
}

#[macro_export]
macro_rules! err {
    ($msg:expr) => {{
        Err(SemanticError {
            message: $msg.into(),
        })?
    }};
}

#[macro_export]
macro_rules! not_none {
    ($option:expr) => {{
        $option.ok_or(SemanticError {
            message: "expect not none".into(),
        })?
    }};
}

use scope_query::ScopeError;
use vm::VMError;
