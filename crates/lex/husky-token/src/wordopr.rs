use std::ops::Deref;

use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WordOpr {
    And,
    Or,
    As,
    Be,
}

impl const From<WordOpr> for TokenKind {
    fn from(wordopr: WordOpr) -> Self {
        TokenKind::WordOpr(wordopr)
    }
}

impl Deref for WordOpr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl WordOpr {
    pub const fn as_str(&self) -> &'static str {
        match self {
            WordOpr::And => "and",
            WordOpr::Or => "or",
            WordOpr::As => "as",
            WordOpr::Be => "be",
        }
    }
}
