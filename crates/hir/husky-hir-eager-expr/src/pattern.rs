use husky_hir_ty::HirType;

use husky_sema_expr::LetVariableObelisk;
use husky_syn_expr::BePatternSyndicate;

use crate::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerLetVariablesPattern {
    pub pattern_expr_idx: HirEagerPatternExprIdx,
    // variables: CurrentHirEagerSymbolIdxRange,
    pub ty: Option<HirType>,
}

impl HirEagerLetVariablesPattern {
    pub fn pattern_expr_idx(self) -> HirEagerPatternExprIdx {
        self.pattern_expr_idx
    }

    pub fn ty(self) -> Option<HirType> {
        self.ty
    }
}

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_let_variables_pattern(
        &mut self,
        let_pattern_sema_obelisk: &LetVariableObelisk,
    ) -> HirEagerLetVariablesPattern {
        HirEagerLetVariablesPattern {
            pattern_expr_idx: self.new_pattern_expr(let_pattern_sema_obelisk.syn_pattern_root()),
            ty: let_pattern_sema_obelisk
                .ty_sema_expr_idx()
                .map(|ty_sema_expr_idx| {
                    HirType::from_eth(self.expr_term(ty_sema_expr_idx), self.db()).unwrap()
                }),
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerBeVariablesPattern {
    pub pattern_expr_idx: HirEagerPatternExprIdx,
}

impl ToHirEager for BePatternSyndicate {
    type Output = HirEagerBeVariablesPattern;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerBeVariablesPattern {
            pattern_expr_idx: builder.new_pattern_expr(self.syn_pattern_root()),
        }
    }
}
