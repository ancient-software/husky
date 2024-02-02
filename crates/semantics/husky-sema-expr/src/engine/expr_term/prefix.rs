use husky_sema_opr::prefix::SemaPrefixOpr;

use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_prefix_expr_term(
        &mut self,
        expr_idx: SemaExprIdx,
        opr: SemaPrefixOpr,
        opd: SemaExprIdx,
    ) -> SemaExprTermResult<FlyTerm> {
        let Some(opd_term) = self.infer_expr_term(opd) else {
            return Err(DerivedSemaExprTermError::PrefixOprTermNotInferred.into());
        };
        match opr {
            SemaPrefixOpr::Minus => todo!(),
            SemaPrefixOpr::Not => todo!(),
            SemaPrefixOpr::BitNot => todo!(),
            SemaPrefixOpr::LeashType => Ok(FlyTerm::new_leashed(self, opd_term)?),
            SemaPrefixOpr::RefType => {
                // let opd_ty = self.infer
                // match
                todo!()
            }
            SemaPrefixOpr::Option => {
                Ok(
                    FlyTerm::new_application(self, self.term_menu.option_ty_ontology(), opd_term)
                        .map_err(|e| DerivedSemaExprTermError::OptionApplicationTerm(e))?
                        .into(),
                )
            }
        }
    }
}
