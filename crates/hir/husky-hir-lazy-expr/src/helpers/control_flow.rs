use std::ops::FromResidual;

use crate::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyExprRegionControlFlowChart {
    // has value means has control flow
    hir_lazy_expr_control_flow_chart: HirLazyExprMap<HasControlFlow>,
    // has value means has control flow
    hir_lazy_stmt_control_flow_chart: HirLazyStmtMap<HasControlFlow>,
}

impl std::ops::Index<HirLazyExprIdx> for HirLazyExprRegionControlFlowChart {
    type Output = HasControlFlow;

    fn index(&self, index: HirLazyExprIdx) -> &Self::Output {
        &self.hir_lazy_expr_control_flow_chart[index]
    }
}

impl std::ops::Index<HirLazyStmtIdx> for HirLazyExprRegionControlFlowChart {
    type Output = HasControlFlow;

    fn index(&self, index: HirLazyStmtIdx) -> &Self::Output {
        &self.hir_lazy_stmt_control_flow_chart[index]
    }
}

impl HirLazyExprRegion {
    pub fn control_flow<'a>(self, db: &'a ::salsa::Db) -> &'a HirLazyExprRegionControlFlowChart {
        hir_lazy_expr_region_control_flow(db, self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HasControlFlow {
    True = 1,
    False = -1,
}

impl HasControlFlow {
    pub fn to_bool(self) -> bool {
        self == HasControlFlow::True
    }
}

impl std::ops::Try for HasControlFlow {
    type Output = ();

    type Residual = HasControlFlowR;

    fn from_output(_output: Self::Output) -> Self {
        Self::False
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HasControlFlow::True => std::ops::ControlFlow::Break(HasControlFlowR),
            HasControlFlow::False => std::ops::ControlFlow::Continue(()),
        }
    }
}

pub struct HasControlFlowR;

impl FromResidual<HasControlFlowR> for HasControlFlow {
    fn from_residual(_residual: HasControlFlowR) -> Self {
        HasControlFlow::True
    }
}

#[salsa::tracked(jar = HirLazyExprJar, return_ref)]
fn hir_lazy_expr_region_control_flow(
    db: &::salsa::Db,
    hir_lazy_expr_region: HirLazyExprRegion,
) -> HirLazyExprRegionControlFlowChart {
    let mut builder = HirLazyExprControlFlowRegionBuilder::new(hir_lazy_expr_region, db);
    builder.infer_all();
    builder.finish()
}

struct HirLazyExprControlFlowRegionBuilder<'a> {
    hir_lazy_expr_region_data: HirLazyExprRegionData<'a>,
    // has value means has control flow
    hir_lazy_expr_control_flow_chart: HirLazyExprMap<HasControlFlow>,
    // has value means has control flow
    hir_lazy_stmt_control_flow_chart: HirLazyStmtMap<HasControlFlow>,
}

impl<'a> HirLazyExprControlFlowRegionBuilder<'a> {
    fn new(hir_lazy_expr_region: HirLazyExprRegion, db: &'a ::salsa::Db) -> Self {
        let hir_lazy_expr_region_data = hir_lazy_expr_region.data(db);
        Self {
            hir_lazy_expr_region_data,
            hir_lazy_expr_control_flow_chart: HirLazyExprMap::new2(
                hir_lazy_expr_region_data.hir_lazy_expr_arena(),
            ),
            hir_lazy_stmt_control_flow_chart: HirLazyStmtMap::new2(
                hir_lazy_expr_region_data.hir_lazy_stmt_arena(),
            ),
        }
    }

    fn infer_all(&mut self) {
        for (hir_lazy_expr_idx, hir_lazy_expr) in self
            .hir_lazy_expr_region_data
            .hir_lazy_expr_arena()
            .indexed_iter()
        {
            self.infer_new_expr(hir_lazy_expr_idx, hir_lazy_expr);
        }
    }

    fn infer_new_expr(
        &mut self,
        hir_lazy_expr_idx: HirLazyExprIdx,
        hir_lazy_expr: &HirLazyExprData,
    ) {
        let has_control_flow = self.infer_new_expr_aux(hir_lazy_expr);
        self.hir_lazy_expr_control_flow_chart
            .insert_new(hir_lazy_expr_idx, has_control_flow)
    }

