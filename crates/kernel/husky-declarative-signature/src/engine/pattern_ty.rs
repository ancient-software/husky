mod pattern_expr_ty;
mod pattern_symbol_ty;

pub(crate) use self::pattern_expr_ty::*;
pub(crate) use self::pattern_symbol_ty::*;

use super::*;

impl<'a> DeclarativeTermEngine<'a> {
    /// only use this for explicit parameters
    pub(super) fn infer_pattern_tys_in_parenate_parameter(
        &mut self,
        pattern_expr: SynPatternExprIdx,
        ty: DeclarativeTerm,
    ) {
        self.infer_pattern_expr_tys(pattern_expr, ty);
        self.infer_pattern_symbol_tys(pattern_expr)
    }
}
