#[cfg(feature = "signal_support")]
use husky_signal::Signalable;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct SampleId(pub usize);

#[cfg(feature = "signal_support")]
impl Signalable for SampleId {}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Label(pub i32);

impl From<u8> for Label {
    fn from(v: u8) -> Self {
        Self(v as i32)
    }
}
