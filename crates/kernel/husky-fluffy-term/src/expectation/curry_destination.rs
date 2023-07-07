use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectCurryDestination {
    curry_destination: FluffyTerm,
}

impl ExpectCurryDestination {
    pub fn new(curry_destination: FluffyTerm) -> Self {
        Self { curry_destination }
    }
}

impl ExpectFluffyTerm for ExpectCurryDestination {
    type Outcome = ();

    #[inline(always)]
    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        &()
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        fluffy_terms: &FluffyTerms,
    ) -> FinalDestination {
        self.curry_destination
            .final_destination_inner(db, fluffy_terms)
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        meta: &mut ExpectationMeta,
    ) -> Option<ExpectationEffect> {
        None
    }
}
