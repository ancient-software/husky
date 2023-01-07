use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct UnitVariantDecl {
    #[id]
    pub path: VariantPath,
    pub expr_sheet: ExprSheet,
}
