pub(crate) use crate::test_utils::*;

use crate::*;
use husky_coword::CowordJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_vfs::*;

#[salsa::db(
    CowordJar,
    VfsJar,
    husky_entity_path::jar::EntityPathJar,
    husky_token_data::jar::TokenDataJar,
    TokenJar,
    crate::jar::AstJar,
    TermPreludeJar
)]
pub(crate) struct DB;

#[test]
fn ast_sheet_works() {
    DB::token_expect_test_debug_with_db(
        |db, module_path: ModulePath| module_path.ast_sheet(db),
        &TokenTestConfig::new(
            "ast_sheet",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    );
}
