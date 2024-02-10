use std::fmt::{self, Debug, Formatter};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg};

use ecow::EcoString;

use crate::foundations::{cast, repr, Repr, Resolve, StyleChain, TypstValue};
use crate::layout::Abs;
use crate::text::TextElem;
use crate::util::{Numeric, Scalar};

/// A length that is relative to the font size.
///
/// `1em` is the same as the font size.
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LengthInEm(Scalar);

impl LengthInEm {
    /// The zero em length.
    pub const fn zero() -> Self {
        Self(Scalar::ZERO)
    }

    /// The font size.
    pub const fn one() -> Self {
        Self(Scalar::ONE)
    }

    /// Create a font-relative length.
    pub const fn new(em: f64) -> Self {
        Self(Scalar::new(em))
    }

    /// Create an em length from font units at the given units per em.
    pub fn from_units(units: impl Into<f64>, units_per_em: f64) -> Self {
        Self(Scalar::new(units.into() / units_per_em))
    }

    /// Create an em length from a length at the given font size.
    pub fn from_length(length: Abs, font_size: Abs) -> Self {
        let result = length / font_size;
        if result.is_finite() {
            Self(Scalar::new(result))
        } else {
            Self::zero()
        }
    }

    /// The number of em units.
    pub const fn get(self) -> f64 {
        (self.0).get()
    }

    /// The absolute value of this em length.
    pub fn abs(self) -> Self {
        Self::new(self.get().abs())
    }

    /// Convert to an absolute length at the given font size.
    pub fn at(self, font_size: Abs) -> Abs {
        let resolved = font_size * self.get();
        if resolved.is_finite() {
            resolved
        } else {
            Abs::zero()
        }
    }
}

impl Numeric for LengthInEm {
    fn zero() -> Self {
        Self::zero()
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }
}

impl Debug for LengthInEm {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}em", self.get())
    }
}

impl Repr for LengthInEm {
    fn repr(&self) -> EcoString {
        repr::format_float_with_unit(self.get(), "em")
    }
}

impl Neg for LengthInEm {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add for LengthInEm {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

sub_impl!(LengthInEm - LengthInEm -> LengthInEm);

impl Mul<f64> for LengthInEm {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.0 * other)
    }
}

impl Mul<LengthInEm> for f64 {
    type Output = LengthInEm;

    fn mul(self, other: LengthInEm) -> LengthInEm {
        other * self
    }
}

impl Div<f64> for LengthInEm {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self(self.0 / other)
    }
}

impl Div for LengthInEm {
    type Output = f64;

    fn div(self, other: Self) -> f64 {
        self.get() / other.get()
    }
}

assign_impl!(LengthInEm += LengthInEm);
assign_impl!(LengthInEm -= LengthInEm);
assign_impl!(LengthInEm *= f64);
assign_impl!(LengthInEm /= f64);

impl Sum for LengthInEm {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|s| s.0).sum())
    }
}

cast! {
     LengthInEm,
     self => TypstValue::Length(self.into()),
}

impl Resolve for LengthInEm {
    type Output = Abs;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        if self.is_zero() {
            Abs::zero()
        } else {
            self.at(TextElem::size_in(styles))
        }
    }
}
