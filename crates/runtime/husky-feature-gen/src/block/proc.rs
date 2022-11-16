use super::*;
use avec::Avec;
use husky_eager_semantics::ProcStmt;
use husky_term::Ty;
use husky_vm::{InstructionSheet, __Linkage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureProcBody {
    pub opt_this: Option<FeatureRepr>,
    pub feature: FeatureItd,
    pub file: PathItd,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub return_ty: Ty,
    pub stmts: Avec<ProcStmt>,
    pub instruction_sheet: Arc<InstructionSheet>,
    pub opt_linkage: Option<__Linkage>,
}

impl<'eval> std::hash::Hash for FeatureProcBody {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}
