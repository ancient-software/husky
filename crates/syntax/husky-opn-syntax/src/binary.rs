mod comparison;
mod logic;
mod pure_closed;

pub use comparison::*;
pub use logic::*;
pub use pure_closed::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOpr {
    PureClosed(BinaryPureClosedOpr),
    Comparison(BinaryComparisonOpr),
    ShortcuitLogic(BinaryShortcuitLogicOpr),
    Assign(Option<BinaryPureClosedOpr>),
    ScopeResolution, // ::
    Curry,           // ->
    As,              // as
}

impl Into<RawOpnVariant> for BinaryOpr {
    fn into(self) -> RawOpnVariant {
        RawOpnVariant::Binary(self)
    }
}

impl BinaryOpr {
    pub fn code(self) -> &'static str {
        match self {
            BinaryOpr::PureClosed(pure_opr) => pure_opr.husky_code(),
            BinaryOpr::Assign(None) => "=",
            BinaryOpr::Assign(Some(pure_opr)) => match pure_opr {
                BinaryPureClosedOpr::Add => "+=",
                BinaryPureClosedOpr::BitAnd => "&=",
                BinaryPureClosedOpr::BitOr => "|=",
                BinaryPureClosedOpr::BitXor => "^=",
                BinaryPureClosedOpr::Div => "/=",
                BinaryPureClosedOpr::Mul => "*=",
                BinaryPureClosedOpr::RemEuclid => "%=",
                BinaryPureClosedOpr::Power => "**=",
                BinaryPureClosedOpr::Shl => "<<=",
                BinaryPureClosedOpr::Shr => ">>=",
                BinaryPureClosedOpr::Sub => "-=",
            },
            BinaryOpr::Comparison(cmp_opr) => cmp_opr.husky_code(),
            BinaryOpr::ShortcuitLogic(logic_opr) => logic_opr.husky_code(),
            BinaryOpr::ScopeResolution => "::",
            BinaryOpr::Curry => "->",
            BinaryOpr::As => todo!(),
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            BinaryOpr::PureClosed(pure_binary_opr) => pure_binary_opr.spaced_husky_code(),
            BinaryOpr::Comparison(cmp_opr) => cmp_opr.spaced_husky_code(),
            BinaryOpr::ShortcuitLogic(logic_opr) => logic_opr.spaced_husky_code(),
            BinaryOpr::Assign(opt_binary_opr) => {
                if let Some(binary_opr) = opt_binary_opr {
                    match binary_opr {
                        BinaryPureClosedOpr::Add => " += ",
                        BinaryPureClosedOpr::BitAnd => " &= ",
                        BinaryPureClosedOpr::BitOr => " |= ",
                        BinaryPureClosedOpr::BitXor => " ^= ",
                        BinaryPureClosedOpr::Div => " /= ",
                        BinaryPureClosedOpr::Mul => " *= ",
                        BinaryPureClosedOpr::RemEuclid => " %= ",
                        BinaryPureClosedOpr::Power => " **= ",
                        BinaryPureClosedOpr::Shl => " <<= ",
                        BinaryPureClosedOpr::Shr => " >>= ",
                        BinaryPureClosedOpr::Sub => " -= ",
                    }
                } else {
                    " = "
                }
            }
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => " -> ",
            BinaryOpr::As => todo!(),
        }
    }
}