    fn infer_new_expr_aux(&mut self, hir_lazy_expr: &HirLazyExprData) -> HasControlFlow {
        match *hir_lazy_expr {
            HirLazyExprData::Literal(_)
            | HirLazyExprData::PrincipalEntityPath(_)
            | HirLazyExprData::ConstSymbol(_)
            | HirLazyExprData::Variable(_) => (),
            HirLazyExprData::Binary { lopd, ropd, .. } => {
                self.expr_has_control_flow(lopd)?;
                self.expr_has_control_flow(ropd)?
            }
            HirLazyExprData::Be { src, pattern: _ } => self.expr_has_control_flow(src)?,
            HirLazyExprData::Prefix {
                opd_hir_expr_idx, ..
            } => self.expr_has_control_flow(opd_hir_expr_idx)?,
            HirLazyExprData::Suffix {
                opd_hir_expr_idx, ..
            } => self.expr_has_control_flow(opd_hir_expr_idx)?,
            HirLazyExprData::Unveil { .. } => HasControlFlow::True?,
            HirLazyExprData::Unwrap {
                opd_hir_expr_idx, ..
            } => self.expr_has_control_flow(opd_hir_expr_idx)?,
            HirLazyExprData::TypeConstructorCall {
                ref item_groups, ..
            }
            | HirLazyExprData::TypeVariantConstructorCall {
                ref item_groups, ..
            }
            | HirLazyExprData::FunctionRitchieItemCall {
                ref item_groups, ..
            }
            | HirLazyExprData::AssocFunctionRitchieCall {
                ref item_groups, ..
            } => self.infer_new_item_groups(item_groups)?,
            HirLazyExprData::FunctionRitchieItemCall {
                ref item_groups, ..
            } => self.infer_new_item_groups(item_groups)?,
            HirLazyExprData::PropsStructField {
                self_argument: owner,
                ..
            }
            | HirLazyExprData::MemoizedField { owner, .. } => self.expr_has_control_flow(owner)?,
            HirLazyExprData::MethodRitchieCall {
                self_argument,
                ref item_groups,
                ..
            } => {
                self.expr_has_control_flow(self_argument)?;
                self.infer_new_item_groups(item_groups)?
            }
            HirLazyExprData::NewTuple { ref items } => {
                for _item in items {
                    todo!()
                }
            }
            HirLazyExprData::Index { owner, ref items } => {
                self.expr_has_control_flow(owner)?;
                self.infer_new_exprs(items)?
            }
            HirLazyExprData::ConstructList { ref items, .. } => self.infer_new_exprs(items)?,
            HirLazyExprData::Block { stmts } => {
                for stmt in stmts {
                    self.infer_new_stmt(stmt)?
                }
            }
            HirLazyExprData::EmptyHtmlTag {
                function_ident: _,
                ref arguments,
            } => {
                for argument in arguments.iter() {
                    match *argument {
                        HirLazyHtmlArgumentExpr::Expanded { expr, .. } => {
                            self.expr_has_control_flow(expr)?
                        }
                        HirLazyHtmlArgumentExpr::Shortened { .. } => (),
                    }
                }
            }
            HirLazyExprData::Todo => (),
            HirLazyExprData::Unreachable => (),
            HirLazyExprData::AssocRitchie { .. } => (),
            HirLazyExprData::As { opd, .. } => self.expr_has_control_flow(opd)?,
        }
        HasControlFlow::False
    }

    fn infer_new_item_groups(&mut self, item_groups: &[HirLazyCallListArgument]) -> HasControlFlow {
        for item_group in item_groups {
            match *item_group {
                HirLazyCallListArgument::Simple(item_expr_idx) => {
                    self.expr_has_control_flow(item_expr_idx)?
                }
                HirLazyCallListArgument::Variadic(ref item_expr_idxs) => {
                    for &item_expr_idx in item_expr_idxs {
                        self.expr_has_control_flow(item_expr_idx)?
                    }
                }
                HirLazyCallListArgument::Keyed(_, item_expr_idx) => {
                    if let Some(item_expr_idx) = item_expr_idx {
                        self.expr_has_control_flow(item_expr_idx)?
                    }
                }
            }
        }
        HasControlFlow::False
    }

    fn infer_new_exprs(&mut self, exprs: &[HirLazyExprIdx]) -> HasControlFlow {
        for &expr in exprs {
            self.expr_has_control_flow(expr)?
        }
        HasControlFlow::False
    }

    fn expr_has_control_flow(&self, hir_lazy_expr_idx: HirLazyExprIdx) -> HasControlFlow {
        self.hir_lazy_expr_control_flow_chart[hir_lazy_expr_idx]
    }

    fn condition_has_control_flow(&self, condition: &HirLazyCondition) -> HasControlFlow {
        match *condition {
            HirLazyCondition::Be { src, .. } => self.hir_lazy_expr_control_flow_chart[src],
            HirLazyCondition::Other {
                hir_lazy_expr_idx, ..
            } => self.hir_lazy_expr_control_flow_chart[hir_lazy_expr_idx],
        }
    }

    fn infer_new_stmts(&mut self, stmts: HirLazyStmtIdxRange) -> HasControlFlow {
        for stmt in stmts {
            self.infer_new_stmt(stmt)?
        }
        HasControlFlow::False
    }

    fn infer_new_stmt(&mut self, hir_lazy_stmt: HirLazyStmtIdx) -> HasControlFlow {
        let has_control_flow = self.infer_new_stmt_aux(
            &self.hir_lazy_expr_region_data.hir_lazy_stmt_arena()[hir_lazy_stmt],
        );
        self.hir_lazy_stmt_control_flow_chart
            .insert_new(hir_lazy_stmt, has_control_flow);
        has_control_flow
    }

    fn infer_new_stmt_aux(&mut self, hir_lazy_stmt: &HirLazyStmtData) -> HasControlFlow {
        match *hir_lazy_stmt {
            HirLazyStmtData::Let {
                pattern: _,
                initial_value,
            } => self.expr_has_control_flow(initial_value),
            HirLazyStmtData::Return { result: _ } => HasControlFlow::True,
            HirLazyStmtData::Require { .. } => HasControlFlow::True,
            HirLazyStmtData::Assert { ref condition } => self.condition_has_control_flow(condition),
            HirLazyStmtData::Eval { expr_idx, .. } => self.expr_has_control_flow(expr_idx),
            HirLazyStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                self.infer_new_stmts(if_branch.stmts)?;
                for elif_branch in elif_branches {
                    self.infer_new_stmts(elif_branch.stmts)?;
                }
                if let Some(else_branch) = else_branch {
                    self.infer_new_stmts(else_branch.stmts)?
                }
                HasControlFlow::False
            }
            // ad hoc
            HirLazyStmtData::Match {} => HasControlFlow::False,
            HirLazyStmtData::Narrate { .. } => HasControlFlow::False,
        }
    }

    fn finish(self) -> HirLazyExprRegionControlFlowChart {
        HirLazyExprRegionControlFlowChart {
            hir_lazy_expr_control_flow_chart: self.hir_lazy_expr_control_flow_chart,
            hir_lazy_stmt_control_flow_chart: self.hir_lazy_stmt_control_flow_chart,
        }
    }
}
