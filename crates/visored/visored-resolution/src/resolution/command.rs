use super::*;
use latex_command::path::LxCommandPath;
use latex_math_letter::LxMathLetter;
use rustc_hash::FxHashMap;
use visored_item_path::VdItemPath;
use visored_opr::opr::VdBaseOpr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdCommandResolution {
    Letter(LxMathLetter),
    Item(VdItemPath),
    Opr(VdBaseOpr),
    Frac,
    Sqrt,
    Text,
    Todo,
}

pub type VdCommandResolutionMap = FxHashMap<LxCommandPath, VdCommandResolution>;

impl VdCommandResolution {
    pub const INT: Self = Self::Opr(VdBaseOpr::INTEGRAL);
    pub const SUM: Self = Self::Opr(VdBaseOpr::SUM);
    pub const PROD: Self = Self::Opr(VdBaseOpr::PROD);
    pub const TIMES: Self = Self::Opr(VdBaseOpr::TIMES);
    pub const OTIMES: Self = Self::Opr(VdBaseOpr::OTIMES);
    pub const ALPHA: Self = Self::Letter(LxMathLetter::LowerAlpha);
    pub const BETA: Self = Self::Letter(LxMathLetter::LowerBeta);
    pub const GAMMA: Self = Self::Letter(LxMathLetter::LowerGamma);
    pub const PI: Self = Self::Letter(LxMathLetter::LowerPi);
}
