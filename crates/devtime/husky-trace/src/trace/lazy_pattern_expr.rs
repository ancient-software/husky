use super::*;
use crate::registry::assoc_trace::VoidAssocTraceRegistry;
use husky_coword::IdentPairMap;
use husky_hir_lazy_expr::{
    variable::HirLazyVariableIdx, HirLazyExprRegion, HirLazyPatternExpr, HirLazyPatternExprIdx,
};
use husky_ki_repr::expansion::KiReprExpansion;
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprRegion};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LazyPatternExprTracePath(TracePath);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LazyPatternExprTracePathData {
    biological_parent_path: TracePath,
    essence: LazyPatternExprEssence,
    disambiguator: TracePathDisambiguator<LazyPatternExprEssence>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyPatternExprEssence {
    AdHoc,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LazyPatternExprTraceData {
    path: TracePath,
    biological_parent: Trace,
    syn_pattern_expr_idx: PatternSynExprIdx,
    hir_lazy_pattern_expr_idx: Option<HirLazyPatternExprIdx>,
    hir_lazy_variable_idxs: IdentPairMap<Option<HirLazyVariableIdx>>,
    #[skip_fmt]
    sema_expr_region: SemaExprRegion,
    #[skip_fmt]
    hir_lazy_expr_region: HirLazyExprRegion,
}

impl Trace {
    pub(crate) fn new_lazy_pattern_expr(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        syn_pattern_expr_idx: PatternSynExprIdx,
        hir_lazy_pattern_expr_idx: Option<HirLazyPatternExprIdx>,
        hir_lazy_variable_idxs: IdentPairMap<Option<HirLazyVariableIdx>>,
        sema_expr_region: SemaExprRegion,
        hir_lazy_expr_region: HirLazyExprRegion,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyPatternExprEssence>,
        db: &::salsa::Db,
    ) -> Self {
        let essence = LazyPatternExprEssence::AdHoc;
        let path = TracePath::new(
            LazyPatternExprTracePathData {
                biological_parent_path,
                essence: essence.clone(),
                disambiguator: lazy_expr_trace_path_registry.issue(essence),
            },
            db,
        );
        Trace::new(
            path,
            LazyPatternExprTraceData {
                path,
                biological_parent: biological_parent.into(),
                syn_pattern_expr_idx,
                hir_lazy_pattern_expr_idx,
                hir_lazy_variable_idxs,
                sema_expr_region,
                hir_lazy_expr_region,
            }
            .into(),
            db,
        )
    }
}

impl LazyPatternExprTraceData {
    pub fn have_subtraces(&self) -> bool {
        false
    }

    pub(super) fn subtraces(&self) -> Vec<Trace> {
        vec![]
    }

    pub(super) fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
        let sema_expr_region = self.sema_expr_region;
        let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
        let sema_expr_range_region_data = sema_expr_range_region.data(db);
        let region_path = sema_expr_region.path(db);
        let regional_token_idx_range = sema_expr_range_region_data[self.syn_pattern_expr_idx];
        let token_idx_range = regional_token_idx_range
            .token_idx_range(region_path.regional_token_idx_base(db).unwrap());
        TraceViewLines::new(
            region_path.module_path(db),
            token_idx_range,
            VoidAssocTraceRegistry,
            db,
        )
    }

    pub(super) fn ki_repr(&self, trace_id: Trace, db: &::salsa::Db) -> Option<KiRepr> {
        let ki_repr_expansion = trace_ki_repr_expansion(db, trace_id);
        match self.hir_lazy_expr_region.hir_lazy_pattern_expr_arena(db)
            [self.hir_lazy_pattern_expr_idx?]
        {
            HirLazyPatternExpr::Literal(_) => todo!(),
            HirLazyPatternExpr::Ident { .. } => {
                let hir_lazy_variable_idxs = &self.hir_lazy_variable_idxs;
                debug_assert_eq!(hir_lazy_variable_idxs.len(), 1);
                let hir_lazy_variable_idx = hir_lazy_variable_idxs.data()[0].1?;
                Some(ki_repr_expansion.hir_lazy_variable_ki_repr_map(db)[hir_lazy_variable_idx])
            }
            HirLazyPatternExpr::Unit(_) => todo!(),
            HirLazyPatternExpr::Tuple { path: _, fields: _ } => todo!(),
            HirLazyPatternExpr::Props { path: _, fields: _ } => todo!(),
            HirLazyPatternExpr::OneOf { options: _ } => todo!(),
            HirLazyPatternExpr::Binding { ident: _, src: _ } => todo!(),
            HirLazyPatternExpr::Range { start: _, end: _ } => todo!(),
        }
    }

    pub(super) fn ki_repr_expansion(&self, db: &::salsa::Db) -> KiReprExpansion {
        self.biological_parent.ki_repr_expansion(db)
    }
}
