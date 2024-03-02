#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
mod db;
mod error;
mod line_group;
mod sheet;
mod special;
mod stream;
#[cfg(test)]
mod tests;
mod tokenize;

pub use self::db::*;
pub use self::error::*;
pub use self::sheet::*;
pub use self::special::*;
pub use self::stream::*;

use husky_coword::Coword;
use husky_text_span::DocumentSpan;
use std::char;
use std::str;
use std::sync::Arc;

use husky_text_protocol::range::TextRange;
use tokenize::*;

#[salsa::jar]
pub struct TomlTokenJar(toml_token_sheet);

pub type StringValue = Arc<String>;

/// tokens in toml file
#[derive(Debug, PartialEq, Eq)]
pub struct TomlToken {
    span: DocumentSpan,
    range: TextRange,
    data: TomlTokenData,
}

impl TomlToken {
    pub fn new(span: DocumentSpan, range: TextRange, data: TomlTokenData) -> Self {
        Self { span, range, data }
    }

    pub fn span(&self) -> DocumentSpan {
        self.span
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn data(&self) -> &TomlTokenData {
        &self.data
    }
}

/// variants for tokens in toml file
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TomlTokenData {
    Comment,
    Special(TomlSpecialToken),
    Word(Coword),
    StringLiteral { val: StringValue, multiline: bool },
    Err(TomlTokenError),
}

impl TomlTokenData {
    pub fn describe(&self) -> &'static str {
        match *self {
            TomlTokenData::Word(_) => "a word",
            TomlTokenData::Comment => "a comment",
            TomlTokenData::Special(special) => special.describe(),
            TomlTokenData::StringLiteral { multiline, .. } => {
                if multiline {
                    "a multiline string"
                } else {
                    "a string"
                }
            }
            TomlTokenData::Err(_) => todo!(),
        }
    }
}

impl std::ops::FromResidual<Result<core::convert::Infallible, TomlTokenError>> for TomlTokenData {
    fn from_residual(residual: Result<core::convert::Infallible, TomlTokenError>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => TomlTokenData::Err(e),
        }
    }
}
