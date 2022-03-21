use std::sync::Arc;

use atom::AtomKind;
use common::*;
use text::Row;
use vm::PrimitiveValue;
use word::WordPtr;

use crate::*;
use scope::{RangedScope, ScopeKind, ScopePtr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprKind {
    Variable {
        varname: CustomIdentifier,
        init_row: Row,
    },
    Unrecognized(CustomIdentifier),
    Scope {
        scope: ScopePtr,
        kind: ScopeKind,
    },
    PrimitiveLiteral(PrimitiveValue),
    Bracketed(RawExprIdx),
    Opn {
        opr: Opr,
        opds: RawExprRange,
    },
    Lambda(Vec<(CustomIdentifier, Option<RangedScope>)>, RawExprIdx),
}

// impl From<AtomKind> for RawExprKind {
//     fn from(kind: AtomKind) -> Self {
//         match kind {
//             AtomKind::Variable(ident) => RawExprKind::Variable(ident),
//             AtomKind::Literal(literal) => RawExprKind::Literal(literal.clone()),
//             AtomKind::Scope {
//                 scope,
//                 kind,
//                 ref tokens,
//             } => RawExprKind::Scope {
//                 scope,
//                 kind,
//                 tokens: tokens.clone(),
//             },
//             _ => {
//                 p!(kind);
//                 panic!()
//             }
//         }
//     }
// }
