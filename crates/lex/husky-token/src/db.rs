use crate::*;

use husky_vfs::*;

use salsa::DbWithJar;

pub trait TokenDb: DbWithJar<TokenJar> + VfsDb {
    fn token_sheet(&self, module_path: ModulePath) -> VfsResult<&TokenSheet>;
    fn tokenize_snippet(&self, snippet: Snippet) -> &TokenSheet;
}

impl<T> TokenDb for T
where
    T: DbWithJar<TokenJar> + VfsDb,
{
    fn token_sheet(&self, module_path: ModulePath) -> VfsResult<&TokenSheet> {
        Ok(token_sheet(self, module_path).as_ref()?)
    }

    fn tokenize_snippet(&self, snippet: Snippet) -> &TokenSheet {
        tokenize_snippet(self, snippet)
    }
}
