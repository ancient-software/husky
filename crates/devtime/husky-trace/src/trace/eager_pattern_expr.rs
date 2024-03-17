use husky_coword::IdentPairMap;
use husky_hir_eager_expr::var::rvar::HirEagerRvarIdx;
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprRegion};

use crate::registry::assoc_trace::VoidAssocTraceRegistry;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EagerPatternExprTracePathData {
    biological_parent_path: TracePath,
    essence: EagerPatternExprEssence,
    disambiguator: TracePathDisambiguator<EagerPatternExprEssence>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerPatternExprEssence {
    Haha,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EagerPatternExprTrace(Trace);

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerPatternExprTraceData {
    path: TracePath,
    biological_parent: Trace,
    syn_pattern_expr_idx: SynPatternIdx,
    hir_eager_runtime_symbol_idxs: IdentPairMap<Option<HirEagerRvarIdx>>,
    #[skip_fmt]
    sema_expr_region: SemaExprRegion,
}

impl Trace {
    pub(crate) fn new_eager_pattern_expr(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        syn_pattern_expr_idx: SynPatternIdx,
        hir_eager_runtime_symbol_idxs: IdentPairMap<Option<HirEagerRvarIdx>>,
        sema_expr_region: SemaExprRegion,
        eager_pattern_expr_trace_path_registry: &mut TracePathRegistry<EagerPatternExprEssence>,
        db: &::salsa::Db,
    ) -> Self {
        let essence = EagerPatternExprEssence::Haha;
        let path = TracePath::new(
            EagerPatternExprTracePathData {
                biological_parent_path,
                essence: essence.clone(),
                disambiguator: eager_pattern_expr_trace_path_registry.issue(essence),
            },
            db,
        );
        Trace::new(
            path,
            EagerPatternExprTraceData {
                path,
                biological_parent: biological_parent.into(),
                syn_pattern_expr_idx,
                hir_eager_runtime_symbol_idxs,
                sema_expr_region,
            }
            .into(),
            db,
        )
    }
}

impl EagerPatternExprTraceData {
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

    pub fn have_subtraces(&self, _db: &::salsa::Db) -> bool {
        false
    }

    pub fn subtraces(&self) -> Vec<Trace> {
        vec![]
    }
}
