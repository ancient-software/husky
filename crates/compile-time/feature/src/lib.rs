mod branch;
mod eval;
mod expr;
mod object;
mod query;
mod record;
mod repr;
mod sheet;
mod stmt;
mod this;
mod unique_allocate;

pub use branch::{FeatureBranch, FeatureBranchVariant};
pub use eval::{eval_feature_block, eval_feature_expr, eval_feature_stmt, FeatureEvalIndicator};
pub use expr::{FeatureExpr, FeatureExprKind};
pub use query::{FeatureQueryGroup, FeatureQueryGroupStorage};
pub use repr::*;
pub use sheet::FeatureSheet;
pub use stmt::{FeatureStmt, FeatureStmtVariant};
pub use this::FeatureBlock;
pub use unique_allocate::{
    new_feature_unique_allocator, AllocateUniqueFeature, FeaturePtr, FeatureUniqueAllocator,
};

use defn_head::*;
use entity_route::EntityRoutePtr;
use eval::*;
use object::Object;
use print_utils::*;
use std::sync::Arc;
use text::*;
use vm::EntityUid;
use vm::{PrimitiveValue, PureBinaryOpr};
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureSymbol {
    varname: CustomIdentifier,
    value: Arc<FeatureExpr>,
    feature: FeaturePtr,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Feature {
    Input,
    PrimitiveLiteral(PrimitiveValue),
    EnumLiteral(EntityRoutePtr),
    Assert {
        condition: FeaturePtr,
    },
    Cascade(Vec<FeaturePtr>),
    PrimitiveBinaryOpr {
        opr: PureBinaryOpr,
        lopd: FeaturePtr,
        ropd: FeaturePtr,
    },
    FuncCall {
        func: EntityRoutePtr,
        uid: EntityUid,
        inputs: Vec<FeaturePtr>,
    },
    Branches {
        branches: Vec<BranchedFeature>,
    },
    StructOriginalFieldAccess {
        this: FeaturePtr,
        field_ident: CustomIdentifier,
    },
    RecordDerivedFieldAccess {
        this: FeaturePtr,
        field_uid: EntityUid,
    },
    ElementAccess {
        opds: Vec<FeaturePtr>,
    },
    MethodCall {
        method_ident: CustomIdentifier,
        opds: Vec<FeaturePtr>,
    },
    EntityFeature {
        route: EntityRoutePtr,
        uid: EntityUid,
    },
    RecordTypeCall {
        ty: EntityRoutePtr,
        uid: EntityUid,
        opds: Vec<FeaturePtr>,
    },
}

impl Feature {
    pub fn block(features: &FeatureUniqueAllocator, stmts: &[Arc<FeatureStmt>]) -> FeaturePtr {
        let stmt_features: Vec<_> = stmts.iter().filter_map(|stmt| stmt.feature).collect();
        if stmt_features.len() == 1 {
            stmt_features[0]
        } else {
            features.alloc(Feature::Cascade(stmt_features))
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct BranchedFeature {
    condition: Option<FeaturePtr>,
    block: FeaturePtr,
}

impl From<&Feature> for Feature {
    fn from(feature: &Feature) -> Self {
        feature.clone()
    }
}
