mod ifelse;
mod r#loop;
mod r#match;

use crate::{expr::VmirExprIdx, pattern::VmirPattern, ToVmir};
use husky_hir_eager_expr::{HirEagerStmtData, HirEagerStmtIdxRange};
use husky_linkage::linkage::Linkage;
use husky_task_interface::IsLinkageImpl;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, PartialEq, Eq)]
pub enum VmirStmtData<LinkageImpl: IsLinkageImpl> {
    Let,
    Return { expr: VmirExprIdx<LinkageImpl> },
    Require,
    Assert,
    Break,
    Eval,
    ForBetween,
    Forext,
    ForIn,
    While,
    DoWhile,
    IfElse,
    Match,
}

pub type VmirStmtArena<LinkageImpl> = Arena<VmirStmtData<LinkageImpl>>;
pub type VmirStmtIdx<LinkageImpl> = ArenaIdx<VmirStmtData<LinkageImpl>>;
pub type VmirStmtIdxRange<LinkageImpl> = ArenaIdxRange<VmirStmtData<LinkageImpl>>;

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerStmtIdxRange {
    type Output = VmirStmtIdxRange<LinkageImpl>;

    fn to_vmir(self, builder: &mut crate::builder::VmirExprBuilder<LinkageImpl>) -> Self::Output {
        let stmts = self
            .into_iter()
            .map(|stmt| match builder.hir_eager_stmt_arena()[stmt] {
                HirEagerStmtData::Let {
                    pattern,
                    contract,
                    initial_value,
                    coersion,
                } => VmirStmtData::Let,
                HirEagerStmtData::Return { result, coersion } => {
                    VmirStmtData::Return { expr: todo!() }
                }
                HirEagerStmtData::Require { ref condition } => VmirStmtData::Require,
                HirEagerStmtData::Assert { ref condition } => VmirStmtData::Assert,
                HirEagerStmtData::Break => VmirStmtData::Break,
                HirEagerStmtData::Eval {
                    expr_idx,
                    coersion,
                    discarded,
                } => VmirStmtData::Eval,
                HirEagerStmtData::ForBetween {
                    ref particulars,
                    block,
                } => VmirStmtData::ForBetween,
                HirEagerStmtData::Forext {
                    ref particulars,
                    block,
                } => VmirStmtData::Forext,
                HirEagerStmtData::ForIn {
                    ref condition,
                    block,
                } => VmirStmtData::ForIn,
                HirEagerStmtData::While {
                    ref condition,
                    stmts,
                } => VmirStmtData::While,
                HirEagerStmtData::DoWhile {
                    ref condition,
                    block,
                } => VmirStmtData::DoWhile,
                HirEagerStmtData::IfElse {
                    ref if_branch,
                    ref elif_branches,
                    ref else_branch,
                } => VmirStmtData::IfElse,
                HirEagerStmtData::Match {
                    ref case_branches,
                    ref match_target,
                } => VmirStmtData::Match,
            })
            .collect();
        builder.alloc_stmts(stmts)
    }
}

pub enum VmirCondition<LinkageImpl: IsLinkageImpl> {
    /// `be` condition with syntactically correct pattern.
    /// This requires special handling for many cases.
    Be {
        src: VmirExprIdx<LinkageImpl>,
        target: VmirPattern,
    },
    /// all other conditions.
    /// for simplicity, `be` with a syntactically broken pattern is also included in there
    Other {
        expr: VmirExprIdx<LinkageImpl>,
        conversion: VmirConditionConversion,
    },
}

pub enum VmirConditionConversion {}
