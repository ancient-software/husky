use std::iter::Peekable;

use crate::{
    builder::{ToVdSyn, VdSynExprBuilder},
    sentence::{VdSynSentenceIdx, VdSynSentenceIdxRange},
};
use husky_coword::Coword;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::rose::{LxRoseAstData, LxRoseAstIdx, LxRoseAstIdxRange};
use latex_token::idx::{LxRoseTokenIdx, LxTokenIdxRange};

pub enum VdSynStmtData {
    Paragraph(VdSynSentenceIdxRange),
    Block {
        environment: (),
        stmts: VdSynStmtIdxRange,
    },
}

pub enum VdSynStmtChild {
    Sentence(VdSynSentenceIdx),
    Stmt(VdSynStmtIdx),
}

impl VdSynStmtData {
    pub(crate) fn children(&self) -> Vec<VdSynStmtChild> {
        match self {
            VdSynStmtData::Paragraph(sentences) => sentences
                .into_iter()
                .map(VdSynStmtChild::Sentence)
                .collect(),
            VdSynStmtData::Block { stmts, .. } => {
                stmts.into_iter().map(VdSynStmtChild::Stmt).collect()
            }
        }
    }
}

pub type VdSynStmtArena = Arena<VdSynStmtData>;
pub type VdSynStmtArenaRef<'a> = ArenaRef<'a, VdSynStmtData>;
pub type VdSynStmtIdx = ArenaIdx<VdSynStmtData>;
pub type VdSynStmtIdxRange = ArenaIdxRange<VdSynStmtData>;
pub type VdSynStmtMap<T> = ArenaMap<VdSynStmtData, T>;
pub type VdSynStmtOrderedMap<T> = ArenaOrderedMap<VdSynStmtData, T>;

impl ToVdSyn<VdSynStmtIdxRange> for (LxTokenIdxRange, LxRoseAstIdxRange) {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> VdSynStmtIdxRange {
        let (_, ast_idx_range) = self;
        builder.parse_stmts(ast_idx_range)
    }
}

impl<'db> VdSynExprBuilder<'db> {
    fn parse_stmts(&mut self, asts: LxRoseAstIdxRange) -> VdSynStmtIdxRange {
        let mut stmts: Vec<VdSynStmtData> = Vec::new();
        let mut asts = asts.into_iter().peekable();
        while let Some(stmt) = self.parse_stmt(&mut asts) {
            stmts.push(stmt);
        }
        self.alloc_stmts(stmts)
    }

    fn parse_stmt(
        &mut self,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
    ) -> Option<VdSynStmtData> {
        let ast_idx = asts.next()?;
        match self.ast_arena()[ast_idx] {
            LxRoseAstData::TextEdit { ref buffer } => todo!(),
            LxRoseAstData::Word(token_idx, word) => self.parse_paragraph(token_idx, word, asts),
            LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
            LxRoseAstData::Math {
                left_dollar_token_idx,
                math_asts,
                right_dollar_token_idx,
            } => todo!(),
        }
    }

    fn parse_paragraph(
        &mut self,
        token_idx: LxRoseTokenIdx,
        word: Coword,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
    ) -> Option<VdSynStmtData> {
        let sentences = vec![self.parse_sentence(token_idx, word, asts)];
        while let Some(ast_idx) = asts.next() {
            match self.ast_arena()[ast_idx] {
                LxRoseAstData::TextEdit { .. } => todo!(),
                LxRoseAstData::Word(lx_rose_token_idx, coword) => todo!(),
                LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                LxRoseAstData::Math {
                    left_dollar_token_idx,
                    math_asts,
                    right_dollar_token_idx,
                } => todo!(),
            }
        }
        Some(VdSynStmtData::Paragraph(self.alloc_sentences(sentences)))
    }
}
