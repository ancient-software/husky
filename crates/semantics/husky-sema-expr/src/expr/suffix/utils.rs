use super::*;
use husky_sema_opr::suffix::SemaSuffixOpr;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_ambiguous_suffix_expr_ty<F1, F2, F3>(
        &mut self,
        opd: SynExprIdx,
        opr_regional_token_idx: RegionalTokenIdx,
        final_destination: FinalDestination,
        naive_suffix_f_given_opd_ty: F1,
        naive_suffix_f: F2,
        application_composition_f_given_opd_ty: F3,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FlyTerm>,
    )
    where
        F1: FnOnce(
            &mut Self,
            FlyTerm,
        ) -> (
            SemaExprDataResult<SemaSuffixOpr>,
            SemaExprTypeResult<FlyTerm>,
        ),
        F2: FnOnce(
            &mut Self,
            SynExprIdx,
            RegionalTokenIdx,
        ) -> (
            SemaExprDataResult<SemaExprData>,
            SemaExprTypeResult<FlyTerm>,
        ),
        F3: FnOnce(
            &mut Self,
            FlyTerm,
        ) -> (
            SemaExprDataResult<SemaSuffixOpr>,
            SemaExprTypeResult<FlyTerm>,
        ),
    {
        match final_destination {
            FinalDestination::Sort => {
                let (opd_sema_expr_idx, opd_ty) = self
                    .build_sema_expr_with_ty(opd, ExpectFinalDestination::new(final_destination));
                match opd_ty {
                    Some(opd_ty) => match opd_ty.data(self) {
                        FlyTermData::Literal(_) => todo!(),
                        FlyTermData::TypeOntology { .. } => {
                            let (sema_opr_result, ty_result) =
                                naive_suffix_f_given_opd_ty(self, opd_ty);
                            (
                                sema_opr_result.map(|opr| SemaExprData::Suffix {
                                    opd_sema_expr_idx,
                                    opr,
                                    opr_regional_token_idx,
                                }),
                                ty_result,
                            )
                        }
                        FlyTermData::Curry { .. } => todo!(),
                        FlyTermData::Hole(_, _) => todo!(),
                        FlyTermData::Sort(_) => todo!(),
                        FlyTermData::Ritchie {
                            ritchie_kind,
                            parameter_contracted_tys,
                            return_ty,
                        } => todo!(),
                        FlyTermData::Symbol { .. } => todo!(),
                        FlyTermData::Hvar { .. } => todo!(),
                        FlyTermData::TypeVariant { path } => todo!(),
                    },
                    None => (
                        todo!(),
                        Err(DerivedSemaExprTypeError::UnableToInferSuffixOperandType.into()),
                    ),
                }
            }
            _ => naive_suffix_f(self, opd, opr_regional_token_idx),
        }
    }
}
