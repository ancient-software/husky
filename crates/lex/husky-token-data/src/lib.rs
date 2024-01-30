mod convexity;
pub mod db;
mod error;
mod keyword;
mod literal;
mod punctuation;
mod wordopr;

pub use self::convexity::*;
pub use self::error::*;
pub use self::keyword::*;
pub use self::literal::*;
pub use self::punctuation::*;
pub use self::wordopr::*;

use self::db::*;
use husky_coword::*;

#[cfg(feature = "protocol_support")]
use husky_token_protocol::TokenClass;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TokenData {
    Keyword(Keyword),
    Ident(Ident),
    Label(Label),
    Punctuation(Punctuation),
    WordOpr(WordOpr),
    Literal(LiteralData),
    Error(TokenDataError),
}

#[cfg(feature = "protocol_support")]
impl TokenData {
    // overridable given more information
    pub fn default_token_class(self) -> TokenClass {
        match self {
            TokenData::Keyword(kw) => kw.class().into(),
            TokenData::Punctuation(_) => TokenClass::Punctuation,
            TokenData::WordOpr(_) => TokenClass::WordOpr,
            TokenData::Literal(_) => TokenClass::Literal,
            TokenData::Ident(_) => TokenClass::Ident,
            TokenData::Label(_) => TokenClass::Label,
            TokenData::Error(_) => TokenClass::Error,
        }
    }
}

impl std::hash::Hash for TokenData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
