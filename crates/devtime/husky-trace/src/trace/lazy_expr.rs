use super::*;
use crate::registry::assoc_trace::VoidAssocTraceRegistry;
use husky_hir_lazy_expr::{
    source_map::{HirLazyExprSourceMap, HirLazyExprSourceMapData},
    HirLazyExprData, HirLazyExprIdx, HirLazyExprRegion,
};
use husky_ki_repr::expansion::KiReprExpansion;
use husky_sema_expr::{
    helpers::range::sema_expr_range_region, SemaExprData, SemaExprRegion,
    SemaRitchieParameterArgumentMatch,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LazyExprTracePathData {
    biological_parent_path: TracePath,
    essence: LazyExprEssence,
    disambiguator: TracePathDisambiguator<LazyExprEssence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LazyExprEssence {
    Haha,
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyExprTraceData {
    path: TracePath,
    biological_parent: Trace,
    sema_expr_idx: SemaExprIdx,
    hir_lazy_expr_idx: Option<HirLazyExprIdx>,
    #[skip_fmt]
    sema_expr_region: SemaExprRegion,
    #[skip_fmt]
    hir_lazy_expr_region: HirLazyExprRegion,
    #[skip_fmt]
    hir_lazy_expr_source_map: HirLazyExprSourceMap,
}

impl Trace {
    pub(crate) fn new_lazy_expr(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        sema_expr_idx: SemaExprIdx,
        hir_lazy_expr_idx: Option<HirLazyExprIdx>,
        sema_expr_region: SemaExprRegion,
        hir_lazy_expr_region: HirLazyExprRegion,
        hir_lazy_expr_source_map: HirLazyExprSourceMap,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyExprEssence>,
        db: &::salsa::Db,
    ) -> Self {
        let essence = LazyExprEssence::Haha;
        let path = TracePath::new(
            LazyExprTracePathData {
                biological_parent_path,
                essence: essence.clone(),
                disambiguator: lazy_expr_trace_path_registry.issue(essence),
            },
            db,
        );
        Trace::new(
            path,
            LazyExprTraceData {
                path,
                biological_parent: biological_parent.into(),
                sema_expr_idx,
                hir_lazy_expr_idx,
                sema_expr_region,
                hir_lazy_expr_region,
                hir_lazy_expr_source_map,
            }
            .into(),
            db,
        )
    }
}

impl LazyExprTraceData {
    pub(super) fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
        let sema_expr_region = self.sema_expr_region;
        let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
        let sema_expr_range_region_data = sema_expr_range_region.data(db);
        let region_path = sema_expr_region.path(db);
        let regional_token_idx_range = sema_expr_range_region_data[self.sema_expr_idx];
        let token_idx_range = regional_token_idx_range
            .token_idx_range(region_path.regional_token_idx_base(db).unwrap());
        TraceViewLines::new(
            region_path.module_path(db),
            token_idx_range,
            VoidAssocTraceRegistry,
            db,
        )
    }

    pub(super) fn have_subtraces(&self, db: &::salsa::Db) -> bool {
        use husky_hir_defn::defn::HasHirDefn;
        let Some(hir_eager_expr_idx) = self.hir_lazy_expr_idx else {
            return false;
        };
        match self.hir_lazy_expr_region.hir_lazy_expr_arena(db)[hir_eager_expr_idx] {
            HirLazyExprData::FunctionFnItemCall { path, .. } => path.hir_defn(db).is_some(),
            HirLazyExprData::AssocFunctionFnCall { path, .. } => path.hir_defn(db).is_some(),
            HirLazyExprData::MethodFnCall { path, .. } => path.hir_defn(db).is_some(),
            HirLazyExprData::AssocFn { path } => path.hir_defn(db).is_some(),
            HirLazyExprData::FunctionGnItemCall { path, .. } => path.hir_defn(db).is_some(),
            HirLazyExprData::Block { stmts: _ } => unreachable!(),
            _ => false,
        }
    }

    pub(super) fn subtraces(&self, trace: Trace, db: &::salsa::Db) -> Vec<Trace> {
        let biological_parent_path = self.path;
        let biological_parent = trace;
        let sema_expr_idx = self.sema_expr_idx;
        let Some(hir_eager_expr_idx) = self.hir_lazy_expr_idx else {
            return vec![];
        };
        let sema_expr_region_data = self.sema_expr_region.data(db);
        let hir_lazy_expr_source_map_data = self.hir_lazy_expr_source_map.data(db);
        match self.hir_lazy_expr_region.hir_lazy_expr_arena(db)[hir_eager_expr_idx] {
            HirLazyExprData::FunctionFnItemCall { path, .. } => {
                let SemaExprData::FunctionRitchieCall {
                    ref ritchie_parameter_argument_matches,
                    ..
                } = sema_expr_idx.data(sema_expr_region_data.sema_expr_arena())
                else {
                    unreachable!()
                };
                let mut subtraces: Vec<Trace> = fn_call_lazy_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    hir_lazy_expr_source_map_data,
                    db,
                );
                subtraces.push(
                    Trace::new_lazy_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
                        db,
                    )
                    .into(),
                );
                subtraces
            }
            HirLazyExprData::AssocFunctionFnCall { path, .. } => {
                let SemaExprData::FunctionRitchieCall {
                    ref ritchie_parameter_argument_matches,
                    ..
                } = sema_expr_idx.data(sema_expr_region_data.sema_expr_arena())
                else {
                    unreachable!()
                };
                let mut subtraces: Vec<Trace> = fn_call_lazy_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    hir_lazy_expr_source_map_data,
                    db,
                );
                subtraces.push(
                    Trace::new_lazy_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
                        db,
                    )
                    .into(),
                );
                subtraces
            }
            HirLazyExprData::MethodFnCall { path, .. } => {
                let SemaExprData::FunctionRitchieCall {
                    ref ritchie_parameter_argument_matches,
                    ..
                } = sema_expr_idx.data(sema_expr_region_data.sema_expr_arena())
                else {
                    unreachable!()
                };
                let mut subtraces: Vec<Trace> = fn_call_lazy_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    hir_lazy_expr_source_map_data,
                    db,
                );
                subtraces.push(
                    Trace::new_lazy_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
                        db,
                    )
                    .into(),
                );
                subtraces
            }
            HirLazyExprData::Block { .. } => unreachable!(),
            HirLazyExprData::FunctionGnItemCall { path, .. } => {
                let SemaExprData::FunctionRitchieCall {
                    ref ritchie_parameter_argument_matches,
                    ..
                } = sema_expr_idx.data(sema_expr_region_data.sema_expr_arena())
                else {
                    unreachable!()
                };
                let mut subtraces: Vec<Trace> = fn_call_lazy_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    hir_lazy_expr_source_map_data,
                    db,
                );
                subtraces.push(
                    Trace::new_lazy_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
                        db,
                    )
                    .into(),
                );
                subtraces
            }
            _ => vec![],
        }
    }

    pub(super) fn ki_repr(&self, trace_id: Trace, db: &::salsa::Db) -> Option<KiRepr> {
        let ki_repr_expansion = trace_ki_repr_expansion(db, trace_id);
        ki_repr_expansion
            .hir_lazy_expr_ki_repr_map(db)
            .get(self.hir_lazy_expr_idx?)
            .copied()
    }

    pub(super) fn ki_repr_expansion(&self, db: &::salsa::Db) -> KiReprExpansion {
        self.biological_parent.ki_repr_expansion(db)
    }
}

fn fn_call_lazy_expr_trace_input_traces(
    trace_path: TracePath,
    trace: Trace,
    ritchie_parameter_argument_matches: &[SemaRitchieParameterArgumentMatch],
    hir_lazy_expr_source_map_data: &HirLazyExprSourceMapData,
    db: &::salsa::Db,
) -> Vec<Trace> {
    ritchie_parameter_argument_matches
        .iter()
        .map(|m| {
            let data = match m {
                SemaRitchieParameterArgumentMatch::Simple(_, list_item) => {
                    let sema_expr_idx = list_item.argument_sema_expr_idx();
                    LazyCallInputSketch::Regular {
                        sema_expr_idx,
                        hir_lazy_expr_idx: hir_lazy_expr_source_map_data
                            .sema_to_hir_lazy_expr_idx(sema_expr_idx),
                    }
                }
                SemaRitchieParameterArgumentMatch::Variadic(_, _) => {
                    todo!()
                }
                SemaRitchieParameterArgumentMatch::Keyed(_, _) => {
                    todo!()
                }
            };
            Trace::new_lazy_call_input(trace_path, trace, data, db).into()
        })
        .collect()
}
