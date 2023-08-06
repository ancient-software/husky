use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DerefCoersion {
    Leashed,
}

impl ExpectCoersion {
    pub(super) fn resolve_deref(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        let (expectee_place, expectee_base_ty_data) = state.expectee().ty_data_inner(db, terms);
        // todo: check contract
        match expectee_base_ty_data {
            FluffyBaseTypeData::TypeOntology {
                refined_ty_path: Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)),
                ty_arguments: expectee_ty_arguments,
                ..
            } => {
                match prelude_indirection_ty_path {
                    PreludeIndirectionTypePath::Ref => todo!(),
                    PreludeIndirectionTypePath::RefMut => todo!(),
                    PreludeIndirectionTypePath::Leash => {
                        debug_assert_eq!(expectee_ty_arguments.len(), 1);
                        // todo: check expected_ty_argument_place
                        resolve_aux(
                            self.ty_expected,
                            expectee_ty_arguments[0],
                            |_, _| Some(Coersion::Deref(DerefCoersion::Leashed)),
                            db,
                            terms,
                            state,
                        )
                    }
                }
            }
            _ => AltNone,
        }
    }
}
