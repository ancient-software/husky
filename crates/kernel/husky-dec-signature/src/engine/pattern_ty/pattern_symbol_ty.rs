use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PatternSymbolDeclarativeTypeInfo {
    modifier: VariableModifier,
    base_ty: DecTerm,
}

impl PatternSymbolDeclarativeTypeInfo {
    fn new(modifier: VariableModifier, base_ty: DecTerm) -> Self {
        Self { modifier, base_ty }
    }

    pub fn modifier(&self) -> VariableModifier {
        self.modifier
    }

    pub fn base_ty(&self) -> DecTerm {
        self.base_ty
    }
}

impl<'a> DecTermEngine<'a> {
    pub(super) fn infer_pattern_symbol_tys(
        &mut self,
        syn_pattern_expr_root: impl Into<SynPatternRoot>,
    ) {
        let syn_pattern_expr_root = syn_pattern_expr_root.into();
        for (_, pattern_symbol) in self
            .syn_expr_region_data
            .pattern_expr_region()
            .pattern_expr_symbols(syn_pattern_expr_root.syn_pattern_expr_idx())
        {
            self.infer_new_pattern_symbol_ty(*pattern_symbol)
        }
    }

    fn infer_new_pattern_symbol_ty(&mut self, pattern_variable_idx: PatternVariableIdx) {
        let modifier = self
            .syn_expr_region_data
            .pattern_symbol_modifier(pattern_variable_idx);
        let base_ty = self.calc_new_pattern_symbol_base_ty(pattern_variable_idx);
        self.pattern_symbol_ty_infos.insert_new(
            pattern_variable_idx,
            PatternSymbolDeclarativeTypeInfo::new(modifier, base_ty),
        )
    }

    fn calc_new_pattern_symbol_base_ty(&mut self, pattern_symbol: PatternVariableIdx) -> DecTerm {
        match self.syn_expr_region_data[pattern_symbol] {
            PatternVariable::Atom(pattern_expr) => self
                .get_pattern_expr_ty(pattern_expr)
                .expect("pattern expression type should be inferred at this point"),
        }
    }
}
