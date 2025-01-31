use super::*;
use visored_global_dispatch::dispatch::sqrt::VdSqrtGlobalDispatch;
use visored_signature::signature::sqrt::VdBaseSqrtSignature;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemSqrtDispatch {
    Base { signature: VdBaseSqrtSignature },
}

impl<'a> VdSemExprBuilder<'a> {
    pub fn build_sqrt(
        &mut self,
        command_token_idx: LxMathTokenIdx,
        syn_radicand: VdSynExprIdx,
        radicand_arg: LxMathCompleteCommandArgument,
    ) -> (VdSemExprData, VdType) {
        let radicand = self.build_expr_entry(syn_radicand);
        if let Some(dispatch) = self
            .default_global_dispatch_table()
            .base_sqrt_default_dispatch(radicand.ty)
        {
            match dispatch {
                VdSqrtGlobalDispatch::Base { signature } => {
                    let expr_ty = signature.expr_ty();
                    let radicand =
                        self.alloc_expr(syn_radicand, radicand, Some(signature.radicand_ty()));
                    let expr_data = VdSemExprData::Sqrt {
                        command_token_idx,
                        radicand,
                        radicand_arg,
                        dispatch: VdSemSqrtDispatch::Base { signature },
                    };
                    (expr_data, expr_ty)
                }
            }
        } else {
            todo!()
        }
    }
}
