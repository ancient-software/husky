use husky_hir_ty::ritchie::HirEagerContract;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub enum HirEagerParenateParameter {
    Simple {
        pattern_expr_idx: HirEagerPatternExprIdx,
        contract: HirEagerContract,
        ty: HirType,
    },
    Keyed,
    Variadic,
}

impl HirEagerParenateParameter {
    pub(crate) fn from_syn(
        syndicate: &ParenateParameterSyndicate,
        builder: &HirDeclBuilder,
    ) -> Option<Self> {
        Some(match syndicate {
            &ParenateParameterSyndicate::Simple {
                syn_pattern_root,
                ty,
                ..
            } => HirEagerParenateParameter::Simple {
                pattern_expr_idx: builder.hir_eager_pattern_expr_idx(syn_pattern_root),
                contract: HirEagerContract::from_term(
                    builder
                        .syn_expr_region_data()
                        .pattern_contract(syn_pattern_root.syn_pattern_expr_idx()),
                ),
                ty: builder.hir_ty(ty).unwrap(),
            },
            ParenateParameterSyndicate::Variadic {
                dot_dot_dot_token: _,
                variadic_variant: _,
                symbol_modifier_keyword_group: _,
                ident_token: _,
                variable: _,
                colon: _,
                ty: _,
            } => HirEagerParenateParameter::Variadic,
            ParenateParameterSyndicate::Keyed {
                syn_pattern_root: _,
                symbol_modifier_keyword_group: _,
                ident_token: _,
                variable: _,
                colon: _,
                ty: _,
                eq_token: _,
                default: _,
            } => HirEagerParenateParameter::Keyed,
        })
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerParenateParameters(SmallVec<[HirEagerParenateParameter; 4]>);

impl std::ops::Deref for HirEagerParenateParameters {
    type Target = SmallVec<[HirEagerParenateParameter; 4]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HirEagerParenateParameters {
    pub(crate) fn from_syn(
        syndicates: &[ParenateParameterSyndicate],
        builder: &HirDeclBuilder,
    ) -> Self {
        Self(
            syndicates
                .iter()
                .filter_map(|syndicate| HirEagerParenateParameter::from_syn(syndicate, builder))
                .collect(),
        )
    }
}
