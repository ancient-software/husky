use crate::*;
// use husky_opr::{
//     BinaryClosedOpr, BinaryComparisonOpr, SynBinaryOpr, BinaryShiftOpr, BinaryShortcuitLogicOpr,
//     Bracket, SynSuffixOpr,
// };
use husky_opr::{BinaryClosedOpr, BinaryComparisonOpr, BinaryShiftOpr, BinaryShortcuitLogicOpr};
use husky_syn_opr::{SynBinaryOpr, SynSuffixOpr};

use self::delimiter::Delimiter;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Punctuation(PunctuationMapped);

impl Punctuation {
    pub fn mapped(self) -> PunctuationMapped {
        self.0
    }

    pub fn code(self) -> &'static str {
        self.0.code()
    }

    pub fn right_convexity(self) -> Convexity {
        match self.0 {
            PunctuationMapped::Binary(_) => Convexity::Concave,
            PunctuationMapped::Suffix(_) => Convexity::Convex,
            PunctuationMapped::Bra(_) => Convexity::Concave,
            PunctuationMapped::Ket(_) => Convexity::Convex,
            PunctuationMapped::LaOrLt => Convexity::Concave,
            PunctuationMapped::ColonColonLa => Convexity::Concave,
            PunctuationMapped::RaOrGt => Convexity::Any,
            PunctuationMapped::Shr => Convexity::Concave,
            PunctuationMapped::DeriveAssign => Convexity::Concave,
            PunctuationMapped::Minus => Convexity::Concave,
            PunctuationMapped::DoubleVertical => Convexity::Concave,
            PunctuationMapped::Tilde => Convexity::Concave,
            PunctuationMapped::Dot => Convexity::Concave,
            PunctuationMapped::Colon => Convexity::Any,
            PunctuationMapped::Comma => Convexity::Concave,
            PunctuationMapped::Vertical => Convexity::Any,
            PunctuationMapped::Exclamation => Convexity::Concave,
            PunctuationMapped::DoubleExclamation => Convexity::Concave,
            PunctuationMapped::Semicolon => Convexity::Any,
            PunctuationMapped::EmptyHtmlKet => Convexity::Convex,
            PunctuationMapped::At => Convexity::Concave,
            PunctuationMapped::AtEq => Convexity::Concave,
            PunctuationMapped::Ambersand => Convexity::Concave,
            PunctuationMapped::Pound => Convexity::Concave,
            PunctuationMapped::Question => Convexity::Any,
            PunctuationMapped::DotDot => Convexity::Concave,
            PunctuationMapped::Star => Convexity::Concave,
            PunctuationMapped::Sheba => Convexity::Any,
            PunctuationMapped::Eq => Convexity::Concave,
            PunctuationMapped::ForAll => Convexity::Concave,
            PunctuationMapped::Exists => Convexity::Concave,
            PunctuationMapped::HeavyArrow => Convexity::Concave,
            _ => unreachable!(),
        }
    }

    /// `->`
    pub const LIGHT_ARROW: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::CurryType));
    /// `=>`
    pub const HEAVY_ARROW: Self = Self(PunctuationMapped::HeavyArrow);
    /// `:`
    pub const COLON: Self = Self(PunctuationMapped::Colon);
    /// `::`
    pub const COLON_COLON: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::ScopeResolution));
    /// `;`
    pub const SEMICOLON: Self = Self(PunctuationMapped::Semicolon);
    /// `(`
    pub const LPAR: Self = Self(PunctuationMapped::Bra(Delimiter::Par));
    /// `[`
    pub const LBOX: Self = Self(PunctuationMapped::Bra(Delimiter::Box));
    /// `{`
    pub const NESTED_LCURL: Self = Self(PunctuationMapped::Bra(Delimiter::NestedCurl));
    /// `{`
    pub const INLINE_LCURL: Self = Self(PunctuationMapped::Bra(Delimiter::InlineCurl));
    /// `<`
    pub const LA_OR_LT: Self = Self(PunctuationMapped::LaOrLt);
    /// `::<`
    pub const COLON_COLON_LA: Self = Self(PunctuationMapped::ColonColonLa);
    /// `)`
    pub const RPAR: Self = Self(PunctuationMapped::Ket(Delimiter::Par));
    /// `]`
    pub const RBOX: Self = Self(PunctuationMapped::Ket(Delimiter::Box));
    /// `}`
    pub const NESTED_RCURL: Self = Self(PunctuationMapped::Ket(Delimiter::NestedCurl));
    /// `}`
    pub const INLINE_RCURL: Self = Self(PunctuationMapped::Ket(Delimiter::InlineCurl));
    /// `>`
    pub const RA_OR_GT: Self = Self(PunctuationMapped::RaOrGt);
    /// `|`
    pub const VERTICAL: Self = Self(PunctuationMapped::Vertical);
    /// `||`
    pub const DOUBLE_VERTICAL: Self = Self(PunctuationMapped::DoubleVertical);
    /// `..`
    pub const DOT_DOT: Self = Self(PunctuationMapped::DotDot);
    /// `...`
    pub const DOT_DOT_DOT: Self = Self(PunctuationMapped::DotDotDot);
    /// `~`
    pub const TILDE: Self = Self(PunctuationMapped::Tilde);
    /// `%`
    pub const REM_EUCLID: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Closed(
        BinaryClosedOpr::RemEuclid,
    )));
    pub const AMBERSAND: Self = Self(PunctuationMapped::Ambersand);
    /// `.`
    pub const DOT: Self = Self(PunctuationMapped::Dot);

    pub const ADD: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Closed(
        BinaryClosedOpr::Add,
    )));
    pub const SUB: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Closed(
        BinaryClosedOpr::Sub,
    )));
    pub const DIV: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Closed(
        BinaryClosedOpr::Div,
    )));
    pub const MINUS: Self = Self(PunctuationMapped::Minus);
    pub const STAR_STAR: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Closed(
        BinaryClosedOpr::Power,
    )));
    /// `=`
    pub const EQ: Self = Self(PunctuationMapped::Eq);
    pub const ADD_ASSIGN: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::AssignClosed(
        BinaryClosedOpr::Add,
    )));
    pub const SUB_ASSIGN: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::AssignClosed(
        BinaryClosedOpr::Sub,
    )));
    /// `*=`
    pub const MUL_ASSIGN: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::AssignClosed(
        BinaryClosedOpr::Mul,
    )));
    /// `*=`
    pub const BIT_AND_ASSIGN: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::AssignClosed(
        BinaryClosedOpr::BitAnd,
    )));
    pub const BIT_OR_ASSIGN: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::AssignClosed(
        BinaryClosedOpr::BitOr,
    )));
    /// `>>`
    pub const SHL: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Shift(
        BinaryShiftOpr::Shl,
    )));
    /// `>>`
    pub const SHR: Self = Self(PunctuationMapped::Shr);
    /// `/=`
    pub const DIV_ASSIGN: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::AssignClosed(
        BinaryClosedOpr::Div,
    )));
    /// `==`
    pub const EQ_EQ: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Comparison(
        BinaryComparisonOpr::Eq,
    )));
    /// `!=`
    pub const NEQ: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Comparison(
        BinaryComparisonOpr::Neq,
    )));
    /// `>=`
    pub const LEQ: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Comparison(
        BinaryComparisonOpr::Leq,
    )));
    /// `>=`
    pub const GEQ: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::Comparison(
        BinaryComparisonOpr::Geq,
    )));
    pub const INCR: Self = Self(PunctuationMapped::Suffix(SynSuffixOpr::Incr));
    pub const DECR: Self = Self(PunctuationMapped::Suffix(SynSuffixOpr::Decr));
    /// `!`
    pub const EXCLAMATION: Self = Self(PunctuationMapped::Exclamation);
    /// `!!`
    pub const DOUBLE_EXCLAMATION: Self = Self(PunctuationMapped::DoubleExclamation);
    /// `?`
    pub const QUESTION: Self = Self(PunctuationMapped::Question);
    /// `#`
    pub const POUND: Self = Self(PunctuationMapped::Pound);
    /// `∀`
    pub const FOR_ALL: Self = Self(PunctuationMapped::ForAll);
    /// `∃`
    pub const EXISTS: Self = Self(PunctuationMapped::Exists);
    /// `:=`
    pub const COLON_EQ: Self = Self(PunctuationMapped::DeriveAssign);
    /// `,`
    pub const COMMA: Self = Self(PunctuationMapped::Comma);
    /// `@=`
    pub const AT_EQ: Self = Self(PunctuationMapped::AtEq);
    /// `@`
    pub const AT: Self = Self(PunctuationMapped::At);
    /// `$`
    pub const SHEBA: Self = Self(PunctuationMapped::Sheba);
    /// `&&`
    pub const LOGIC_AND: Self = Self(PunctuationMapped::Binary(SynBinaryOpr::ShortCircuitLogic(
        BinaryShortcuitLogicOpr::And,
    )));
    /// `*`
    pub const STAR: Self = Self(PunctuationMapped::Star);
    /// `/>`
    pub const EMPTY_HTML_KET: Self = Self(PunctuationMapped::EmptyHtmlKet);
}

