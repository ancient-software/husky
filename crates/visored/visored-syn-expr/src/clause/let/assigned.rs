use super::*;
use pattern::VdSynPattern;
use symbol::{
    builder::VdSynSymbolBuilder,
    local_defn::{VdSynSymbolLocalDefnBody, VdSynSymbolLocalDefnHead, VdSynSymbolLocalDefnSrc},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VdSynLetAssignedResolution {
    pattern: VdSynPattern,
    pattern_expr: VdSynExprIdx,
    assignment: VdSynExprIdx,
}

/// # getters
impl VdSynLetAssignedResolution {
    pub fn pattern(&self) -> &VdSynPattern {
        &self.pattern
    }

    pub fn assignment(&self) -> VdSynExprIdx {
        self.assignment
    }
}

impl<'db> VdSynSymbolBuilder<'db> {
    pub fn infer_let_assigned_resolution(
        &self,
        pattern_expr: VdSynExprIdx,
        assignment: VdSynExprIdx,
    ) -> VdSynLetAssignedResolution {
        VdSynLetAssignedResolution {
            pattern: self.build_pattern(pattern_expr),
            pattern_expr,
            assignment,
        }
    }
}

impl<'db> VdSynSymbolBuilder<'db> {
    /// Order matters!
    ///
    /// - First, build the assignment.
    /// - Then define the symbols in the pattern.
    pub(crate) fn build_symbols_in_let_assigned_resolution(
        &mut self,
        clause: VdSynClauseIdx,
        resolution: VdSynLetAssignedResolution,
    ) {
        self.build_expr(resolution.assignment);
        match resolution.pattern {
            VdSynPattern::Letter {
                token_idx_range,
                letter,
                pattern_expr,
            } => {
                self.define_symbol(
                    VdSynSymbolLocalDefnHead::Letter {
                        token_idx_range,
                        letter,
                    },
                    VdSynSymbolLocalDefnBody::Assigned,
                    VdSynSymbolLocalDefnSrc::LetAssigned(clause),
                );
            }
        }
        self.build_expr(resolution.pattern_expr);
    }
}
