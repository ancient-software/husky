pub mod menu;

use crate::term::VdTerm;
use interned::db::{attached_interner_db, InternerDb};
use lisp_csv::expr::LpCsvExpr;
use smallvec::SmallVec;
use visored_entity_path::path::VdItemPath;

#[interned::interned]
pub struct VdInstantiation {
    pub path: VdItemPath,
    pub arguments: SmallVec<[VdTerm; 4]>,
}

impl std::fmt::Debug for VdInstantiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let db = attached_interner_db();
        write!(f, "{:?} ...", self.path(db))
    }
}

impl VdInstantiation {
    pub fn from_lp_csv_expr(expr: &LpCsvExpr, db: &InternerDb) -> Self {
        let (path, args) = expr.application_expansion();
        let path = VdItemPath::from_lp_csv_expr(path, db);
        let arguments = args
            .iter()
            .map(|arg| VdTerm::from_lp_csv_expr(arg, db))
            .collect();
        Self::new(path, arguments, db)
    }
}
