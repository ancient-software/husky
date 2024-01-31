use super::*;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermSymbols {
    #[return_ref]
    symbols: VecSet<SymbolDeclarativeTerm>,
}

impl DeclarativeTermSymbols {
    pub(crate) fn contains(self, db: &::salsa::Db, symbol: SymbolDeclarativeTerm) -> bool {
        self.symbols(db).has(symbol)
    }

    fn merge(
        db: &::salsa::Db,
        fst: impl Into<Option<Self>>,
        snd: impl Into<Option<Self>>,
    ) -> Option<Self> {
        let fst: Option<_> = fst.into();
        let snd: Option<_> = snd.into();
        match (fst, snd) {
            (None, None) => None,
            (None, Some(snd)) => Some(snd),
            (Some(fst), None) => Some(fst),
            (Some(fst), Some(snd)) => {
                let mut symbols = fst.symbols(db).clone();
                symbols.extend(snd.symbols(db).iter().copied());
                Some(DeclarativeTermSymbols::new(db, symbols))
            }
        }
    }
}
impl DeclarativeTerm {
    pub fn contains_symbol(self, db: &::salsa::Db, symbol: SymbolDeclarativeTerm) -> bool {
        calc_declarative_term_symbols(db, self)
            .map(|declarative_term_symbols| declarative_term_symbols.contains(db, symbol))
            .unwrap_or_default()
    }
}

fn calc_declarative_term_symbols(
    db: &::salsa::Db,
    declarative_term: DeclarativeTerm,
) -> Option<DeclarativeTermSymbols> {
    match declarative_term {
        DeclarativeTerm::Literal(_) => None,
        DeclarativeTerm::Symbol(symbol) => Some(DeclarativeTermSymbols::new(
            db,
            VecSet::new_one_elem_set(symbol),
        )),
        DeclarativeTerm::Rune(_) => None,
        DeclarativeTerm::EntityPath(path) => match path {
            ItemPathDeclarativeTerm::Fugitive(_) => todo!(),
            ItemPathDeclarativeTerm::Trait(_) | ItemPathDeclarativeTerm::Type(_) => None,
            ItemPathDeclarativeTerm::TypeVariant(_) => todo!(),
        },
        DeclarativeTerm::Category(_) => None,
        DeclarativeTerm::Universe(_) => None,
        DeclarativeTerm::Curry(declarative_term) => {
            declarative_term_curry_symbols(db, declarative_term)
        }
        DeclarativeTerm::Ritchie(declarative_term) => {
            declarative_term_ritchie_symbols(db, declarative_term)
        }
        DeclarativeTerm::Abstraction(_) => todo!(),
        DeclarativeTerm::Application(declarative_term) => {
            application_declarative_term_symbols(db, declarative_term)
        }
        DeclarativeTerm::ApplicationOrRitchieCall(_declarative_ty) => todo!(),
        DeclarativeTerm::AssociatedItem(_) => todo!(),
        DeclarativeTerm::TypeAsTraitAssociatedItem(_) => todo!(),
        DeclarativeTerm::TraitConstraint(_) => todo!(),
        DeclarativeTerm::LeashOrBitNot(_) => todo!(),
        DeclarativeTerm::List(_) => todo!(),
        DeclarativeTerm::Wrapper(_) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_curry_symbols(
    db: &::salsa::Db,
    declarative_term: CurryDeclarativeTerm,
) -> Option<DeclarativeTermSymbols> {
    let parameter_ty_symbols = calc_declarative_term_symbols(db, declarative_term.parameter_ty(db));
    let return_ty_symbols = calc_declarative_term_symbols(db, declarative_term.return_ty(db));
    DeclarativeTermSymbols::merge(db, parameter_ty_symbols, return_ty_symbols)
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_ritchie_symbols(
    db: &::salsa::Db,
    declarative_term: RitchieDeclarativeTerm,
) -> Option<DeclarativeTermSymbols> {
    let mut symbols: Option<DeclarativeTermSymbols> = None;
    for param in declarative_term.params(db) {
        symbols = DeclarativeTermSymbols::merge(
            db,
            symbols,
            calc_declarative_term_symbols(db, param.ty()),
        )
    }
    DeclarativeTermSymbols::merge(
        db,
        symbols,
        calc_declarative_term_symbols(db, declarative_term.return_ty(db)),
    )
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn application_declarative_term_symbols(
    db: &::salsa::Db,
    declarative_term: ApplicationDeclarativeTerm,
) -> Option<DeclarativeTermSymbols> {
    DeclarativeTermSymbols::merge(
        db,
        calc_declarative_term_symbols(db, declarative_term.function(db)),
        calc_declarative_term_symbols(db, declarative_term.argument(db)),
    )
}
