use crate::{
    builder::{ToVdSyn, VdSynExprBuilder},
    sentence::{VdSynSentenceIdx, VdSynSentenceIdxRange},
    symbol::builder::VdSynSymbolBuilder,
};
use husky_coword::Coword;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::{
    root::LxRootAstIdxRange,
    rose::{LxRoseAstData, LxRoseAstIdx, LxRoseAstIdxRange},
};
use latex_environment::signature::LxEnvironmentSignature;
use latex_token::idx::{LxRoseTokenIdx, LxTokenIdxRange};
use std::iter::Peekable;

pub enum VdSynStmtData {
    Paragraph(VdSynSentenceIdxRange),
    Environment {
        environment_signature: LxEnvironmentSignature,
        stmts: VdSynStmtIdxRange,
    },
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

impl ToVdSyn<VdSynStmtIdxRange> for (LxTokenIdxRange, LxRootAstIdxRange) {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> VdSynStmtIdxRange {
        todo!()
        // let (_, ast_idx_range) = self;
        // builder.parse_stmts(ast_idx_range)
    }
}

impl<'db> VdSynExprBuilder<'db> {
    fn parse_stmts(&mut self, asts: LxRoseAstIdxRange) -> VdSynStmtIdxRange {
        let mut asts = asts.into_iter().peekable();
        self.parse_stmt_aux(&mut asts)
    }

    pub(crate) fn parse_stmt_aux(
        &mut self,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
    ) -> VdSynStmtIdxRange {
        let mut stmts: Vec<VdSynStmtData> = Vec::new();
        while let Some(stmt) = self.parse_stmt(asts) {
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
            LxRoseAstData::Math { .. } => todo!(),
            LxRoseAstData::Delimited {
                left_delimiter_token_idx,
                left_delimiter,
                asts,
                right_delimiter_token_idx,
                right_delimiter,
            } => todo!(),
            LxRoseAstData::CompleteCommand {
                command_token_idx,
                command_path,
                options,
                ref arguments,
            } => todo!(),
            LxRoseAstData::Environment { .. } => todo!(),
            LxRoseAstData::NewParagraph(_) => todo!(),
            LxRoseAstData::NewDivision { .. } => todo!(),
        }
    }

    fn parse_paragraph(
        &mut self,
        token_idx: LxRoseTokenIdx,
        word: Coword,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
    ) -> Option<VdSynStmtData> {
        let mut sentences = vec![self.parse_sentence(token_idx, word, asts)];
        while let Some(ast_idx) = asts.next() {
            match self.ast_arena()[ast_idx] {
                LxRoseAstData::TextEdit { .. } => todo!(),
                LxRoseAstData::Word(lx_rose_token_idx, coword) => {
                    sentences.push(self.parse_sentence(lx_rose_token_idx, coword, asts));
                }
                LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                LxRoseAstData::Math {
                    left_delimiter_token_idx: left_dollar_token_idx,
                    math_asts,
                    right_delimiter_token_idx: right_dollar_token_idx,
                } => todo!(),
                LxRoseAstData::Delimited {
                    left_delimiter_token_idx,
                    left_delimiter,
                    asts,
                    right_delimiter_token_idx,
                    right_delimiter,
                } => todo!(),
                LxRoseAstData::CompleteCommand {
                    command_token_idx,
                    command_path,
                    options,
                    ref arguments,
                } => todo!(),
                LxRoseAstData::Environment { .. } => todo!(),
                LxRoseAstData::NewParagraph(_) => todo!(),
                LxRoseAstData::NewDivision { .. } => todo!(),
            }
        }
        Some(VdSynStmtData::Paragraph(self.alloc_sentences(sentences)))
    }
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
            VdSynStmtData::Environment { stmts, .. } => {
                stmts.into_iter().map(VdSynStmtChild::Stmt).collect()
            }
        }
    }
}

impl<'db> VdSynSymbolBuilder<'db> {
    pub(crate) fn build_stmt_aux(&mut self, stmt: VdSynStmtIdx) {
        match self.stmt_arena()[stmt] {
            VdSynStmtData::Paragraph(sentences) => self.build_sentences(sentences),
            VdSynStmtData::Environment {
                environment_signature,
                stmts,
            } => self.build_stmts(stmts),
        }
    }
}
