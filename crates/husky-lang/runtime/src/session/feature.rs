use common::*;
use semantics::Opn;
use stdx::sync::ARwLock;
use syntax_types::PrimitiveValue;
use vm::{BinaryOpr, Compiled};
use word::CustomIdentifier;

use super::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Feature {
    Input,
    Literal(PrimitiveValue),
    Assert {
        condition: FeatureId,
    },
    Do {
        first: FeatureId,
        then: FeatureId,
    },
    Cached(FeatureId),
    PrimitiveBinaryFunc {
        func: BinaryOpr,
        lopd: FeatureId,
        ropd: FeatureId,
    },
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct FeatureId {
    pub(super) raw: usize,
    pub(super) cached: bool,
}
