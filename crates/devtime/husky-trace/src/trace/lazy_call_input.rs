use husky_hir_lazy_expr::HirLazyExprIdx;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LazyCallInputTracePathData {
    biological_parent_path: TracePath,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LazyCallInputTraceData {
    path: TracePath,
    biological_parent: Trace,
    input_sketch: LazyCallInputSketch,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyCallInputSketch {
    Regular {
        sema_expr_idx: SemaExprIdx,
        hir_lazy_expr_idx: Option<HirLazyExprIdx>,
    },
    Variadic,
    Keyed,
}

impl Trace {
    pub(crate) fn new_lazy_call_input(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        input_sketch: LazyCallInputSketch,
        db: &::salsa::Db,
    ) -> Self {
        let path = TracePath::new(
            LazyCallInputTracePathData {
                biological_parent_path: biological_parent_path.into(),
            },
            db,
        );
        Trace::new(
            path,
            LazyCallInputTraceData {
                path,
                biological_parent: biological_parent.into(),
                input_sketch,
            }
            .into(),
            db,
        )
    }
}

impl LazyCallInputTraceData {
    pub(super) fn view_lines(&self, _db: &::salsa::Db) -> TraceViewLines {
        todo!()
    }

    pub(super) fn have_subtraces(&self) -> bool {
        false
    }

    pub(super) fn subtraces(&self) -> Vec<Trace> {
        vec![]
    }

    pub(super) fn ki_repr(&self, _db: &::salsa::Db) -> KiRepr {
        todo!()
    }
}
