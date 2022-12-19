use crate::*;

use husky_dev_utils::dev_src;

use husky_entity_path::{EntityPath, EntityPathDb};
use husky_text::TextCharIter;
use husky_vfs::*;
use husky_word::WordDb;
use salsa::DbWithJar;

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(token_sheet);

pub trait TokenDb: DbWithJar<TokenJar> + VfsDb + EntityPathDb {
    fn token_sheet(&self, entity_path: EntityPath) -> &VfsResult<TokenSheet>;
}

impl<T> TokenDb for T
where
    T: DbWithJar<TokenJar> + VfsDb + EntityPathDb,
{
    fn token_sheet(&self, entity_path: EntityPath) -> &VfsResult<TokenSheet> {
        token_sheet(self, entity_path)
    }
}

#[salsa::tracked(jar = TokenJar,return_ref)]
fn token_sheet(db: &dyn TokenDb, entity_path: EntityPath) -> VfsResult<TokenSheet> {
    Ok(TokenSheet::new(tokenize::tokenize(
        db.word_db(),
        db.module_content(entity_path)?,
    )))
}
