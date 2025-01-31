use super::*;
use crate::{
    builder::VdSynExprBuilder,
    clause::VdSynClauseArena,
    expr::VdSynExprArena,
    phrase::VdSynPhraseArena,
    range::{
        VdSynClauseTokenIdxRangeMap, VdSynExprTokenIdxRangeMap, VdSynPhraseTokenIdxRangeMap,
        VdSynSentenceTokenIdxRangeMap,
    },
    sentence::VdSynSentenceArena,
};
use block::{VdSynBlockArena, VdSynBlockIdx, VdSynBlockIdxRange, VdSynBlockMap};
use builder::FromToVdSyn;
use clause::{r#let::VdSynLetClauseResolution, VdSynClauseIdx, VdSynClauseMap};
use division::{VdSynDivisionArena, VdSynDivisionIdxRange, VdSynDivisionMap};
use entity_tree::{builder::VdSynExprEntityTreeBuilder, VdSynExprEntityTreeNode};
use eterned::db::EternerDb;
use expr::VdSynExprIdx;
use helpers::show::display_tree::VdSynExprDisplayTreeBuilder;
use husky_tree_utils::display::DisplayTree;
use latex_ast::{
    ast::{
        math::LxMathAstIdxRange, parse_latex_input_into_asts, root::LxRootAstIdxRange,
        rose::LxRoseAstIdxRange, LxAstArena, LxAstIdxRange,
    },
    helpers::tracker::{IsLxAstInput, LxAstTracker},
    range::{calc_ast_token_idx_range_map, LxAstTokenIdxRangeMap},
};
use latex_command::signature::table::LxCommandSignatureTable;
use latex_environment::signature::table::LxEnvironmentSignatureTable;
use latex_prelude::{
    helper::tracker::{LxDocumentBodyInput, LxDocumentInput, LxFormulaInput, LxPageInput},
    mode::LxMode,
};
use latex_token::{idx::LxTokenIdxRange, lane::LxTokenLane, storage::LxTokenStorage};
use phrase::VdSynPhraseIdx;
use range::{calc_expr_range_map, VdSynBlockTokenIdxRangeMap, VdSynDivisionTokenIdxRangeMap};
use sealed::*;
use sentence::VdSynSentenceIdx;
use symbol::{
    builder::VdSynSymbolBuilder, local_defn::VdSynSymbolLocalDefnStorage,
    resolution::VdSynSymbolResolutionsTable,
};
use visored_annotation::{
    annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation},
    annotations::VdAnnotations,
};
use visored_entity_path::module::VdModulePath;
use visored_global_resolution::default_table::VdDefaultGlobalResolutionTable;
use visored_models::VdModels;

pub struct VdSynExprTracker<'a, Input: IsVdSynExprInput<'a>> {
    pub input: Input,
    pub annotations: VdAnnotations,
    pub default_resolution_table: VdDefaultGlobalResolutionTable,
    pub token_storage: LxTokenStorage,
    pub ast_arena: LxAstArena,
    pub ast_token_idx_range_map: LxAstTokenIdxRangeMap,
    pub expr_arena: VdSynExprArena,
    pub phrase_arena: VdSynPhraseArena,
    pub clause_arena: VdSynClauseArena,
    pub sentence_arena: VdSynSentenceArena,
    pub stmt_arena: VdSynBlockArena,
    pub division_arena: VdSynDivisionArena,
    pub expr_range_map: VdSynExprTokenIdxRangeMap,
    pub phrase_range_map: VdSynPhraseTokenIdxRangeMap,
    pub clause_range_map: VdSynClauseTokenIdxRangeMap,
    pub sentence_range_map: VdSynSentenceTokenIdxRangeMap,
    pub stmt_range_map: VdSynBlockTokenIdxRangeMap,
    pub division_range_map: VdSynDivisionTokenIdxRangeMap,
    pub let_clause_resolutions: VdSynClauseMap<VdSynLetClauseResolution>,
    pub symbol_local_defn_storage: VdSynSymbolLocalDefnStorage,
    pub symbol_resolution_table: VdSynSymbolResolutionsTable,
    pub root_entity_tree_node: VdSynExprEntityTreeNode,
    pub stmt_entity_tree_node_map: VdSynBlockMap<VdSynExprEntityTreeNode>,
    pub division_entity_tree_node_map: VdSynDivisionMap<VdSynExprEntityTreeNode>,
    pub output: Input::VdSynExprOutput,
}

// #[sealed]
pub trait IsVdSynExprInput<'a>: IsLxAstInput<'a> {
    type VdSynExprOutput: IsVdSynOutput + FromToVdSyn<(LxTokenIdxRange, Self::LxAstOutput)>;
}

pub trait IsVdSynOutput: std::fmt::Debug + Copy {
    fn build_entity_tree_root_node(
        self,
        builder: &mut VdSynExprEntityTreeBuilder,
    ) -> VdSynExprEntityTreeNode;
    fn build_all_symbols(self, builder: &mut VdSynSymbolBuilder);
    fn show(&self, builder: &VdSynExprDisplayTreeBuilder) -> String;
}

