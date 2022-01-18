pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[
        ("dataset1", DATASET1_SCOPE_DATA),
        ("dataset2", DATASET2_SCOPE_DATA),
    ],
    call_signature: None,
    compiled: None,
};

pub const DATASET1_SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Func,
    subscopes: &[],
    call_signature: Some(CallSignature {
        inputs: vec![],
        output: ScopeId::Builtin(ReservedIdentifier::DatasetType),
    }),
    compiled: Some(|stack| stack.push(VirtualStackValue::Owned(Box::new(dataset1())))),
};

pub const DATASET2_SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Func,
    subscopes: &[],
    call_signature: Some(CallSignature {
        inputs: vec![],
        output: ScopeId::Builtin(ReservedIdentifier::DatasetType),
    }),
    compiled: None,
};

use scope::{CallSignature, ScopeId};
use virtual_stack::VirtualStackValue;
use word::ReservedIdentifier;
use xrng::XRng;

use crate::{synthetic::SimpleSyntheticDataset, *};

#[derive(Debug, Clone)]
pub struct Real1dDatapoint {
    pub x: f32,
    pub y: i32,
}

pub fn gen_sample1(idx: usize) -> Real1dDatapoint {
    let mut xrng = XRng::new(((idx >> 32) & (idx << 32)) as u64);
    if xrng.with_probability(0.5) {
        Real1dDatapoint { x: 1.0, y: 1 }
    } else {
        Real1dDatapoint { x: -1.0, y: 0 }
    }
}

pub fn gen_sample2(idx: usize) -> Real1dDatapoint {
    let mut xrng = XRng::new(((idx >> 32) & (idx << 32)) as u64);
    if xrng.with_probability(0.5) {
        Real1dDatapoint { x: 1.0, y: 1 }
    } else {
        Real1dDatapoint { x: -1.0, y: 0 }
    }
}

pub fn dataset1() -> SimpleSyntheticDataset<Real1dDatapoint> {
    SimpleSyntheticDataset::new(gen_sample1)
}

pub fn dataset2() -> SimpleSyntheticDataset<Real1dDatapoint> {
    SimpleSyntheticDataset::new(gen_sample2)
}
