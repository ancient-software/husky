use crate::*;
use entity_route::{EntityKind, EntityRoutePtr, RangedEntityRoute};
use syntax_types::Opr;
use text::Row;
use vm::PrimitiveValue;
use word::RangedCustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprVariant {
    Variable {
        varname: CustomIdentifier,
        init_row: Row,
    },
    FrameVariable {
        varname: CustomIdentifier,
        init_row: Row,
    },
    This {
        ty: Option<EntityRoutePtr>,
    },
    Unrecognized(CustomIdentifier),
    Entity {
        route: EntityRoutePtr,
        kind: EntityKind,
    },
    PrimitiveLiteral(PrimitiveValue),
    Bracketed(RawExprIdx),
    Opn {
        opr: Opr,
        opds: RawExprRange,
    },
    Lambda(
        Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>,
        RawExprIdx,
    ),
}
