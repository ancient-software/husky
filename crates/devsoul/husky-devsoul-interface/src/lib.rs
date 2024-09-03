#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
pub mod devsoul;
#[cfg(feature = "ugly")]
pub mod ugly;

pub use husky_devsoul_interface_macros::*;

use husky_value::IsValue;
use once_cell::sync::OnceCell;
use shifted_unsigned_int::ShiftedU32;
use std::convert::Infallible;
