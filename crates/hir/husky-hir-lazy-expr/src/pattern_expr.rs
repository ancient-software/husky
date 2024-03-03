use crate::*;
use husky_entity_path::PatternPath;
use husky_syn_expr::{PatternSynExprIdx, SynPatternExprData, SynPatternExprRoot};
use husky_term_prelude::literal::Literal;
use idx_arena::ArenaRef;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirLazyPatternExpr {
    /// example: `1`
    /// todo: change this to primitive value data
    Literal(Literal),
    /// example: `a`
    Ident {
        // symbol_modifier: Option<EphemSymbolModifier>,
        ident: Ident,
    },
    /// example: `A::B`
    Unit(PatternPath),
    /// example: `(a, b)`
    Tuple {
        path: Option<PatternPath>,
        fields: HirLazyPatternExprIdxRange,
    },
    /// example: `C { .. }`
    Props {
        path: Option<PatternPath>,
        // todo: change to punctuated
        fields: HirLazyPatternExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf { options: HirLazyPatternExprIdxRange },
    /// example: `x @ 1..9`
    Binding {
        ident: Ident,
        /// example: `1..9`
        src: HirLazyPatternExprIdx,
    },
    /// example: `1..9`
    Range {
        start: HirLazyPatternExprIdx,
        end: HirLazyPatternExprIdx,
    },
}

pub type HirLazyPatternExprArena = Arena<HirLazyPatternExpr>;
pub type HirLazyPatternExprArenaRef<'a> = ArenaRef<'a, HirLazyPatternExpr>;
pub type HirLazyPatternExprIdx = ArenaIdx<HirLazyPatternExpr>;
pub type HirLazyPatternExprIdxRange = ArenaIdxRange<HirLazyPatternExpr>;
pub type HirLazyPatternExprMap<V> = ArenaMap<HirLazyPatternExpr, V>;
pub type HirLazyPatternExprOrderedMap<V> = ArenaOrderedMap<HirLazyPatternExpr, V>;

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_pattern_expr(
        &mut self,
        syn_pattern_root: impl Into<SynPatternExprRoot>,
    ) -> HirLazyPatternExprIdx {
        let pattern_expr =
            self.new_pattern_expr_aux(syn_pattern_root.into().syn_pattern_expr_idx());
        self.alloc_pattern_expr(pattern_expr)
    }

    fn new_pattern_expr_aux(
        &mut self,
        syn_pattern_expr_idx: PatternSynExprIdx,
    ) -> HirLazyPatternExpr {
        match self.syn_expr_region_data()[syn_pattern_expr_idx] {
            SynPatternExprData::Literal { .. } => todo!(),
            SynPatternExprData::Ident {
                symbol_modifier_tokens: _symbol_modifier_keyword_group,
                ident_token,
            } => HirLazyPatternExpr::Ident {
                // symbol_modifier: (),
                ident: ident_token.ident(),
            },
            SynPatternExprData::UnitTypeVariant { .. } => todo!(),
            SynPatternExprData::Tuple { .. } => todo!(),
            SynPatternExprData::TupleStruct { .. } => todo!(),
            SynPatternExprData::TupleTypeVariant { .. } => todo!(),
            SynPatternExprData::Props { name: _, fields: _ } => todo!(),
            SynPatternExprData::OneOf { options: _ } => todo!(),
            SynPatternExprData::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternExprData::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
