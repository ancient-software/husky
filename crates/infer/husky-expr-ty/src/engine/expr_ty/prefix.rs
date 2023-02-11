use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_prefix_ty(
        &mut self,
        opd: ExprIdx,
        opr: PrefixOpr,
    ) -> ExprTypeResult<LocalTerm> {
        match opr {
            PrefixOpr::Minus => {
                let opd_ty = self.infer_new_expr_ty(opd, LocalTermExpectation::None);
                match opd_ty {
                    Some(opd_ty) => match opd_ty {
                        LocalTerm::Resolved(_) => todo!(),
                        LocalTerm::Unresolved(unresolved_term) => {
                            match self.local_term_table[unresolved_term].unresolved_term() {
                                UnresolvedTerm::ImplicitSymbol(implicit_symbol) => {
                                    match implicit_symbol.variant() {
                                        ImplicitSymbolVariant::ExprEvalLifetime => todo!(),
                                        ImplicitSymbolVariant::UnspecifiedIntegerType
                                        | ImplicitSymbolVariant::UnspecifiedFloatType => Ok(opd_ty),
                                        ImplicitSymbolVariant::ImplicitType => todo!(),
                                        ImplicitSymbolVariant::ImplicitLifetime => todo!(),
                                    }
                                }
                                UnresolvedTerm::TypeApplication { ty, arguments } => todo!(),
                            }
                        }
                    },
                    None => Err(DerivedExprTypeError::PrefixOperandTypeNotInferred.into()),
                }
            }
            PrefixOpr::Not => {
                let _opd_ty = self.infer_new_expr_ty(opd, LocalTermExpectation::CastibleAsBool);
                // here we differs from Rust, but agrees with C
                Ok(self.reduced_term_menu.bool().into())
            }
            PrefixOpr::BitNot => todo!(),
            PrefixOpr::Ref => {
                let opd_ty = self.infer_new_expr_ty(opd, LocalTermExpectation::None);
                // Should consider more cases, could also be taking references
                opd_ty.ok_or(DerivedExprTypeError::PrefixOperandTypeNotInferred.into())
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => {
                let opd_ty = self.infer_new_expr_ty(opd, LocalTermExpectation::TypeType);
                opd_ty.ok_or(DerivedExprTypeError::PrefixOperandTypeNotInferred.into())
            }
        }
    }
}
