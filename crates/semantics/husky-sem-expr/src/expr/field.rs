use super::*;
use husky_regional_token::IdentRegionalToken;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_field_expr_ty(
        &mut self,
        owner: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
    ) -> (SemExprDataResult<SemExprData>, SemExprTypeResult<FlyTerm>) {
        let (owner_sem_expr_idx, owner_ty) = self.build_sem_expr_with_ty(owner, ExpectAnyOriginal);
        let Some(owner_ty) = owner_ty else {
            return (
                Err(
                    DerivedSemExprDataError::FieldOwnerTypeNotInferred { owner_sem_expr_idx }
                        .into(),
                ),
                Err(DerivedSemExprTypeError::FieldOwnerTypeNotInferred.into()),
            );
        };
        let field_dispatch = owner_ty
            .field_dispatch(self, ident_token.ident(), /* ad hoc: traits */ &[])
            .into_result_or(OriginalSemExprDataError::NoSuchField {
                owner_ty,
                ident_token,
            });
        let field_dispatch = match field_dispatch {
            Ok(field_dispatch) => field_dispatch,
            Err(e) => return (Err(e), todo!()),
        };
        let expr_ty_result = field_dispatch.expr_ty_result();
        (
            Ok(SemExprData::Field {
                self_argument: owner_sem_expr_idx,
                self_ty: owner_ty,
                dot_regional_token_idx,
                ident_token,
                dispatch: field_dispatch,
            }),
            expr_ty_result.map_err(Into::into),
        )
    }
}
