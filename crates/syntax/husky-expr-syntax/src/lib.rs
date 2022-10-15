mod atom;
mod automata;
mod error;
mod opn;
mod parser;
mod precedence;
#[cfg(test)]
mod tests;
mod variant;

pub use atom::*;
pub use automata::*;
pub use error::*;
pub use variant::*;

use husky_opn_syntax::*;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_symbol_syntax::Symbol;
use husky_text::*;
use husky_token::{Token, TokenKind};
use husky_word::*;
use precedence::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawExpr {
    pub variant: RawExprVariant,
    pub range: TextRange,
}

impl TextRanged for RawExpr {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprVariant {
    Atom(RawAtom),
    Bracketed(RawExprIdx),
    Opn {
        opn_variant: RawOpnVariant,
        opds: RawExprRange,
    },
    Lambda(
        Vec<(RangedCustomIdentifier, Option<RawExprIdx>)>,
        RawExprIdx,
    ),
}

impl From<RawAtom> for RawExprVariant {
    fn from(atom: RawAtom) -> Self {
        RawExprVariant::Atom(atom)
    }
}

use ::idx_arena::{map::ArenaMap, ArenaIdx, ArenaRange, IdxArena};

pub type RawExprArena = IdxArena<RawExpr>;
pub type RawExprIdx = ArenaIdx<RawExpr>;
pub type RawExprRange = ArenaRange<RawExpr>;
pub type RawExprMap<V> = ArenaMap<RawExpr, V>;

impl RawExpr {
    fn new(variant: RawExprVariant, range: TextRange) -> Self {
        Self { variant, range }
    }
}
