use crate::{
    clause::{VdSynClauseArena, VdSynClauseArenaRef},
    division::{VdSynDivisionArena, VdSynDivisionArenaRef},
    expr::{VdSynExprArena, VdSynExprArenaRef, VdSynExprData, VdSynExprIdx},
    phrase::{VdSynPhraseArena, VdSynPhraseArenaRef, VdSynPhraseData, VdSynPhraseIdx},
    sentence::{VdSynSentenceArena, VdSynSentenceArenaRef, VdSynSentenceData, VdSynSentenceIdx},
    stmt::{VdSynStmtArena, VdSynStmtArenaRef},
    symbol::{defn::VdSynSymbolDefns, resolution::VdSynSymbolResolutionTable},
};

pub struct VdSynExprRegionData {
    expr_arena: VdSynExprArena,
    phrase_arena: VdSynPhraseArena,
    clause_arena: VdSynClauseArena,
    sentence_arena: VdSynSentenceArena,
    stmt_arena: VdSynStmtArena,
    division_arena: VdSynDivisionArena,
    symbol_defns: VdSynSymbolDefns,
    symbol_resolutions: VdSynSymbolResolutionTable,
}

impl VdSynExprRegionData {
    pub(crate) fn new(
        expr_arena: VdSynExprArena,
        phrase_arena: VdSynPhraseArena,
        clause_arena: VdSynClauseArena,
        sentence_arena: VdSynSentenceArena,
        stmt_arena: VdSynStmtArena,
        division_arena: VdSynDivisionArena,
        symbol_defns: VdSynSymbolDefns,
        symbol_resolutions: VdSynSymbolResolutionTable,
    ) -> Self {
        Self {
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            stmt_arena,
            division_arena,
            symbol_defns,
            symbol_resolutions,
        }
    }

    pub fn expr_arena(&self) -> VdSynExprArenaRef {
        self.expr_arena.as_arena_ref()
    }

    pub fn phrase_arena(&self) -> VdSynPhraseArenaRef {
        self.phrase_arena.as_arena_ref()
    }

    pub fn clause_arena(&self) -> VdSynClauseArenaRef {
        self.clause_arena.as_arena_ref()
    }

    pub fn sentence_arena(&self) -> VdSynSentenceArenaRef {
        self.sentence_arena.as_arena_ref()
    }

    pub fn stmt_arena(&self) -> VdSynStmtArenaRef {
        self.stmt_arena.as_arena_ref()
    }

    pub fn division_arena(&self) -> VdSynDivisionArenaRef {
        self.division_arena.as_arena_ref()
    }

    pub fn symbol_defns(&self) -> &VdSynSymbolDefns {
        &self.symbol_defns
    }

    pub fn symbol_resolutions(&self) -> &VdSynSymbolResolutionTable {
        &self.symbol_resolutions
    }
}
