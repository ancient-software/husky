use super::*;

/// source
///
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum HoleSource {
    Expr(SynExprIdx),
    Expectation(FluffyTermExpectationIdx),
}

impl HoleSource {
    pub fn expr_idx(self) -> SynExprIdx {
        todo!()
        // self.expr_idx
    }
}

/// kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HoleKind {
    UnspecifiedIntegerType,
    UnspecifiedFloatType,
    ImplicitType,
    Any,
}

impl HollowTerm {
    pub(crate) fn new(engine: &mut impl FluffyTermEngine, data: HollowTermData) -> Self {
        engine
            .fluffy_term_region_mut()
            .hollow_terms_mut()
            .alloc_new(data)
    }

    pub(crate) fn new_hole(
        engine: &mut impl FluffyTermEngine,
        src: impl Into<HoleSource>,
        hole_kind: HoleKind,
    ) -> Self {
        Self::new(
            engine,
            HollowTermData::Hole {
                hole_source: src.into(),
                hole_kind,
                fill: None,
                constraints: smallvec![],
            },
        )
    }
}
