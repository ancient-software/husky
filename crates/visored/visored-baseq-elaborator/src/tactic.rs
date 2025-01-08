pub mod assumption;
pub mod comm_ring;
pub mod library_search;
pub mod term_trivial;

use crate::{
    elaborator::VdBsqElaboratorInner,
    expr::VdMirExprFld,
    hypothesis::{contradiction::VdBsqHypothesisResult, VdBsqHypothesisIdx},
};
use alt_option::AltOption;
use miracle::HasMiracleFull;

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqTactic {
    Assumption,
    TermTrivial,
    LibrarySearch,
    CommRing,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqTacticCall {
    Assumption,
    TermTrivial,
    LibrarySearch,
    CommRing,
}

impl VdBsqTactic {
    pub fn run<'db, 'sess>(
        &self,
        prop: VdMirExprFld<'sess>,
        elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    ) -> VdBsqHypothesisResult<'sess, AltOption<VdBsqHypothesisIdx<'sess>>> {
        match self {
            VdBsqTactic::Assumption => elaborator.assumption(prop),
            VdBsqTactic::TermTrivial => elaborator.term_trivial(prop),
            VdBsqTactic::LibrarySearch => elaborator.library_search(prop),
            VdBsqTactic::CommRing => elaborator.comm_ring(prop),
        }
    }
}
