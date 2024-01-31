use super::*;
use vec_like::VecSet;

/// unlike DeclarativeTermSymbols
/// Some(DeclarativeTermRunes { unaccounted_variables: Default::default() })
/// means different from None
///
/// the former implies that variables exists, but all accounted
#[salsa::tracked(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermRunes {
    /// unaccounted means the variable is not declared within this term
    #[return_ref]
    pub unaccounted_variables: VecSet<RuneDeclarativeTerm>,
}

impl DeclarativeTermRunes {
    #[inline(always)]
    pub(crate) fn contains(self, db: &::salsa::Db, variable: RuneDeclarativeTerm) -> bool {
        self.unaccounted_variables(db).has(variable)
    }

    #[inline(always)]
    fn merge(fst: impl Into<Option<Self>>, snd: impl Into<Option<Self>>) -> Option<Self> {
        let fst: Option<_> = fst.into();
        let snd: Option<_> = snd.into();
        match (fst, snd) {
            (None, None) => None,
            (None, Some(snd)) => Some(snd),
            (Some(fst), None) => Some(fst),
            (Some(_fst), Some(_snd)) => todo!(),
        }
    }

    #[inline(always)]
    fn remove(
        variables: impl Into<Option<Self>>,
        _variable: impl Into<Option<RuneDeclarativeTerm>>,
    ) -> Option<Self> {
        let _variables = variables.into()?;
        todo!()
    }
}
impl DeclarativeTerm {
    pub fn contains_variable(self, db: &::salsa::Db, variable: RuneDeclarativeTerm) -> bool {
        self.variables(db)
            .map(|declarative_term_variables| declarative_term_variables.contains(db, variable))
            .unwrap_or_default()
    }

    pub(crate) fn variables(self, db: &::salsa::Db) -> Option<DeclarativeTermRunes> {
        match self {
            DeclarativeTerm::Literal(_) => todo!(),
            DeclarativeTerm::Rune(variable) => Some(DeclarativeTermRunes::new(
                db,
                VecSet::new_one_elem_set(variable),
            )),
            DeclarativeTerm::Symbol(_symbol) => None,
            DeclarativeTerm::EntityPath(path) => match path {
                ItemPathDeclarativeTerm::Fugitive(_) => todo!(),
                ItemPathDeclarativeTerm::Trait(_) | ItemPathDeclarativeTerm::Type(_) => None,
                ItemPathDeclarativeTerm::TypeVariant(_) => todo!(),
            },
            DeclarativeTerm::Category(_) => None,
            DeclarativeTerm::Universe(_) => None,
            DeclarativeTerm::Curry(declarative_term) => {
                declarative_term_curry_placeholders(db, declarative_term)
            }
            DeclarativeTerm::Ritchie(declarative_term) => {
                declarative_term_ritchie_variables(db, declarative_term)
            }
            DeclarativeTerm::Abstraction(_) => todo!(),
            DeclarativeTerm::Application(declarative_term) => {
                declarative_term_application_variables(db, declarative_term)
            }
            DeclarativeTerm::ApplicationOrRitchieCall(_declarative_ty) => todo!(),
            DeclarativeTerm::AssociatedItem(_) => todo!(),
            DeclarativeTerm::TypeAsTraitItem(_) => todo!(),
            DeclarativeTerm::TraitConstraint(_) => todo!(),
            DeclarativeTerm::LeashOrBitNot(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
            DeclarativeTerm::Wrapper(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_curry_placeholders(
    db: &::salsa::Db,
    term: CurryDeclarativeTerm,
) -> Option<DeclarativeTermRunes> {
    let parameter_ty_variables = term.parameter_ty(db).variables(db);
    let return_ty_variables = term.return_ty(db).variables(db);
    DeclarativeTermRunes::merge(
        parameter_ty_variables,
        DeclarativeTermRunes::remove(return_ty_variables, term.parameter_rune(db)),
    )
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_ritchie_variables(
    db: &::salsa::Db,
    term: RitchieDeclarativeTerm,
) -> Option<DeclarativeTermRunes> {
    let mut variables: Option<DeclarativeTermRunes> = None;
    for param in term.params(db) {
        variables = DeclarativeTermRunes::merge(variables, param.ty().variables(db))
    }
    DeclarativeTermRunes::merge(variables, term.return_ty(db).variables(db))
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_application_variables(
    db: &::salsa::Db,
    term: ApplicationDeclarativeTerm,
) -> Option<DeclarativeTermRunes> {
    DeclarativeTermRunes::merge(
        term.function(db).variables(db),
        term.argument(db).variables(db),
    )
}
