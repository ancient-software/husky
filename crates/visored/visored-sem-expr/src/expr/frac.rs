use super::*;
use visored_global_dispatch::dispatch::frac::VdFracGlobalDispatch;
use visored_signature::signature::binary_opr::base::VdBaseBinaryOprSignature;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemFracDispatch {
    Div { signature: VdBaseBinaryOprSignature },
}

impl<'a> VdSemExprBuilder<'a> {
    pub(super) fn build_frac(
        &mut self,
        command_token_idx: LxMathTokenIdx,
        syn_numerator: VdSynExprIdx,
        syn_denominator: VdSynExprIdx,
        denominator_arg: LxMathCompleteCommandArgument,
    ) -> (VdSemExprData, VdType) {
        let numerator = self.build_expr_entry(syn_numerator);
        let denominator = self.build_expr_entry(syn_denominator);
        if let Some(dispatch) = self
            .default_global_dispatch_table()
            .base_frac_default_dispatch(numerator.ty, denominator.ty)
        {
            let numerator =
                self.alloc_expr(syn_numerator, numerator, Some(dispatch.numerator_ty()));
            let denominator = self.alloc_expr(
                syn_denominator,
                denominator,
                Some(dispatch.denominator_ty()),
            );
            match dispatch {
                VdFracGlobalDispatch::Div { signature } => (
                    VdSemExprData::Frac {
                        command_token_idx,
                        numerator,
                        denominator,
                        denominator_arg,
                        dispatch: VdSemFracDispatch::Div { signature },
                    },
                    signature.expr_ty(),
                ),
            }
        } else {
            todo!()
        }
    }
}
