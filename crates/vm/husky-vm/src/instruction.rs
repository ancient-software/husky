mod condition_flow;
mod id;
mod opn;
mod pattern_match;
mod sheet;

pub use condition_flow::*;
pub use id::{InstructionId, InstructionSource};
pub use opn::*;
pub use pattern_match::*;
pub use sheet::Instructions;

use crate::*;
use husky_coword::Ident;
use husky_text_protocol::range::TextRange;
use std::{ops::Deref, panic::RefUnwindSafe, sync::Arc};

#[derive(Debug)]
pub struct Instruction {
    pub data: InstructionData,
    pub src: InstructionSource,
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.ins_id() == other.ins_id()
    }
}

impl Eq for Instruction {}

impl Instruction {
    pub fn new(data: InstructionData, src: InstructionSource) -> Self {
        Self { data, src }
    }

    pub fn ins_id(&self) -> InstructionId {
        todo!()
        // self.src.instruction_id()
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionData {
    PushVariable {
        stack_idx: VMStackIdx,
        binding: Binding,
        range: TextRange,
        ty: EthTerm,
        varname: Ident,
        explicit: bool,
    },
    PushLiteralValue {
        // value: PrimitiveValue,
        ty: EthTerm,
        explicit: bool,
    },
    WrapInSome {
        number_of_somes: u8,
    },
    CallRoutine {
        resolved_linkage: __ResolvedLinkage,
        nargs: u8,
        return_ty: EthTerm,
        discard: bool,
    },
    CallInterpreted {
        routine_uid: EntityUid,
        nargs: u8,
        return_ty: EthTerm,
        discard: bool,
    },
    VirtualStructField {
        field_idx: u8,
        field_binding: Binding,
        field_ty: EthTerm,
    },
    NewVirtualStruct {
        ty: EthTerm,
        fields: Vec<Ident>,
    },
    Loop {
        body: Instructions,
        loop_kind: VMLoopKind,
    },
    Return {
        return_ty: EthTerm,
    },
    BreakIfFalse,
    Break,
    Assert,
    Require,
    ConditionFlow {
        branches: Vec<VMConditionBranch>,
    },
    PatternMatch {
        branches: Vec<VMPatternBranch>,
    },
    EntityFeature {
        feature_uid: EntityUid,
        ty: EthTerm,
    },
    PushEntityFp {
        opt_linkage: Option<__LinkageGroup>,
        ty: EthTerm,
        opt_instruction_region: Option<Instructions>,
    },
}
