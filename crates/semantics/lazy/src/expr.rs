mod opn;
mod parser;

use std::sync::Arc;

use file::FilePtr;
pub use opn::*;
pub(crate) use parser::LazyExprParser;

use entity_route::{EntityRoute, EntityRoutePtr, RangedEntityRoute};
use syntax_types::*;
use text::TextRange;
use vm::*;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub ty: EntityRoutePtr,
    pub kind: LazyExprKind,
    pub instruction_id: InstructionId,
    pub contract: LazyContract,
}

impl InstructionSource for LazyExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }

    fn file(&self) -> FilePtr {
        self.file
    }

    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyExprKind {
    Variable(CustomIdentifier),
    Scope {
        scope: EntityRoutePtr,
        compiled: (),
    },
    PrimitiveLiteral(PrimitiveValue),
    EnumLiteral {
        entity_route: EntityRoutePtr,
    },
    Bracketed(Arc<LazyExpr>),
    Opn {
        opn_kind: LazyOpnKind,
        opds: Vec<Arc<LazyExpr>>,
    },
    Lambda(
        Vec<(CustomIdentifier, Option<EntityRoutePtr>)>,
        Box<LazyExpr>,
    ),
    This,
    EntityFeature {
        route: EntityRoutePtr,
    },
}
