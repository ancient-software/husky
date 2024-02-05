mod dex_index;
mod num_index;
mod regular_index;

use super::*;
use husky_eth_term::term::application::TermFunctionReduced;

pub(super) fn ethereal_owner_ty_index_dispatch(
    engine: &mut impl FlyTermEngineMut,
    expr_idx: SynExprIdx,
    owner_ty: EthTerm,
    index_ty: FlyTerm,
    indirections: FlyIndirections,
) -> FlyTermMaybeResult<FlyIndexDynamicDispatch> {
    ethereal_owner_ty_index_dispatch_aux(engine, expr_idx, owner_ty, index_ty, indirections)
}

pub(super) fn ethereal_owner_ty_index_dispatch_aux(
    engine: &mut impl FlyTermEngineMut,
    expr_idx: SynExprIdx,
    owner_ty: EthTerm,
    index_ty: FlyTerm,
    mut indirections: FlyIndirections,
) -> FlyTermMaybeResult<FlyIndexDynamicDispatch> {
    let db = engine.db();
    let owner_ty_application_expansion = owner_ty.application_expansion(db);
    let TermFunctionReduced::TypeOntology(ty_path) = owner_ty_application_expansion.function()
    else {
        todo!()
    };
    let refined_ty_path = ty_path.refine(db);
    let owner_ty_arguments = owner_ty_application_expansion.arguments(db);
    if let Some(index_signature) = ethereal_owner_ty_index_signature(
        engine,
        expr_idx,
        owner_ty,
        refined_ty_path,
        owner_ty_arguments,
        index_ty,
        indirections.final_place(),
    )
    .into_result_option()?
    {
        return JustOk(FlyIndexDynamicDispatch::new(indirections, index_signature));
    };
    // indirections
    match refined_ty_path {
        Left(prelude_ty_path) => match prelude_ty_path {
            PreludeTypePath::Indirection(prelude_indirection_ty_path) => {
                match prelude_indirection_ty_path {
                    PreludeIndirectionTypePath::Ref => todo!(),
                    PreludeIndirectionTypePath::RefMut => todo!(),
                    PreludeIndirectionTypePath::Leash => {
                        indirections.add(FlyIndirection::Leash);
                        if owner_ty_arguments.len() != 1 {
                            todo!()
                        }
                        ethereal_owner_ty_index_dispatch_aux(
                            engine,
                            expr_idx,
                            owner_ty_arguments[0],
                            index_ty,
                            indirections,
                        )
                    }
                    PreludeIndirectionTypePath::At => todo!(),
                }
            }
            _ => Nothing,
        },
        // ad hoc
        Right(_) => Nothing,
        // todo!(),
    }
}
