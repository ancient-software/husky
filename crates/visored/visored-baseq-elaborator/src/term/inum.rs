pub mod atom;
pub mod product;
pub mod sum;

use self::{atom::*, product::*, sum::*};
use super::*;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqInumTerm<'sess> {
    Atom(VdBsqAtomInumTerm<'sess>),
    Sum(VdBsqSumInumTerm<'sess>),
    Product(VdBsqProductInumTerm<'sess>),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNonProductNumTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    AtomInum(VdBsqAtomInumTerm<'sess>),
    SumInum(VdBsqSumInumTerm<'sess>),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNonSumInumTerm<'sess> {
    Atom(VdBsqAtomInumTerm<'sess>),
    Product(VdBsqProductInumTerm<'sess>),
}

#[floated]
pub struct VdBsqInumTermFld<'sess> {
    #[return_ref]
    pub data: VdBsqInumTermData<'sess>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqInumTermData<'sess> {
    Atom(VdBsqInumAtomTermData),
    Sum(VdBsqInumSumTermData<'sess>),
    Product(VdBsqProductInumTermData<'sess>),
}

pub type VdBsqNonProductNumTermMap<'sess, T> =
    OrderedSmallVecPairMap<VdBsqNonProductNumTerm<'sess>, T, 4>;
pub type VdBsqInumNonSumTermMap<'sess, T> =
    OrderedSmallVecPairMap<VdBsqNonSumInumTerm<'sess>, T, 4>;
pub type VdBsqInumMonomialCoefficients<'sess> = VdBsqInumNonSumTermMap<'sess, VdBsqRnumTerm>;
pub type VdBsqInumAtomExponentials<'sess> = VdBsqNonProductNumTermMap<'sess, VdBsqNumTerm<'sess>>;
