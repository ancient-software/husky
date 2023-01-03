mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VariantDefn {
    Unit(UnitVariantDefn),
    Tuple(TupleVariantDefn),
    Props(PropsVariantDefn),
}

impl VariantDefn {
    pub fn decl(self, db: &dyn DefnDb) -> Decl {
        todo!()
    }

    pub fn path(self, db: &dyn DefnDb) -> VariantPath {
        todo!()
    }
    pub fn expr_sheet(self, db: &dyn DefnDb) -> ExprSheet {
        todo!()
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for VariantDefn {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
