use super::*;

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct DefnTokraRegion {
    #[id]
    syn_node_path: ItemSynNodePath,
    #[return_ref]
    tokens: Vec<Token>,
}

impl DefnTokraRegion {
    pub fn data<'a>(self, db: &'a dyn EntitySynTreeDb) -> DefnTokraRegionData<'a> {
        DefnTokraRegionData {
            tokens: self.tokens(db),
        }
    }
}

pub struct DefnTokraRegionData<'a> {
    tokens: &'a [Token],
}

impl<'a> std::ops::Index<RegionalTokenIdx> for DefnTokraRegionData<'a> {
    type Output = Token;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens[idx.index()]
    }
}

pub(super) fn defn_token_region(
    syn_node_path: ItemSynNodePath,
    db: &dyn EntitySynTreeDb,
) -> Option<DefnTokraRegion> {
    match syn_node_path {
        ItemSynNodePath::Submodule(_) => None,
        ItemSynNodePath::MajorItem(_) => todo!(),
        ItemSynNodePath::TypeVariant(_) => todo!(),
        ItemSynNodePath::ImplBlock(_) => todo!(),
        ItemSynNodePath::AssociatedItem(_) => todo!(),
        ItemSynNodePath::Decr(_) => todo!(),
    }
}

pub enum DefnAst {}

pub type DefnAstArena = Arena<DefnAst>;
pub type DefnAstArenaIdx = ArenaIdx<DefnAst>;

pub struct DefnTokraRegionSourceMap {
    token_region_base: TokenRegionBase,
}
