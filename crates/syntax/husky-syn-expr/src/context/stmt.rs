use super::*;
use husky_defn_ast::{DefnAstArenaRef, DefnAstIdx, DefnAstIdxRange};
use husky_entity_tree::helpers::tokra_region::DefnTokraRegionData;

pub struct SynStmtContext<'a> {
    expr_context: SynExprContext<'a>,
    defn_tokra_region_data: DefnTokraRegionData<'a>,
}

impl<'a> std::ops::Deref for SynStmtContext<'a> {
    type Target = SynExprContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.expr_context
    }
}

impl<'a> std::ops::DerefMut for SynStmtContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.expr_context
    }
}

impl<'a> SynStmtContext<'a> {
    pub fn new(
        syn_node_path: ItemSynNodePath,
        decl_expr_region: SynExprRegion,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
        db: &'a ::salsa::Db,
    ) -> Option<Self> {
        let module_path = syn_node_path.module_path(db);
        let expr_context = SynExprContext::new(
            db,
            SynNodeRegionPath::Defn(syn_node_path),
            db.module_symbol_context(module_path).unwrap(),
            Some(decl_expr_region),
            allow_self_type,
            allow_self_value,
        );
        Some(Self {
            expr_context,
            defn_tokra_region_data: syn_node_path.defn_tokra_region(db)?.data(db),
        })
    }

    pub(crate) fn expr_parser<'b>(
        &'b mut self,
        token_verse_idx: RegionalTokenVerseIdx,
    ) -> SynExprParser<'a, &'b mut SynExprContext<'a>>
    where
        'a: 'b,
    {
        let token_stream = self.token_verse_token_stream(token_verse_idx);
        SynExprParser::new(self, None, token_stream)
    }

    pub(crate) fn parse_root_body(&mut self) -> SynExprIdx {
        let body = self.defn_tokra_region_data.root_body();
        let stmts = self.parse_stmts(body);
        let expr = SynExprData::Block { stmts };
        let expr = self.alloc_expr(expr);
        self.expr_context
            .add_expr_root(SynExprRootKind::BlockExpr, expr);
        expr
    }

    pub fn finish(self) -> SynExprRegion {
        self.expr_context.finish()
    }

    pub(crate) fn token_verse_token_stream(
        &self,
        regional_token_verse_idx: RegionalTokenVerseIdx,
    ) -> RegionalTokenStream<'a> {
        self.defn_tokra_region_data
            .token_stream(regional_token_verse_idx)
    }

    pub(crate) fn asts(&self) -> DefnAstArenaRef<'a> {
        self.defn_tokra_region_data.ast_arena()
    }

    pub(crate) fn ast_token_idx_range(&self, defn_ast_idx: DefnAstIdx) -> RegionalTokenIdxRange {
        self.defn_tokra_region_data
            .ast_token_idx_range(defn_ast_idx)
    }

    pub(crate) fn fugitive_body_end(&self, body: DefnAstIdxRange) -> RegionalTokenIdxRangeEnd {
        self.defn_tokra_region_data
            .ast_token_idx_range(body.end() - 1)
            .end()
    }

    pub fn defn_tokra_region_data(&self) -> DefnTokraRegionData<'a> {
        self.defn_tokra_region_data
    }
}
