pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_decl::DeclJar;
use husky_decr::DecrJar;
use husky_defn::DefnJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::{EntityTreeJar, EntityTreeResult};
use husky_expr::ExprJar;
use husky_expr_ty::ExprTypeJar;
use husky_fluffy_term::FluffyTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_term::TermJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::{TokenDb, TokenIdx, TokenJar};
use husky_token_info::TokenInfoJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_ty::TypeJar;
use husky_word::WordJar;

#[salsa::db(
    VfsJar,
    WordJar,
    TokenJar,
    TokenInfoJar,
    EntityPathJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    AstJar,
    EntityTreeJar,
    DeclJar,
    DecrJar,
    DefnJar,
    ExprJar,
    TermPreludeJar,
    RawTermJar,
    SignatureJar,
    RawTypeJar,
    TermJar,
    TypeJar,
    FluffyTermJar,
    ExprTypeJar,
    HoverJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn hover_result_works() {
    const N: usize = 20;
    DB::default().ast_expect_test_debug(
        "hover_result",
        |db, module_path| -> EntityTreeResult<Vec<(TokenIdx, Option<HoverResult>)>> {
            let ranged_token_sheet = db.ranged_token_sheet(module_path)?;
            let len = ranged_token_sheet.len();
            let step = (len / N).max(1);
            let mut hover_results = vec![];
            for token_idx in ranged_token_sheet.token_index_iter() {
                // only push some of them, but all of them have to be computed
                let hover_result = calc_hover_result(db, module_path, token_idx)?;
                if token_idx.raw() % step == 0 {
                    hover_results.push((token_idx, hover_result))
                }
            }
            Ok(hover_results)
        },
    )
}
