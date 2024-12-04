use std::convert::Infallible;

use smallvec::SmallVec;

use crate::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum TomlLineGroup {
    SectionTitle {
        title: SmallVec<[BaseCoword; 2]>,
        kind: TomlSectionKind,
    },
    KeyValue(BaseCoword, Option<TomlExprIdx>),
    Comment,
    Err,
}

impl std::ops::FromResidual<Result<Infallible, TomlAstError>> for TomlLineGroup {
    fn from_residual(residual: Result<Infallible, TomlAstError>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(_e) => TomlLineGroup::Err,
        }
    }
}
