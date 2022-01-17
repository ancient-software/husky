use core::hash::Hash;
use std::{borrow::Borrow, fmt::Display, ops::Deref};

use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Identifier {
    Builtin(ReservedIdentifier),
    Custom(CustomIdentifier),
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Identifier::Builtin(ident) => ident.deref(),
            Identifier::Custom(ident) => ident.deref(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct CustomIdentifier(pub(crate) &'static str);

impl Debug for CustomIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl Display for CustomIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(self.0)
    }
}

impl PartialEq for CustomIdentifier {
    fn eq(&self, other: &Self) -> bool {
        (self.0 as *const str) == (other.0 as *const str)
    }
}

impl Eq for CustomIdentifier {}

impl Hash for CustomIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const str).hash(state);
    }
}

impl Deref for CustomIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Borrow<str> for CustomIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl From<CustomIdentifier> for Identifier {
    fn from(ident: CustomIdentifier) -> Self {
        Self::Custom(ident)
    }
}

impl From<&CustomIdentifier> for Identifier {
    fn from(ident: &CustomIdentifier) -> Self {
        Self::Custom(*ident)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ReservedIdentifier {
    Void,
    I32,
    F32,
    B32,
    B64,
    Bool,
    Vector,
    Tuple,
    Debug,
    Std,
    Core,
    Fp,
    Fn,
    FnMut,
    FnOnce,
    Array,
    Input,
    Dataset,
}

impl Deref for ReservedIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            ReservedIdentifier::Void => "()",
            ReservedIdentifier::I32 => "i32",
            ReservedIdentifier::F32 => "f32",
            ReservedIdentifier::B32 => "b32",
            ReservedIdentifier::B64 => "b64",
            ReservedIdentifier::Bool => "bool",
            ReservedIdentifier::Vector => "Vec",
            ReservedIdentifier::Tuple => "Tuple",
            ReservedIdentifier::Debug => "debug",
            ReservedIdentifier::Std => "std",
            ReservedIdentifier::Core => "core",
            ReservedIdentifier::Fp => "Fp",
            ReservedIdentifier::Fn => "Fn",
            ReservedIdentifier::FnMut => "FnMut",
            ReservedIdentifier::FnOnce => "FnOnce",
            ReservedIdentifier::Array => "Array",
            ReservedIdentifier::Input => "Input",
            ReservedIdentifier::Dataset => "builtin_dataset",
        }
    }
}

impl Borrow<str> for ReservedIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

pub fn default_func_type() -> ReservedIdentifier {
    ReservedIdentifier::Fp
}
