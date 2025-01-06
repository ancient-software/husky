use super::*;
use crate::term::{
    builder::sum::VdBsqSumBuilder, inum::product::VdBsqProductInumTermBase, rnum::VdBsqRnumTerm,
};
use crate::term::{
    inum::{sum::VdBsqSumInumTerm, VdBsqNonSumInumTerm},
    num::VdBsqNumTerm,
};
use miracle::error::MiracleAltMaybeResult;
use monadic_fold::engine::{IsMonadicFoldEngineScheme, IsMonadicFoldEngineSchemeFull as _};
use std::marker::PhantomData;

struct Scheme<'db, 'sess>(PhantomData<&'db ()>, PhantomData<&'sess ()>);

impl<'db, 'sess> IsMonadicFoldEngineScheme for Scheme<'db, 'sess>
where
    'db: 'sess,
{
    type Engine = VdBsqElaboratorInner<'db, 'sess>;

    type State = VdBsqSumBuilder<'sess>;

    type Item = (VdBsqNonSumInumTerm<'sess>, VdBsqRnumTerm);

    type Output = MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>;

    fn foldm_step(
        elaborator: &mut Self::Engine,
        sum_builder: VdBsqSumBuilder<'sess>,
        (term, rnum0): (VdBsqNonSumInumTerm<'sess>, VdBsqRnumTerm),
        f: &impl Fn(&mut VdBsqElaboratorInner<'db, 'sess>, VdBsqSumBuilder<'sess>) -> Self::Output,
    ) -> Self::Output {
        let db = elaborator.floater_db();
        match term {
            VdBsqNonSumInumTerm::Atom(vd_bsq_atom_inum_term) => todo!(),
            VdBsqNonSumInumTerm::Product(base) => {
                fold_product(elaborator, base.exponentials(), &|elaborator, expansion| {
                    let mut sum_builder = sum_builder.clone();
                    let expansion = expansion.expect("expansion shouldn't be None after folding");
                    for (rnum, exponentials) in expansion {
                        sum_builder.add_general_product(
                            rnum0.mul(rnum, db),
                            VdBsqProductInumTermBase::from_parts(exponentials, db),
                        );
                    }
                    f(elaborator, sum_builder)
                })
            }
        }
    }
}

pub(super) fn fold_sum<'db, 'sess>(
    engine: &mut VdBsqElaboratorInner<'db, 'sess>,
    terms: &[(VdBsqNonSumInumTerm<'sess>, VdBsqRnumTerm)],
    builder: VdBsqSumBuilder<'sess>,
    f: &impl Fn(
        &mut VdBsqElaboratorInner<'db, 'sess>,
        VdBsqSumBuilder<'sess>,
    ) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>,
) -> MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>> {
    Scheme::foldm(engine, builder, terms.iter().copied(), f)
}
