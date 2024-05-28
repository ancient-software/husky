use super::*;
use husky_entity_tree::node::impl_block::ill_formed_impl_block::IllFormedImplBlockSynNodePath;

#[salsa::tracked]
pub struct IllFormedImplBlockSynNodeDecl {
    #[id]
    pub syn_node_path: IllFormedImplBlockSynNodePath,
    pub syn_expr_region: SynExprRegion,
    // ad hoc
}

impl IllFormedImplBlockSynNodeDecl {
    pub fn errors(self, _db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        // ad hoc
        SmallVec::default()
    }
}

impl HasSynNodeDecl for IllFormedImplBlockSynNodePath {
    type NodeDecl = IllFormedImplBlockSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        ill_formed_impl_block_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ill_formed_impl_block_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> IllFormedImplBlockSynNodeDecl {
    let parser = ItemDeclParser::new(db, syn_node_path.into());
    parser.parse_ill_formed_impl_block_syn_node_decl(syn_node_path)
}

impl<'a> ItemDeclParser<'a> {
    fn parse_ill_formed_impl_block_syn_node_decl(
        &self,
        syn_node_path: IllFormedImplBlockSynNodePath,
    ) -> IllFormedImplBlockSynNodeDecl {
        let db = self.db();
        let parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::False, None);
        IllFormedImplBlockSynNodeDecl::new(db, syn_node_path, parser.finish())
    }
}