impl std::fmt::Display for Punctuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

/// serve as cached
/// for punctuation that unambiguously maps to an opr,
/// we use that opr to represent it
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PunctuationMapped {
    // predetermined
    Binary(SynBinaryOpr),
    Bra(Delimiter),
    Ket(Delimiter),
    Suffix(SynSuffixOpr),
    /// `=`
    ///
    /// assignment, attributes, various type definitions
    Eq,
    /// `<`
    LaOrLt, // <
    ColonColonLa, // ::<
    /// `>`, represents one of several cases:
    /// 1) right angle bracket
    /// 2) greater than
    RaOrGt,
    /// `>>` shift right
    Shr,
    DeriveAssign,      // :=
    Minus,             // -
    DoubleVertical,    // ||
    Dot,               // `.`
    DotDot,            // `..`
    DotDotDot,         // `...`
    Colon,             // `:`
    Star,              // `*`
    Comma,             // `,`
    Ambersand,         // `&`
    Vertical,          // `|`
    Exclamation,       // `!`
    DoubleExclamation, // `!!`
    Semicolon,         // `;`
    EmptyHtmlKet,      // `/>`
    Sheba,             // $
    /// `@`
    At,
    /// `@=`
    AtEq,
    /// `?`
    Question,
    /// written as `#`
    Pound,
    /// `~`
    Tilde,
    /// `∀`
    ForAll,
    /// `∃`
    Exists,
    /// `=>`
    HeavyArrow,
}

