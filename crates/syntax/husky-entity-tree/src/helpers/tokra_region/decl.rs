use super::*;
use husky_decl_ast::DeclAst;
use husky_token::{TokenDb, TokenIdxRange};

///
#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct DeclTokraRegion {
    #[return_ref]
    tokens_data: Vec<TokenData>,
    pub saved_regional_token_stream_state: Option<RegionalTokenStreamState>,
    pub ast: DeclAst,
}

#[derive(Debug, Clone, Copy)]
pub struct DeclTokraRegionDataRef<'a> {
    saved_regional_token_stream_state: Option<RegionalTokenStreamState>,
    tokens_data: &'a [TokenData],
}

impl DeclTokraRegion {
    pub fn data<'a>(self, db: &'a ::salsa::Db) -> DeclTokraRegionDataRef<'a> {
        DeclTokraRegionDataRef {
            tokens_data: self.tokens_data(db),
            saved_regional_token_stream_state: self.saved_regional_token_stream_state(db),
        }
    }

    pub fn regional_tokens_data<'a>(self, db: &'a ::salsa::Db) -> RegionalTokensData<'a> {
        RegionalTokensData::new(self.tokens_data(db))
    }
}

impl<'a> DeclTokraRegionDataRef<'a> {
    pub fn regional_token_stream(self) -> RegionalTokenStream<'a> {
        RegionalTokenStream::new_decl_regional_token_stream(
            self.tokens_data,
            self.saved_regional_token_stream_state,
        )
    }
}

impl<'a> std::ops::Index<RegionalTokenIdx> for DeclTokraRegionDataRef<'a> {
    type Output = TokenData;

    fn index(&self, idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens_data[idx.index()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclTokraRegionSourceMap {
    regional_token_idx_base: RegionalTokenIdxBase,
    ast_idx: AstIdx,
}

impl DeclTokraRegionSourceMap {
    pub fn regional_token_idx_base(&self) -> RegionalTokenIdxBase {
        self.regional_token_idx_base
    }

    pub fn ast_idx(&self) -> ArenaIdx<AstData> {
        self.ast_idx
    }
}

fn build_decl_tokra_region(
    module_path: ModulePath,
    opt_ast_idx: Option<AstIdx>,
    db: &::salsa::Db,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    let token_sheet_data = db.token_sheet_data(module_path);
    let ast_sheet = module_path.ast_sheet(db);
    let Some(ast_idx) = opt_ast_idx else { todo!() };
    let (token_verse_idx, ast, saved_regional_stream_state) = match ast_sheet[ast_idx] {
        AstData::Attr {
            token_verse_idx,
            ident: _,
        } => (token_verse_idx, DeclAst::Attr, None),
        AstData::Identifiable {
            token_verse_idx,
            visibility_expr: _,
            item_kind: _,
            ident_token: _,
            is_generic: _,
            saved_stream_state,
            block: _,
        } => (
            token_verse_idx,
            DeclAst::Identifiable {},
            Some(saved_stream_state),
        ),
        AstData::TypeVariant {
            token_verse_idx,
            variant_path: _,
            vertical_token: _,
            ident_token: _,
            saved_stream_state,
        } => (
            token_verse_idx,
            DeclAst::TypeVariant,
            Some(saved_stream_state),
        ),
        AstData::ImplBlock {
            token_verse_idx,
            items: _,
        } => (token_verse_idx, DeclAst::ImplBlock, None),
        _ => unreachable!(),
    };
    let tokens = token_sheet_data[token_verse_idx].to_vec();
    let regional_token_idx_base =
        RegionalTokenIdxBase::new(token_sheet_data.token_verse_start(token_verse_idx));
    let saved_regional_stream_state = saved_regional_stream_state.map(|token_stream_state| {
        RegionalTokenStreamState::from_token_stream_state(
            token_stream_state,
            regional_token_idx_base,
        )
    });
    let decl_tokra_region =
        DeclTokraRegion::new_inner(db, tokens, saved_regional_stream_state, ast);
    let decl_tokra_region_source_map = DeclTokraRegionSourceMap {
        regional_token_idx_base,
        ast_idx,
    };
    (decl_tokra_region, decl_tokra_region_source_map)
}

impl ItemSynNodePathId {
    pub fn decl_tokra_region(self, db: &::salsa::Db) -> DeclTokraRegion {
        item_syn_node_decl_tokra_region_with_source_map(db, self).0
    }

    // use this only when necessary
    pub fn decl_tokra_region_source_map(self, db: &::salsa::Db) -> DeclTokraRegionSourceMap {
        item_syn_node_decl_tokra_region_with_source_map(db, self).1
    }

    pub fn decl_ast_idx(self, db: &::salsa::Db) -> AstIdx {
        self.decl_tokra_region_source_map(db).ast_idx()
    }

    pub fn decl_regional_token_idx_base(self, db: &::salsa::Db) -> RegionalTokenIdxBase {
        self.decl_tokra_region_source_map(db)
            .regional_token_idx_base
    }

    pub fn decl_tokra_region_token_idx_range(self, db: &::salsa::Db) -> TokenIdxRange {
        let decl_tokra_region = self.decl_tokra_region(db);
        let decl_tokra_region_source_map = self.decl_tokra_region_source_map(db);
        let start = decl_tokra_region_source_map
            .regional_token_idx_base
            .index_base();
        let end = start + decl_tokra_region.regional_tokens_data(db).len();
        unsafe {
            TokenIdxRange::new(
                TokenIdx::from_usize_index_ext(start),
                TokenIdx::from_usize_index_ext(end),
            )
        }
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
fn item_syn_node_decl_tokra_region_with_source_map(
    db: &::salsa::Db,
    id: ItemSynNodePathId,
) -> (DeclTokraRegion, DeclTokraRegionSourceMap) {
    build_decl_tokra_region(id.module_path(db), id.opt_ast_idx(db), db)
}
