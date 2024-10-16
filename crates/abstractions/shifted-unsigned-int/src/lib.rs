#![feature(nonzero_ops)]
use std::{
    num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    ops::AddAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "usize", into = "usize"))]
pub struct ShiftedU8(NonZeroU8);

impl Default for ShiftedU8 {
    fn default() -> Self {
        0usize.into()
    }
}

impl From<usize> for ShiftedU8 {
    fn from(value: usize) -> Self {
        debug_assert!(value + 1 < u8::MAX as usize);
        ShiftedU8(unsafe { NonZeroU8::new_unchecked((value + 1) as u8) })
    }
}

impl Into<usize> for ShiftedU8 {
    fn into(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "usize", into = "usize"))]
pub struct ShiftedU16(NonZeroU16);

impl Default for ShiftedU16 {
    fn default() -> Self {
        0usize.into()
    }
}

impl From<usize> for ShiftedU16 {
    fn from(value: usize) -> Self {
        debug_assert!(value + 1 < u16::MAX as usize);
        ShiftedU16(unsafe { NonZeroU16::new_unchecked((value + 1) as u16) })
    }
}

impl Into<usize> for ShiftedU16 {
    fn into(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "usize", into = "usize"))]
pub struct ShiftedU32(NonZeroU32);

impl std::fmt::Debug for ShiftedU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.index().fmt(f)
    }
}

impl Default for ShiftedU32 {
    fn default() -> Self {
        0usize.into()
    }
}

impl ShiftedU32 {
    pub fn new(value: u32) -> Self {
        ShiftedU32(unsafe { NonZeroU32::new_unchecked(value + 1) })
    }

    pub unsafe fn unchecked_add(self, rhs: u32) -> Self {
        Self(self.0.unchecked_add(rhs))
    }
}

impl From<u32> for ShiftedU32 {
    fn from(value: u32) -> Self {
        debug_assert!(value + 1 < u32::MAX as u32);
        ShiftedU32(unsafe { NonZeroU32::new_unchecked((value + 1) as u32) })
    }
}

impl From<usize> for ShiftedU32 {
    fn from(value: usize) -> Self {
        debug_assert!(value + 1 < u32::MAX as usize);
        ShiftedU32(unsafe { NonZeroU32::new_unchecked((value + 1) as u32) })
    }
}

impl Into<u32> for ShiftedU32 {
    fn into(self) -> u32 {
        self.0.get() - 1
    }
}

impl Into<usize> for ShiftedU32 {
    fn into(self) -> usize {
        self.index()
    }
}

impl ShiftedU32 {
    pub fn index(self) -> usize {
        self.0.get() as usize - 1
    }
}

impl AddAssign<u32> for ShiftedU32 {
    fn add_assign(&mut self, rhs: u32) {
        self.0 = unsafe { NonZeroU32::new_unchecked(self.0.get() + rhs) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "usize", into = "usize"))]
pub struct ShiftedU64(NonZeroU64);

impl Default for ShiftedU64 {
    fn default() -> Self {
        0usize.into()
    }
}

impl From<usize> for ShiftedU64 {
    fn from(value: usize) -> Self {
        debug_assert!(value + 1 < u64::MAX as usize);
        ShiftedU64(unsafe { NonZeroU64::new_unchecked((value + 1) as u64) })
    }
}

impl Into<usize> for ShiftedU64 {
    fn into(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "usize", into = "usize"))]
pub struct ShiftedUsize(NonZeroUsize);

impl Default for ShiftedUsize {
    fn default() -> Self {
        0usize.into()
    }
}

impl From<usize> for ShiftedUsize {
    fn from(value: usize) -> Self {
        debug_assert!(value + 1 < usize::MAX as usize);
        ShiftedUsize(unsafe { NonZeroUsize::new_unchecked((value + 1) as usize) })
    }
}

impl Into<usize> for ShiftedUsize {
    fn into(self) -> usize {
        self.0.get() as usize - 1
    }
}
