use husky_token::TokenSheetData;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum RegionPath {
    Snippet(Toolchain),
    Decr(DecrId),
    Decl(EntityNodePath),
    Defn(EntityNodePath),
}

impl RegionPath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            RegionPath::Snippet(_) => todo!(),
            RegionPath::Decr(id) => id.module_path(db),
            RegionPath::Decl(path) => path.module_path(db),
            RegionPath::Defn(path) => path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn EntityTreeDb) -> Toolchain {
        self.module_path(db).toolchain(db)
    }

    pub fn token_sheet_data<'a>(self, db: &'a dyn EntityTreeDb) -> VfsResult<&'a TokenSheetData> {
        db.token_sheet_data(self.module_path(db))
    }
}

impl From<DecrId> for RegionPath {
    fn from(v: DecrId) -> Self {
        Self::Decr(v)
    }
}
