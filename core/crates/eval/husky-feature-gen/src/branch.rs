use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureBranch {
    pub block: Arc<FeatureLazyBlock>,
    pub variant: FeatureBranchVariant,
    pub opt_arrival_indicator: Option<Arc<FeatureArrivalIndicator>>,
    pub(crate) eval_id: FeatureEvalId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchVariant {
    If { condition: Arc<FeatureExpr> },
    Elif { condition: Arc<FeatureExpr> },
    Else,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureArrivalIndicator {
    pub variant: FeatureBranchIndicatorVariant,
    pub feature: FeaturePtr,
}

impl FeatureArrivalIndicator {
    pub fn new(
        variant: FeatureBranchIndicatorVariant,
        feature_interner: &FeatureInterner,
    ) -> Arc<Self> {
        let feature = feature_interner.intern(match variant {
            FeatureBranchIndicatorVariant::AfterStmtNotReturn { ref stmt } => {
                Feature::ArrivalAfterStmtNotReturn {
                    stmt: stmt.opt_feature.unwrap(),
                }
            }
            FeatureBranchIndicatorVariant::AfterConditionNotMet {
                ref opt_parent,
                ref condition,
            } => Feature::ArrivalAfterConditionNotMet {
                opt_parent: opt_parent.as_ref().map(|p| p.feature),
                condition: condition.feature,
            },
            FeatureBranchIndicatorVariant::IfConditionMet {
                ref opt_parent,
                ref condition,
            } => Feature::ArrivalIfConditionMet {
                opt_parent: opt_parent.as_ref().map(|p| p.feature),
                condition: condition.feature,
            },
        });
        Arc::new(Self { variant, feature })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureBranchIndicatorVariant {
    AfterStmtNotReturn {
        stmt: Arc<FeatureStmt>,
    },
    AfterConditionNotMet {
        opt_parent: Option<Arc<FeatureArrivalIndicator>>,
        condition: Arc<FeatureExpr>,
    },
    IfConditionMet {
        opt_parent: Option<Arc<FeatureArrivalIndicator>>,
        condition: Arc<FeatureExpr>,
    },
}