impl PunctuationMapped {
    pub fn code(self) -> &'static str {
        match self {
            PunctuationMapped::Binary(opr) => opr.code(),
            PunctuationMapped::Bra(bra) => bra.bra_code(),
            PunctuationMapped::Ket(ket) => ket.ket_code(),
            PunctuationMapped::Suffix(opr) => opr.code(),
            PunctuationMapped::LaOrLt => "<",
            PunctuationMapped::ColonColonLa => "::<",
            PunctuationMapped::RaOrGt => ">",
            PunctuationMapped::Shr => ">>",
            PunctuationMapped::DeriveAssign => ":=",
            PunctuationMapped::Minus => "-",
            PunctuationMapped::DoubleVertical => "||",
            PunctuationMapped::Tilde => "~",
            PunctuationMapped::Dot => ".",
            PunctuationMapped::DotDot => "..",
            PunctuationMapped::DotDotDot => "...",
            PunctuationMapped::Colon => ":",
            PunctuationMapped::Comma => ",",
            PunctuationMapped::Ambersand => "&",
            PunctuationMapped::Vertical => "|",
            PunctuationMapped::Exclamation => "!",
            PunctuationMapped::DoubleExclamation => "!!",
            PunctuationMapped::Semicolon => ";",
            PunctuationMapped::EmptyHtmlKet => "/>",
            PunctuationMapped::At => "@",
            PunctuationMapped::AtEq => "@=",
            PunctuationMapped::Question => "?",
            PunctuationMapped::Pound => "#",
            PunctuationMapped::Star => "*",
            PunctuationMapped::Sheba => "#",
            PunctuationMapped::Eq => "=",
            PunctuationMapped::ForAll => "∀",
            PunctuationMapped::Exists => "∃",
            PunctuationMapped::HeavyArrow => "=>",
        }
    }

    pub fn opt_bra(self) -> Option<Delimiter> {
        match self {
            PunctuationMapped::LaOrLt => Some(Delimiter::TurboFish),
            PunctuationMapped::Bra(bracket) => Some(bracket),
            _ => None,
        }
    }
}
