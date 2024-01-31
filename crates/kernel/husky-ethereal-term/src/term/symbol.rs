mod index;
mod set;

pub use self::index::*;
pub use self::set::*;

use super::*;

use thiserror::Error;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = pub new_inner)]
pub struct SymbolEtherealTerm {
    pub toolchain: Toolchain,
    pub ty: EtherealTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    /// todo: improve this by adding TypeFamily
    pub index: EtherealTermSymbolIndex,
}

#[test]
fn term_symbol_size_works() {
    assert_eq!(
        std::mem::size_of::<SymbolEtherealTerm>(),
        std::mem::size_of::<u32>()
    );
}

impl SymbolEtherealTerm {
    #[inline(always)]
    pub fn from_declarative(
        db: &::salsa::Db,
        declarative_term_symbol: SymbolDeclarativeTerm,
    ) -> EtherealTermResult<Self> {
        let ty = declarative_term_symbol.ty(db)?;
        let ty = EtherealTerm::ty_from_declarative(db, ty)?;
        Ok(Self::new_inner(
            db,
            declarative_term_symbol.toolchain(db),
            ty,
            EtherealTermSymbolIndex::from_declarative(declarative_term_symbol.index(db)),
        ))
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_symbol(db, self, f)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermSymbolTypeErrorKind {
    #[error("signature term error")]
    SignatureTermError,
    #[error("sketch term error")]
    SketchTermError,
}
pub type TermSymbolTypeResult<T> = Result<T, TermSymbolTypeErrorKind>;

impl salsa::DisplayWithDb for SymbolEtherealTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_fmt(format_args!("${:?}", self.index(db)))
    }
}

impl EtherealInstantiate for SymbolEtherealTerm {
    type Output = EtherealTerm;

    fn instantiate(self, _db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        /// it's assumed that all symbols will be replaced by its map
        /// otherwise it's illegal
        instantiation.symbol_instantiation(self)
    }
}
