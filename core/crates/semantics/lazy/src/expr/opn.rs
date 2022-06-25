use ast::FieldAstKind;
use entity_kind::FieldKind;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use text::RangedCustomIdentifier;
use vm::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyOpnKind {
    Binary {
        opr: PureBinaryOpr,
        this: EntityRoutePtr,
    },
    Prefix(PrefixOpr),
    FunctionModelCall(RangedEntityRoute),
    FunctionRoutineCall(RangedEntityRoute),
    StructCall(RangedEntityRoute),
    RecordCall(RangedEntityRoute),
    FieldAccess {
        field_ident: RangedCustomIdentifier,
        field_binding: Binding,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        method_route: EntityRoutePtr,
        output_binding: Binding,
    },
    ElementAccess {
        element_binding: Binding,
    },
}
