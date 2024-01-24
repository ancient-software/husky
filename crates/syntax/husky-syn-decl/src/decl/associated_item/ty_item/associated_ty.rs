use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedTypeSynNodeDecl {
    pub fn errors(self, _db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        Default::default()
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedTypeSynDecl {
    #[id]
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub syn_expr_region: SynExprRegion,
}
