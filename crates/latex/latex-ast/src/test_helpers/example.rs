use crate::{
    ast::{parse_latex_input_into_asts, LxAstArena, LxAstIdxRange},
    helpers::show::display_tree::LxAstDisplayTreeBuilder,
    range::{calc_ast_token_idx_range_map, LxAstTokenIdxRangeMap},
};
use latex_prelude::mode::LxMode;
use latex_token::storage::LxTokenStorage;

#[derive(Debug)]
pub struct LxAstExample {
    pub input: String,
    pub token_storage: LxTokenStorage,
    pub ast_arena: LxAstArena,
    pub asts: LxAstIdxRange,
    pub ast_token_idx_range_map: LxAstTokenIdxRangeMap,
}

impl LxAstExample {
    pub fn new(input: &str, root_mode: LxMode, db: &salsa::Db) -> Self {
        let mut ast_arena = LxAstArena::default();
        let mut token_storage = LxTokenStorage::default();
        let asts =
            parse_latex_input_into_asts(db, input, root_mode, &mut token_storage, &mut ast_arena);
        let ast_token_idx_range_map = calc_ast_token_idx_range_map(db, &ast_arena);
        Self {
            input: input.to_string(),
            token_storage,
            ast_arena,
            asts,
            ast_token_idx_range_map,
        }
    }
}

impl LxAstExample {
    pub fn show(&self, db: &salsa::Db) -> String {
        let display_tree_builder = LxAstDisplayTreeBuilder::new(
            db,
            &self.input,
            &self.token_storage,
            self.ast_arena.as_arena_ref(),
            &self.ast_token_idx_range_map,
        );
        format!(
            "{}",
            display_tree_builder
                .render_all(self.asts)
                .show(&Default::default())
        )
    }
}
