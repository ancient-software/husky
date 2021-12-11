use common::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Special {
    LessOrLAngular,
    Leq,
    GreaterOrRAngular,
    Geq,
    Neq,
    Eq,
    LShift,
    RShift,
    LCurl,
    RCurl,
    LBox,
    RBox,
    LPar,
    RPar,
    Add,
    Sub,
    Mult,
    Div,
    And,
    Or,
    BitNot,
    Modulo,
    MemberAccess,
    HeavyArrow,  // =>
    ScopeAccess, // ::
    Colon,       // :
    Comma,       // ,
    Ambersand,   // &
    Incr,        // ++
    Decr,        // --
    Vertical,    // |
    Be,          // =
    AddAssign,
    SubAssign,
    MultAssign,
    DivAssign,
    NotOrExclusive,
}
