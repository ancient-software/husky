mod ifelse;
mod r#loop;
mod r#match;

use crate::{coersion::VmirCoersion, expr::VmirExprIdx, pattern::VmirPatternIdx, *};
use husky_expr::stmt::ConditionConversion;
use husky_hir_eager_expr::{HirEagerCondition, HirEagerStmtData, HirEagerStmtIdxRange};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirStmtData<LinkageImpl: IsLinkageImpl> {
    Let,
    Return {
        result: VmirExprIdx<LinkageImpl>,
        coersion: VmirCoersion,
    },
    Require {
        condition: VmirCondition<LinkageImpl>,
    },
    Assert,
    Break,
    Eval {
        expr: VmirExprIdx<LinkageImpl>,
        coersion: Option<VmirCoersion>,
        discarded: bool,
    },
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

    fn to_vmir<Linktime>(
        self,
        builder: &mut crate::builder::VmirExprBuilder<Linktime>,
    ) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        let stmts = self
            .into_iter()
            .map(|stmt| match builder.hir_eager_stmt_arena()[stmt] {
                HirEagerStmtData::Let {
                    pattern,
                    contract,
                    initial_value,
                    coersion,
                } => VmirStmtData::Let,
                HirEagerStmtData::Return { result, coersion } => VmirStmtData::Return {
                    result: result.to_vmir(builder),
                    coersion: coersion.to_vmir(builder),
                },
                HirEagerStmtData::Require { ref condition } => VmirStmtData::Require {
                    condition: condition.to_vmir(builder),
                },
                HirEagerStmtData::Assert { ref condition } => VmirStmtData::Assert,
                HirEagerStmtData::Break => VmirStmtData::Break,
                HirEagerStmtData::Eval {
                    expr,
                    coersion,
                    discarded,
                } => VmirStmtData::Eval {
                    expr: expr.to_vmir(builder),
                    coersion: coersion.to_vmir(builder),
                    discarded,
                },
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

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirCondition<LinkageImpl: IsLinkageImpl> {
    /// `be` condition with syntactically correct pattern.
    /// This requires special handling for many cases.
    Be {
        opd: VmirExprIdx<LinkageImpl>,
        pattern: VmirPatternIdx<LinkageImpl>,
    },
    /// all other conditions.
    /// for simplicity, `be` with a syntactically broken pattern is also included in there
    Other {
        opd: VmirExprIdx<LinkageImpl>,
        conversion: VmirConditionConversion<LinkageImpl>,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirConditionConversion<LinkageImpl> {
    None,
    IntToBool,
    Todo(LinkageImpl),
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for &HirEagerCondition {
    type Output = VmirCondition<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirExprBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        match *self {
            HirEagerCondition::Be { opd, ref pattern } => VmirCondition::Be {
                opd: opd.to_vmir(builder),
                pattern: pattern.pattern.to_vmir(builder),
            },
            HirEagerCondition::Other { opd, conversion } => VmirCondition::Other {
                opd: opd.to_vmir(builder),
                conversion: conversion.to_vmir(builder),
            },
        }
    }
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for ConditionConversion {
    type Output = VmirConditionConversion<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirExprBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        match self {
            ConditionConversion::None => VmirConditionConversion::None,
            ConditionConversion::IntToBool(_) => VmirConditionConversion::IntToBool,
        }
    }
}
