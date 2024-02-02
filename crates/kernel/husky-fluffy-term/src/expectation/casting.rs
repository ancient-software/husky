use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db]
pub struct ExpectCasting {
    pub(crate) destination: FlyTerm,
}

impl ExpectCasting {
    pub fn new(destination: FlyTerm) -> Self {
        Self { destination }
    }

    pub(crate) fn try_substitute_unresolved_fluffy_term<'a>(
        &self,
        fluffy_terms: &'a FlyTerms,
    ) -> Result<Option<Expectation>, &'a HollowTermResolveError> {
        todo!()
        // match fluffy_terms.try_reduce_fluffy_term(self.destination)? {
        //     Some(destination) => Ok(Some(ExpectExplicitlyConvertible { destination }.into())),
        //     None => Ok(None),
        // }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db]
pub struct ExpectExplicitlyConvertibleOutcome {
    destination: FlyTerm,
}

impl ExpectFlyTerm for ExpectCasting {
    type Outcome = ExpectExplicitlyConvertibleOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::ExplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        todo!()
    }

    #[inline(always)]
    fn destination(&self) -> Option<FlyTerm> {
        Some(self.destination)
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        // todo
        AltOption::AltNone
    }
}
