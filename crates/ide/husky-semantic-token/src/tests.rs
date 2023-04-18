pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_decl::DeclJar;
use husky_declarative_term::DeclarativeTermJar;
use husky_declarative_ty::DeclarativeTypeJar;
use husky_decr::DecrJar;
use husky_defn::DefnJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_ethereal_term::EtherealTermJar;
use husky_expr::ExprJar;
use husky_expr_ty::ExprTypeJar;
use husky_fluffy_term::FluffyTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_signature::DeclarativeSignatureJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
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
    DeclarativeTermJar,
    DeclarativeSignatureJar,
    DeclarativeTypeJar,
    EtherealTermJar,
    FluffyTermJar,
    ExprTypeJar,
    SemanticTokenJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn semantic_tokens_works() {
    DB::default().ast_expect_test_debug("semantic_tokens", |db, module_path| {
        SemanticTokenDb::semantic_tokens_ext(db, module_path, None)
    })
}
