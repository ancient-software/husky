use crate::{
    range::{ast_token_idx_range_sheet, AstTokenIdxRangeSheet},
    *,
};
use husky_token::TokenIdx;
use husky_vfs::path::module_path::ModulePath;

pub trait HasAstSheet: Copy {
    fn ast_sheet(self, db: &::salsa::Db) -> &AstSheet;
    fn ast_token_idx_range_sheet(self, db: &::salsa::Db) -> &AstTokenIdxRangeSheet;
}

impl HasAstSheet for ModulePath {
    fn ast_sheet(self, db: &::salsa::Db) -> &AstSheet {
        ast_sheet(db, self)
    }

    fn ast_token_idx_range_sheet(self, db: &::salsa::Db) -> &AstTokenIdxRangeSheet {
        ast_token_idx_range_sheet(db, self)
    }
}

#[salsa::tracked(jar = AstJar, return_ref)]
pub(crate) fn ast_sheet(db: &::salsa::Db, module_path: ModulePath) -> AstSheet {
    AstParser::new(db, module_path).parse_all()
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct AstSheet {
    ast_arena: AstArena,
    top_level_asts: AstIdxRange,
    nested_top_level_asts: Vec<(TokenIdx, AstIdxRange)>,
    // a list of siblings indices
    // list index has nothing to do with ast idx
    siblings: Vec<AstIdxRange>,
}

impl std::ops::Index<AstIdx> for AstSheet {
    type Output = AstData;

    fn index(&self, index: AstIdx) -> &Self::Output {
        &self.ast_arena[index]
    }
}

impl AstSheet {
    pub(crate) fn new(
        ast_arena: AstArena,
        top_level_asts: AstIdxRange,
        nested_top_level_asts: Vec<(TokenIdx, AstIdxRange)>,
        siblings: Vec<AstIdxRange>,
    ) -> Self {
        Self {
            ast_arena,
            nested_top_level_asts,
            top_level_asts,
            siblings,
        }
    }

    pub fn all_ast_indexed_iter<'a>(&'a self) -> impl Iterator<Item = (AstIdx, &'a AstData)> + 'a {
        self.ast_arena.indexed_iter()
    }

    pub fn indexed_iter<'a>(
        &'a self,
        ast_idx_range: AstIdxRange,
    ) -> impl Iterator<Item = (AstIdx, &'a AstData)> + 'a {
        ast_idx_range.into_iter().map(|idx| (idx, &self[idx]))
    }

    pub fn top_level_asts(&self) -> AstIdxRange {
        self.top_level_asts
    }

    pub fn top_level_asts_iter<'a>(&'a self) -> impl Iterator<Item = &'a AstData> + 'a {
        self.ast_arena[self.top_level_asts].iter()
    }

    pub fn top_level_asts_indexed_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (AstIdx, &'a AstData)> + 'a {
        self.ast_arena[&self.top_level_asts]
            .iter()
            .enumerate()
            .map(|(i, ast)| (self.top_level_asts.start() + i, ast))
    }

    pub fn nested_top_level_asts<'a>(&'a self) -> &'a [(TokenIdx, AstIdxRange)] {
        &self.nested_top_level_asts
    }

    pub fn siblings(&self) -> &[AstIdxRange] {
        &self.siblings
    }
}

impl std::ops::Deref for AstSheet {
    type Target = AstArena;

    fn deref(&self) -> &Self::Target {
        &self.ast_arena
    }
}
