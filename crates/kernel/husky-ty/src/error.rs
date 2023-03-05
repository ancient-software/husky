use crate::*;
use husky_valid_ty::ValidTypeError;
use thiserror::Error;

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TypeError {
    #[error("original `{0}`")]
    Original(#[from] OriginalTypeError),
    #[error("derived `{0}`")]
    Derived(#[from] DerivedTypeError),
}

impl From<ValidTypeError> for TypeError {
    fn from(value: ValidTypeError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalTypeError {
    #[error("term error")]
    Term(#[from] TermError),
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DerivedTypeError {
    #[error("signature error")]
    SignatureError,
    #[error("declaration error")]
    DeclError,
}

impl<Db: TypeDb + ?Sized> salsa::DebugWithDb<Db> for TypeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}
