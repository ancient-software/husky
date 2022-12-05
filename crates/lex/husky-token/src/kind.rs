mod decorator;
mod keyword;
mod special;
mod wordopr;

pub use decorator::*;
pub use keyword::*;
pub use special::*;
pub use wordopr::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Decorator(Decorator),
    Keyword(Keyword),
    Identifier(Identifier),
    Special(SpecialToken),
    WordOpr(WordOpr),
    Literal(RawLiteralData),
    Unrecognized(char),
    IllFormedLiteral(RawLiteralData),
    Comment,
}

impl std::hash::Hash for TokenKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
