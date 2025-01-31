use visored_opr::{precedence::VdPrecedence, separator::VdSeparatorClass};

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum VdMirBasePrefixOpr {
    RingPos,
    RingNeg,
}

impl VdMirBasePrefixOpr {
    pub const RING_POS: Self = VdMirBasePrefixOpr::RingPos;
    pub const RING_NEG: Self = VdMirBasePrefixOpr::RingNeg;
}

impl VdMirBasePrefixOpr {
    pub fn precedence(self) -> VdPrecedence {
        match self {
            VdMirBasePrefixOpr::RING_POS | VdMirBasePrefixOpr::RING_NEG => VdPrecedence::SIGN,
            _ => todo!(),
        }
    }
}
