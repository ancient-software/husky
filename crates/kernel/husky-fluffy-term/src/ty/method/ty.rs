use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyTypeMethodDisambiguation {
    indirections: SmallVec<[FluffyMethodIndirection; 2]>,
    ty_path: TypePath,
}

impl FluffyTypeMethodDisambiguation {
    pub fn indirections(&self) -> &[FluffyMethodIndirection] {
        &self.indirections
    }

    pub fn ty_path(&self) -> TypePath {
        self.ty_path
    }
}

impl FluffyTerm {
    pub(super) fn ty_method_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
    ) -> FluffyTypeResult<Option<(FluffyTypeMethodDisambiguation, FluffyTypeResult<FluffyTerm>)>>
    {
        let mut indirections = smallvec![];
        let Some((ty_path, ty_result)) = self.ty_method_ty_aux(engine,ident,&mut indirections)? else {
            return Ok(None)
        };
        Ok(Some((
            FluffyTypeMethodDisambiguation {
                indirections,
                ty_path,
            },
            ty_result,
        )))
    }

    pub(super) fn ty_method_ty_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        indirections: &mut SmallVec<[FluffyMethodIndirection; 2]>,
    ) -> FluffyTypeResult<Option<(TypePath, FluffyTypeResult<FluffyTerm>)>> {
        match self.data(engine) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments,
                ..
            } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                arguments,
                ..
            } => match refined_path {
                Right(PreludeTypePath::Borrow(_)) => todo!(),
                _ => match self.ty_method_card(engine, ident)? {
                    Some(_) => todo!(),
                    None => {
                        // todo: `Deref` and `Carrier`
                        Ok(None)
                    },
                }
                // match base_ty_term {
                //     Some(base_ty_term) => todo!(),
                //     None => todo!(),
                // },
            },
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }
}
