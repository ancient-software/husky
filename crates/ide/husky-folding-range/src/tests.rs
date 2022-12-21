use crate::*;

use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_token::TokenJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(WordJar, VfsJar, EntityPathJar, TokenJar, AstJar, FoldingRangeJar)]
#[derive(Default)]
pub struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn folding_ranges_works() {
    DB::expect_test_probable_modules_debug("folding_ranges", FoldingRangeDb::folding_ranges);
}
