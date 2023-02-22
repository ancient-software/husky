use crate::*;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermAbstraction {
    x: TermSymbol,
    m: Term,
}

impl TermAbstraction {
    pub fn ty(&self) -> Term {
        todo!()
    }

    // pub fn universe(&self) -> TermUniverseLevel {
    //     todo!()
    // }
}

impl TermRewriteCopy for TermAbstraction {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}

impl From<TermAbstraction> for Term {
    fn from(val: TermAbstraction) -> Self {
        Term::Abstraction(val)
    }
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for TermAbstraction {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char(husky_unicode_symbols::greek::GREEK_LETTER_LOWERCASE_LAMBDA);
        todo!()
    }
}
