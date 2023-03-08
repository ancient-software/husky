use super::*;
use husky_print_utils::p;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};
use vec_like::VecSet;

pub(crate) type LocalTermExpectationIdx = ArenaIdx<LocalTermExpectationRule>;
pub(crate) type OptionLocalTermExpectationIdx = OptionArenaIdx<LocalTermExpectationRule>;

impl std::ops::Index<UnresolvedTermIdx> for LocalTermRegion {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.unresolved_terms[index]
    }
}

impl std::ops::Index<LocalTermExpectationIdx> for LocalTermRegion {
    type Output = LocalTermExpectationRule;

    fn index(&self, index: LocalTermExpectationIdx) -> &Self::Output {
        &self.expectations[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalTermResolveLevel {
    Weak,
    Strong,
}

impl<'a> ExprTypeEngine<'a> {
    fn next_expectation_effect(
        &self,
        level: LocalTermResolveLevel,
        local_term_region: &mut LocalTermRegion,
    ) -> Option<(LocalTermExpectationIdx, LocalTermExpectationEffect)> {
        for (idx, rule) in local_term_region.expectations.unresolved_rule_iter() {
            if let Some(action) =
                self.resolve_expectation(rule, level, &mut local_term_region.unresolved_terms)
            {
                return Some((idx, action));
            }
        }
        None
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn resolve_term(
        &mut self,
        unresolved_term_idx: UnresolvedTermIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Option<Term> {
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Weak, local_term_region);
        local_term_region
            .unresolved_terms
            .resolve_term(unresolved_term_idx)
    }

    pub(crate) fn resolve_as_much_as_possible(
        &mut self,
        level: LocalTermResolveLevel,
        local_term_region: &mut LocalTermRegion,
    ) {
        while let Some((rule_idx, effect)) = self.next_expectation_effect(level, local_term_region)
        {
            if let Some(actions) = local_term_region.expectations.take_effect(rule_idx, effect) {
                for action in actions {
                    match action {
                        TermResolveAction::SubstituteImplicitSymbol {
                            implicit_symbol,
                            substitution,
                        } => local_term_region
                            .substitute_implicit_symbol(implicit_symbol, substitution),
                    }
                }
            }
        }
    }

    pub(crate) fn finalize_unresolved_term_table(
        &mut self,
        local_term_region: &mut LocalTermRegion,
    ) {
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Strong, local_term_region);
        // ad hoc
        // todo!()
    }
}

pub(super) enum TermResolveAction {
    SubstituteImplicitSymbol {
        implicit_symbol: UnresolvedTermIdx,
        substitution: LocalTerm,
    },
}
