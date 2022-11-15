mod convexity;
mod error;
mod kind;
mod query;
mod raw_token_iter;
mod scanner;
mod semantic_token;
mod special;
#[cfg(test)]
mod tests;
mod tokenized_text;
mod utils;

pub use convexity::*;
pub use error::*;
pub use kind::TokenKind;
pub use query::*;
pub use semantic_token::*;
pub use special::SpecialToken;
pub use tokenized_text::{TokenGroupIter, TokenizedText};

use husky_opn_syntax::*;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_text::{RangedCustomIdentifier, TextIndent, TextRange, TextRanged};
use husky_word::Identifier;
use raw_token_iter::*;
use scanner::TokenScanner;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub range: TextRange,
    pub kind: TokenKind,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Token {
    pub fn new(i: usize, start: usize, end: usize, kind: TokenKind) -> Token {
        Token {
            range: husky_text::new_same_line(i, start, end),
            kind,
        }
    }

    pub fn ranged_custom_ident(&self) -> Option<RangedCustomIdentifier> {
        match self.kind {
            TokenKind::Identifier(Identifier::Custom(ident)) => Some(RangedCustomIdentifier {
                ident,
                range: self.range,
            }),
            _ => todo!(),
        }
    }

    pub fn left_convexity(&self) -> Option<Convexity> {
        self.kind.left_convexity()
    }

    pub fn right_convexity(&self) -> Convexity {
        self.kind.right_convexity()
    }
}

impl TextRanged for Token {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
