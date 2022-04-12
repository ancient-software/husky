use std::sync::Arc;

use entity_route::{EntityRoutePtr, RangedEntityRoute};
use infer_decl::TyDecl;
use syntax_types::{PrefixOpr, SuffixOpr};
use vm::{BinaryOpr, FieldContract};
use word::RangedCustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerOpnKind {
    Binary {
        opr: BinaryOpr,
        this: EntityRoutePtr,
    },
    Prefix {
        opr: PrefixOpr,
        this: EntityRoutePtr,
    },
    Suffix {
        opr: SuffixOpr,
        this: EntityRoutePtr,
    },
    RoutineCall(RangedEntityRoute),
    TypeCall {
        ranged_ty: RangedEntityRoute,
        ty_decl: Arc<TyDecl>,
    },
    PatternCall,
    MembVarAccess {
        field_var_contract: FieldContract,
    },
    MembRoutineCall {
        field_ident: RangedCustomIdentifier,
        this_ty_decl: Arc<TyDecl>,
    },
    ElementAccess,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum PrefixOpn {}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum SuffixOpn {}
