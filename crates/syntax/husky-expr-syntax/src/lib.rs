mod atom;
mod automata;
mod opn;
mod parser;
mod precedence;
mod variant;

pub use atom::*;
pub use automata::*;
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprVariant {
    Atom(RawExprAtomVariant),
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

impl From<Symbol> for RawExprVariant {
    fn from(value: Symbol) -> Self {
        RawExprVariant::Atom(value.into())
    }
}

use ::arena::{map::ArenaMap, Arena, ArenaIdx, ArenaRange};

pub type RawExprArena = Arena<RawExpr>;
pub type RawExprIdx = ArenaIdx<RawExpr>;
pub type RawExprRange = ArenaRange<RawExpr>;

impl RawExpr {
    fn new(variant: RawExprVariant, range: TextRange) -> Self {
        Self { variant, range }
    }
}
