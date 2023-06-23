use super::*;

impl FluffyTerm {
    pub(crate) fn curry_destination(
        self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FluffyTerm {
        match self.data_inner(db, terms) {
            FluffyTermData::Literal(_)
            | FluffyTermData::TypeOntology { .. }
            | FluffyTermData::Hole(_, _)
            | FluffyTermData::Category(_) => self,
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }

    pub fn final_destination(self, engine: &impl FluffyTermEngine) -> FinalDestination {
        self.final_destination_inner(engine.db(), engine.fluffy_terms())
    }

    /// this term as ty, what's its final destination?
    ///
    pub(crate) fn final_destination_inner(
        self,
        db: &dyn FluffyTermDb,
        fluffy_terms: &FluffyTerms,
    ) -> FinalDestination {
        match self.data_inner(db, fluffy_terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology { .. } => FinalDestination::TypeOntology,
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(kind, idx) => match kind {
                HoleKind::UnspecifiedIntegerType
                | HoleKind::UnspecifiedFloatType
                | HoleKind::ImplicitType => FinalDestination::TypeOntology,
            },
            FluffyTermData::Category(_) => FinalDestination::Sort,
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }

    /// the count is always positive but returns i8 for convenience in computing difference
    /// -> i8 {v: v> 0}
    pub fn curry_parameter_count(self, engine: &impl FluffyTermEngine) -> i8 {
        match self.data(engine) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                ty_path,
                refined_ty_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => match ty_ethereal_term {
                Some(ty_ethereal_term) => ty_ethereal_term.curry_parameter_count(engine.db()),
                None => todo!(),
            },
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => 0,
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
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}

fn curry_destination(db: &dyn EtherealTermDb, term: EtherealTerm) -> EtherealTerm {
    match term {
        EtherealTerm::Literal(_) => todo!(),
        EtherealTerm::Symbol(_) => todo!(),
        EtherealTerm::Variable(_) => todo!(),
        EtherealTerm::EntityPath(path) => match path {
            TermEntityPath::Fugitive(_) => todo!(),
            TermEntityPath::Trait(_)
            | TermEntityPath::TypeOntology(_)
            | TermEntityPath::TypeInstance(_) => term,
        },
        // EntityPath::Module(_) => todo!(),
        // EntityPath::ModuleItem(path) => match path {
        //     ModuleItemPath::Type(path) => resolved_term,
        //     ModuleItemPath::Trait(_) => todo!(),
        //     ModuleItemPath::Form(_) => todo!(),
        // },
        // EntityPath::AssociatedItem(_) => todo!(),
        // EntityPath::Variant(_) => todo!(),
        EtherealTerm::Category(_) => term,
        EtherealTerm::Universe(_) => todo!(),
        EtherealTerm::Curry(_) => todo!(),
        EtherealTerm::Ritchie(_) => todo!(),
        EtherealTerm::Abstraction(_) => todo!(),
        EtherealTerm::Application(_) => term,
        EtherealTerm::Subentity(_) => todo!(),
        EtherealTerm::AsTraitSubentity(_) => todo!(),
        EtherealTerm::TraitConstraint(_) => todo!(),
    }
}
