pub(crate) use husky_ast::test_utils::{AstTestConfig, AstTestUtils};
pub(crate) use husky_vfs::ModulePath;

use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_declarative_signature::DecSignatureJar;
use husky_declarative_term::DeclarativeTermJar;
use husky_entity_tree::EntityTreeJar;
use husky_ethereal_signature::EtherealSignatureJar;
use husky_ethereal_term::EthTermJar;
use husky_fluffy_term::FluffyTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_sema_expr::SemaExprJar;
use husky_syn_decl::SynDeclJar;
use husky_syn_defn::SynDefnJar;
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::VfsJar;

#[salsa::db(
    CowordJar,
    VfsJar,
   
 husky_entity_path::jar::EntityPathJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    husky_ast::jar::AstJar,
    EntityTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynExprJar,
    SynDefnJar,
    SynDeclJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DecSignatureJar,
    husky_declarative_ty::db::DeclarativeTypeJar,
    EthTermJar,
    EtherealSignatureJar,
    FluffyTermJar,
    SemaExprJar,
    husky_hir_ty::db::HirTypeJar,
    husky_hir_eager_expr::db::HirEagerExprJar,
    husky_hir_lazy_expr::db::HirLazyExprJar,
    husky_hir_expr::db::HirExprJar,
    husky_hir_decl::db::HirDeclJar,
    husky_hir_defn::db::HirDefnJar,
    // linkage
    husky_linkage::jar::LinkageJar,
    husky_javelin::jar::JavelinJar,
    // val
    husky_val::jar::ValJar,
    husky_val_repr::jar::ValReprJar,
    // ide
    husky_token_info::db::TokenInfoJar,
    // lex
    husky_text::db::TextJar,
    // devtime
    crate::db::TraceJar,
)]
#[derive(Default)]
pub struct DB;
