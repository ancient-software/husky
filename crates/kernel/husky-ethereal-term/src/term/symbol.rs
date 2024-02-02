mod index;
mod set;

pub use self::index::*;
pub use self::set::*;

use super::*;

use thiserror::Error;

#[salsa::interned(db = EthTermDb, jar = EthTermJar, constructor = pub new_inner)]
pub struct SymbolEthTerm {
    pub toolchain: Toolchain,
    pub ty: EthTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    /// todo: improve this by adding TypeFamily
    pub index: EthTermSymbolIndex,
}

#[test]
fn term_symbol_size_works() {
    assert_eq!(
        std::mem::size_of::<SymbolEthTerm>(),
        std::mem::size_of::<u32>()
    );
}

impl SymbolEthTerm {
    #[inline(always)]
    pub fn from_declarative(
        db: &::salsa::Db,
        declarative_term_symbol: SymbolDeclarativeTerm,
    ) -> EthTermResult<Self> {
        let ty = declarative_term_symbol.ty(db)?;
        let ty = EthTerm::ty_from_declarative(db, ty)?;
        Ok(Self::new_inner(
            db,
            declarative_term_symbol.toolchain(db),
            ty,
            EthTermSymbolIndex::from_declarative(declarative_term_symbol.index(db)),
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

impl salsa::DisplayWithDb for SymbolEthTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_fmt(format_args!("${:?}", self.index(db)))
    }
}

impl EthTermInstantiate for SymbolEthTerm {
    type Output = EthTerm;

    fn instantiate(self, _db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        // it's assumed that all symbols will be replaced by its map
        // otherwise it's illegal
        instantiation.symbol_instantiation(self)
    }
}
