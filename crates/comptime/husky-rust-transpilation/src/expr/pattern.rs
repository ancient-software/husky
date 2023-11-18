use husky_term_prelude::SymbolModifier;

use super::*;

impl TranspileToRust<HirEagerExprRegion> for HirEagerPatternExprIdx {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self.data(builder.hir_eager_pattern_expr_arena()) {
            HirEagerPatternExpr::Literal(_) => todo!(),
            HirEagerPatternExpr::Ident {
                symbol_modifier,
                ident,
            } => {
                if let Some(symbol_modifier) = symbol_modifier {
                    match symbol_modifier {
                        SymbolModifier::None => (),
                        SymbolModifier::Mut => builder.keyword(RustKeyword::Mut),
                        SymbolModifier::RefMut => todo!(),
                        SymbolModifier::Const => todo!(),
                        SymbolModifier::Ambersand(_) => todo!(),
                        SymbolModifier::AmbersandMut(_) => todo!(),
                        SymbolModifier::Le => todo!(),
                        SymbolModifier::Tilde => todo!(),
                    }
                }
                ident.transpile_to_rust(builder)
            }
            HirEagerPatternExpr::Unit(_) => todo!(),
            HirEagerPatternExpr::Tuple { path: _, fields: _ } => todo!(),
            HirEagerPatternExpr::Props { path: _, fields: _ } => todo!(),
            HirEagerPatternExpr::OneOf { options: _ } => todo!(),
            HirEagerPatternExpr::Binding { ident: _, src: _ } => todo!(),
            HirEagerPatternExpr::Range { start: _, end: _ } => todo!(),
        }
    }
}
