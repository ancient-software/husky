pub(crate) use husky_ast::test_utils::*;

use crate::{jar::*, *};
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_dec_signature::DecSignatureJar;
use husky_entity_tree::{helpers::paths::module_item_paths, EntityTreeJar};
use husky_eth_signature::EtherealSignatureJar;
use husky_eth_term::EthTermJar;
use husky_fly_term::FlyTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_sema_expr::SemaExprJar;
use husky_syn_decl::SynDeclJar;
use husky_syn_defn::SynDefnJar;
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::jar::TomlTokenJar;

#[salsa::db(
    CowordJar,
    VfsJar,
    husky_entity_path::jar::EntityPathJar,
    husky_token_data::jar::TokenDataJar,
    TokenJar,
    husky_ast::jar::AstJar,
    EntityTreeJar,
    husky_toml_token::jar::TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynExprJar,
    SynDefnJar,
    SynDeclJar,
    TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    DecSignatureJar,
    husky_dec_ty::jar::DeclarativeTypeJar,
    EthTermJar,
    EtherealSignatureJar,
    FlyTermJar,
    SemaExprJar,
    husky_sema_place_contract::jar::SemaPlaceContractJar,
    husky_hir_ty::db::HirTypeJar,
    husky_hir_eager_expr::jar::HirEagerExprJar,
    husky_hir_lazy_expr::jar::HirLazyExprJar,
    husky_hir_expr::jar::HirExprJar,
    husky_hir_decl::jar::HirDeclJar,
    HirDefnJar
)]
#[derive(Default)]
pub(crate) struct DB;

pub(crate) fn module_hir_defns(db: &::salsa::Db, module_path: ModulePath) -> Vec<HirDefn> {
    module_item_paths(db, module_path)
        .iter()
        .filter_map(|path| path.hir_defn(db))
        .collect()
}

#[test]
fn module_hir_defns_works() {
    DB::ast_expect_test_debug_with_db(
        module_hir_defns,
        &AstTestConfig::new(
            "module_hir_defns",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::HIR,
        ),
    );
}
