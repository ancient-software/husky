use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct RecordTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    pub hir_expr_region: HirEagerExprRegion,
}
