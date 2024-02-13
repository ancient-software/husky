mod adv;

use crate::*;
use husky_coword::CowordJar;
use husky_expect_test_snippets_utils::*;
use husky_vfs::{snippet::Snippet, *};
use salsa::DebugWithDb;

#[salsa::db(
    CowordJar,
    VfsJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    TermPreludeJar,
    husky_entity_path::jar::EntityPathJar
)]
#[derive(Default)]
pub(crate) struct DB;

fn tokenize_snippet_debug(snippet: &str) -> String {
    let db = DB::default();
    let db = &*db;
    let snippet = Snippet::new(
        db,
        Ident::from_ref(db, "tokenize_snippet_debug").unwrap(),
        snippet.to_owned(),
    );
    format!("{:#?}", db.snippet_token_sheet_data(snippet).debug(db))
}

#[test]
fn tokenize_works() {
    expect_test_snippets("snippets", &tokenize_snippet_debug);
}

#[test]
fn token_sheet_works() {
    DB::vfs_expect_test_debug_with_db(
        |db, module_path| db.token_sheet_data(module_path),
        &VfsTestConfig::new(
            "token_sheet",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    )
}
