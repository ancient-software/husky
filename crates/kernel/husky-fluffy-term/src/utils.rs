use super::*;

impl FluffyTerm {
    pub(crate) fn curry_destination(self, db: &::salsa::Db, terms: &FluffyTerms) -> FluffyTerm {
        match self.data_inner(db, terms) {
            FluffyTermData::TypeOntology { .. }
            | FluffyTermData::Hole(_, _)
            | FluffyTermData::Category(_)
            | FluffyTermData::Ritchie { .. }
            | FluffyTermData::Symbol { .. }
            | FluffyTermData::Rune { .. } => self,
            FluffyTermData::Curry { return_ty, .. } => return_ty.curry_destination(db, terms),
            FluffyTermData::Literal(_) | FluffyTermData::TypeVariant { .. } => unreachable!(),
        }
    }

    pub fn final_destination(self, engine: &impl FluffyTermEngine) -> FinalDestination {
        self.final_destination_inner(engine.db(), engine.fluffy_terms())
    }

    /// this term as ty, what's its final destination?
    ///
    pub(crate) fn final_destination_inner(
        self,
        db: &::salsa::Db,
        fluffy_terms: &FluffyTerms,
    ) -> FinalDestination {
        match self.data_inner(db, fluffy_terms) {
            FluffyTermData::TypeOntology { .. } => FinalDestination::TypeOntology,
            FluffyTermData::Curry { return_ty, .. } => {
                return_ty.final_destination_inner(db, fluffy_terms)
            }
            FluffyTermData::Hole(kind, idx) => match kind {
                HoleKind::UnspecifiedIntegerType
                | HoleKind::UnspecifiedFloatType
                | HoleKind::ImplicitType => FinalDestination::TypeOntology,
                HoleKind::Any => FinalDestination::AnyOriginal,
            },
            FluffyTermData::Category(_) => FinalDestination::Sort,
            FluffyTermData::Ritchie { ritchie_kind, .. } => FinalDestination::Ritchie(ritchie_kind),
            FluffyTermData::Symbol { .. } | FluffyTermData::Rune { .. } => {
                FinalDestination::AnyOriginal
            }
            FluffyTermData::Literal(_) | FluffyTermData::TypeVariant { .. } => unreachable!(),
        }
    }

    /// the count is always positive but returns i8 for convenience in computing difference
    /// -> i8 {v: v> 0}
    ///
    /// todo: include ritchie??
    pub fn curry_parameter_count(self, engine: &impl FluffyTermEngine) -> i8 {
        self.curry_parameter_count_inner(engine.db(), engine.fluffy_terms())
    }

    pub fn curry_parameter_count_inner(self, db: &::salsa::Db, fluffy_terms: &FluffyTerms) -> i8 {
        match self.data_inner(db, fluffy_terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                ty_arguments: arguments,
                ty_ethereal_term,
            } => 0,
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => match ty_ethereal_term {
                Some(ty_ethereal_term) => ty_ethereal_term.curry_parameter_count(db),
                None => todo!(),
            },
            FluffyTermData::Hole(hole_kind, _) => 0,
            FluffyTermData::Category(_) => 0,
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => 0,
            FluffyTermData::Symbol { .. } | FluffyTermData::Rune { .. } => 0,
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}

fn curry_destination(db: &::salsa::Db, term: EtherealTerm) -> EtherealTerm {
    match term {
        EtherealTerm::Literal(_) => todo!(),
        EtherealTerm::Symbol(_) => todo!(),
        EtherealTerm::Rune(_) => todo!(),
        EtherealTerm::EntityPath(path) => match path {
            TermEntityPath::Fugitive(_) => todo!(),
            TermEntityPath::Trait(_)
            | TermEntityPath::TypeOntology(_)
            | TermEntityPath::TypeInstance(_) => term,
            TermEntityPath::TypeVariant(_) => todo!(),
        },
        EtherealTerm::Category(_) => term,
        EtherealTerm::Universe(_) => todo!(),
        EtherealTerm::Curry(_) => todo!(),
        EtherealTerm::Ritchie(_) => todo!(),
        EtherealTerm::Abstraction(_) => todo!(),
        EtherealTerm::Application(_) => term,
        EtherealTerm::Subitem(_) => todo!(),
        EtherealTerm::AsTraitSubitem(_) => todo!(),
        EtherealTerm::TraitConstraint(_) => todo!(),
    }
}