// #[sealed]
impl<'a, Input: IsVdSynExprInput<'a>> VdSynExprTracker<'a, Input> {
    pub fn new(
        input: Input,
        token_annotations: &[((&str, &str), VdTokenAnnotation)],
        space_annotations: &[((&str, &str), VdSpaceAnnotation)],
        models: &VdModels,
        vibe: VdSynExprVibe,
        db: &EternerDb,
    ) -> Self {
        let LxAstTracker {
            command_signature_table,
            input,
            token_storage,
            ast_arena,
            ast_token_idx_range_map,
            output: lx_ast_output,
        } = LxAstTracker::new(input, db);
        // ad hoc
        let whole_token_range = token_storage.whole_token_idx_range(LxTokenLane::Main);
        let annotations = VdAnnotations::from_sparse(
            input.content(),
            token_annotations.iter().copied(),
            space_annotations.iter().copied(),
            &token_storage,
        );
        let default_resolution_table = VdDefaultGlobalResolutionTable::new_standard(db);
        let mut builder = VdSynExprBuilder::new(
            db,
            input.content(),
            input.file_path(),
            &token_storage,
            ast_arena.as_arena_ref(),
            &ast_token_idx_range_map,
            &annotations,
            &default_resolution_table,
            models,
        );
        let output =
            FromToVdSyn::from_to_vd_syn((whole_token_range, lx_ast_output), &mut builder, vibe);
        //  = (whole_token_range, asts).to_vd_syn(&mut builder);
        let (
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            stmt_arena,
            division_arena,
            expr_range_map,
            phrase_range_map,
            clause_range_map,
            sentence_range_map,
            stmt_range_map,
            division_range_map,
            root_entity_tree_node,
            stmt_entity_tree_node_map,
            division_entity_tree_node_map,
            let_clause_resolutions,
            symbol_defns,
            symbol_resolutions,
        ) = builder.finish_with(output);
        Self {
            input,
            annotations,
            default_resolution_table,
            token_storage,
            ast_arena,
            ast_token_idx_range_map,
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            stmt_arena,
            division_arena,
            expr_range_map,
            phrase_range_map,
            clause_range_map,
            sentence_range_map,
            stmt_range_map,
            division_range_map,
            let_clause_resolutions,
            symbol_local_defn_storage: symbol_defns,
            symbol_resolution_table: symbol_resolutions,
            root_entity_tree_node,
            stmt_entity_tree_node_map,
            division_entity_tree_node_map,
            output,
        }
    }

    fn display_tree_builder<'b>(&'b self, db: &'b EternerDb) -> VdSynExprDisplayTreeBuilder<'b> {
        VdSynExprDisplayTreeBuilder::new(
            db,
            self.input.content(),
            &self.token_storage,
            self.ast_arena.as_arena_ref(),
            &self.ast_token_idx_range_map,
            self.expr_arena.as_arena_ref(),
            self.phrase_arena.as_arena_ref(),
            self.clause_arena.as_arena_ref(),
            self.sentence_arena.as_arena_ref(),
            self.stmt_arena.as_arena_ref(),
            self.division_arena.as_arena_ref(),
            &self.expr_range_map,
            &self.phrase_range_map,
            &self.clause_range_map,
            &self.sentence_range_map,
            &self.stmt_range_map,
            &self.division_range_map,
        )
    }

    pub(crate) fn show_display_tree(&self, db: &EternerDb) -> String {
        let builder = self.display_tree_builder(db);
        self.output.show(&builder)
    }
}

impl<'a> IsVdSynExprInput<'a> for LxDocumentInput<'a> {
    type VdSynExprOutput = VdSynDivisionIdxRange;
}

impl<'a> IsVdSynExprInput<'a> for LxDocumentBodyInput<'a> {
    type VdSynExprOutput = VdSynDivisionIdxRange;
}

impl<'a> IsVdSynExprInput<'a> for LxPageInput<'a> {
    type VdSynExprOutput = VdSynBlockIdxRange;
}

impl<'a> IsVdSynExprInput<'a> for LxFormulaInput<'a> {
    type VdSynExprOutput = VdSynFormula;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdSynFormula {
    pub expr: VdSynExprIdx,
}

impl ToVdSyn<VdSynFormula> for (LxTokenIdxRange, LxMathAstIdxRange) {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder, vibe: VdSynExprVibe) -> VdSynFormula {
        VdSynFormula {
            expr: self.to_vd_syn(builder, vibe),
        }
    }
}

impl IsVdSynOutput for VdSynDivisionIdxRange {
    fn build_entity_tree_root_node(
        self,
        builder: &mut VdSynExprEntityTreeBuilder,
    ) -> VdSynExprEntityTreeNode {
        builder.build_root_divisions(self)
    }

    fn build_all_symbols(self, builder: &mut VdSynSymbolBuilder) {
        builder.build_divisions(self)
    }

    fn show(&self, builder: &VdSynExprDisplayTreeBuilder) -> String {
        DisplayTree::show_trees(&builder.render_divisions(*self), &Default::default())
    }
}

impl IsVdSynOutput for VdSynBlockIdxRange {
    fn build_entity_tree_root_node(
        self,
        builder: &mut VdSynExprEntityTreeBuilder,
    ) -> VdSynExprEntityTreeNode {
        builder.build_root_stmts(self)
    }

    fn build_all_symbols(self, builder: &mut VdSynSymbolBuilder) {
        builder.build_stmts(self);
    }

    fn show(&self, builder: &VdSynExprDisplayTreeBuilder) -> String {
        builder.render_all_stmts(*self).show(&Default::default())
    }
}

impl IsVdSynOutput for VdSynFormula {
    fn build_entity_tree_root_node(
        self,
        builder: &mut VdSynExprEntityTreeBuilder,
    ) -> VdSynExprEntityTreeNode {
        VdSynExprEntityTreeNode::new(
            VdModulePath::new_root(builder.file_path(), builder.db()),
            vec![],
        )
    }

    fn build_all_symbols(self, builder: &mut VdSynSymbolBuilder) {
        builder.build_expr(self.expr);
    }

    fn show(&self, builder: &VdSynExprDisplayTreeBuilder) -> String {
        builder.render_expr(self.expr).show(&Default::default())
    }
}
