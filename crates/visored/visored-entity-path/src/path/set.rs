use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VdSetPath {
    Prelude(VdPreludeSetPath),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VdPreludeSetPath {
    NaturalNumber,
    RationalNumber,
    Integer,
    RealNumber,
    ComplexNumber,
}

impl VdSetPath {
    pub const NATURAL_NUMBER: Self = Self::Prelude(VdPreludeSetPath::NaturalNumber);
    pub const RATIONAL_NUMBER: Self = Self::Prelude(VdPreludeSetPath::RationalNumber);
    pub const INTEGER: Self = Self::Prelude(VdPreludeSetPath::Integer);
    pub const REAL_NUMBER: Self = Self::Prelude(VdPreludeSetPath::RealNumber);
    pub const COMPLEX_NUMBER: Self = Self::Prelude(VdPreludeSetPath::ComplexNumber);
}

impl VdSetPath {
    pub fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdSetPath::Prelude(path) => path.show_fmt(f),
        }
    }
}

impl VdPreludeSetPath {
    fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdPreludeSetPath::NaturalNumber => write!(f, "ℕ"),
            VdPreludeSetPath::RationalNumber => write!(f, "ℚ"),
            VdPreludeSetPath::Integer => write!(f, "ℤ"),
            VdPreludeSetPath::RealNumber => write!(f, "ℝ"),
            VdPreludeSetPath::ComplexNumber => write!(f, "ℂ"),
        }
    }
}
