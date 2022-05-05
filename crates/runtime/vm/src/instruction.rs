mod id;
mod opr;
mod sheet;

pub use opr::*;

use entity_route::EntityRoutePtr;
use file::FilePtr;
pub use id::{InstructionId, InstructionSource};
pub use sheet::InstructionSheet;
use text::TextRange;

use std::{ops::Deref, panic::RefUnwindSafe, sync::Arc};

use crate::*;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub src: Arc<dyn InstructionSource>,
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.src.instruction_id() == other.src.instruction_id()
    }
}

impl Eq for Instruction {}

impl Instruction {
    pub fn new(kind: InstructionKind, src: Arc<dyn InstructionSource>) -> Self {
        Self { kind, src }
    }

    pub fn id(&self) -> InstructionId {
        self.src.instruction_id()
    }
}

impl<
        S: InstructionSource,
        T: Deref<Target = S> + std::fmt::Debug + Send + Sync + RefUnwindSafe,
    > InstructionSource for T
{
    fn instruction_id(&self) -> InstructionId {
        let this: &S = self;
        this.instruction_id()
    }
    fn file(&self) -> FilePtr {
        let this: &S = self;
        this.file()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InstructionKind {
    PushVariable {
        stack_idx: StackIdx,
        binding: Binding,
        range: TextRange,
        ty: EntityRoutePtr,
    },
    PushPrimitiveLiteral(PrimitiveValue),
    FieldAccessCompiled {
        linkage: Linkage,
    },
    FieldAccessInterpreted {
        field_idx: u8,
        contract: EagerContract,
    },
    RoutineCallCompiled {
        linkage: Linkage,
    },
    RoutineCallInterpreted {
        routine: EntityUid,
        nargs: u8,
    },
    NewVirtualStruct {
        fields: Vec<FieldContract>,
    },
    PrimitiveOpn {
        opn: PrimitiveOpn,
        this_ty: EntityRoutePtr,
        this_range: TextRange,
    },
    Loop {
        body: Arc<InstructionSheet>,
        loop_kind: VMLoopKind,
    },
    Return,
    BreakIfFalse,
    Break,
    Assert,
    BranchGroup {
        branches: Vec<()>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitKind {
    Let,
    Var,
    Decl,
}

impl std::fmt::Display for InitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            InitKind::Let => "let",
            InitKind::Var => "var",
            InitKind::Decl => "decl",
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitiveOpn {
    PureBinary(PureBinaryOpr),
    Assign(Option<PureBinaryOpr>),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
}
