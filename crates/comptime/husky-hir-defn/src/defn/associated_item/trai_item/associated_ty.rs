use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeHirDefn {
    pub path: TraitItemPath,
    pub hir_decl: TraitAssociatedTypeHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}
