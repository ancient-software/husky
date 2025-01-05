pub mod inum;
mod num;
pub mod prop;
pub mod rnum;

use self::{inum::*, prop::*, rnum::*};
use crate::elaborator::VdBsqElaboratorInner;
use either::*;
use floated_sequential::db::FloaterDb;
use floated_sequential::floated;
use num_relationship::VdBsqNumRelationshipPropTermKind;
use product::VdBsqProductInumTerm;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_mir_expr::{
    expr::{application::VdMirFunc, VdMirExprData, VdMirExprEntry},
    symbol::local_defn::{
        storage::VdMirSymbolLocalDefnStorage, VdMirSymbolLocalDefnHead, VdMirSymbolLocalDefnIdx,
    },
};
use visored_opr::separator::VdBaseSeparator;
use visored_term::term::literal::VdLiteralData;

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    Inum(VdBsqInumTerm<'sess>),
    Prop(VdBsqPropTerm<'sess>),
}

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNumTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    Inum(VdBsqInumTerm<'sess>),
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn product_or_non_product(
        self,
    ) -> Either<VdBsqProductInumTerm<'sess>, VdBsqNonProductNumTerm<'sess>> {
        match self {
            VdBsqNumTerm::Rnum(term) => todo!(),
            VdBsqNumTerm::Inum(term) => match term {
                VdBsqInumTerm::Atom(term) => Right(VdBsqNonProductNumTerm::AtomInum(term)),
                VdBsqInumTerm::Sum(term) => Right(VdBsqNonProductNumTerm::SumInum(term)),
                VdBsqInumTerm::Product(term) => Left(term),
            },
        }
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn num(self) -> Option<VdBsqNumTerm<'sess>> {
        match self {
            VdBsqTerm::Rnum(rnum) => Some(VdBsqNumTerm::Rnum(rnum)),
            VdBsqTerm::Inum(inum) => Some(VdBsqNumTerm::Inum(inum)),
            VdBsqTerm::Prop(_) => None,
        }
    }
}

impl<'sess> std::fmt::Debug for VdBsqTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'sess> std::fmt::Debug for VdBsqInumTermFld<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'sess> std::fmt::Debug for VdBsqNumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn calc_expr_term(
        &self,
        expr_entry: &VdMirExprEntry,
        symbol_local_defn_storage: &VdMirSymbolLocalDefnStorage,
    ) -> VdBsqTerm<'sess> {
        match *expr_entry.data() {
            VdMirExprData::Literal(vd_literal) => match *vd_literal.data() {
                VdLiteralData::Int128(i) => VdBsqTerm::Rnum(VdBsqRnumTerm::Int128(i)),
                VdLiteralData::BigInt(n) => todo!(),
                VdLiteralData::Float(_) => todo!(),
                VdLiteralData::SpecialConstant(vd_special_constant) => todo!(),
            },
            VdMirExprData::Variable(local_defn_idx) => {
                let lx_math_letter =
                    match *symbol_local_defn_storage.defn_arena()[local_defn_idx].head() {
                        VdMirSymbolLocalDefnHead::Letter(lx_math_letter) => lx_math_letter,
                    };
                if expr_entry.ty().is_numeric(self.eterner_db()) {
                    if let Some(_) = self.eval_variable() {
                        todo!()
                    } else {
                        VdBsqTerm::new_numeric_variable(local_defn_idx, self.floater_db())
                    }
                } else {
                    todo!()
                }
            }
            VdMirExprData::Application {
                function,
                arguments,
            } => match function {
                VdMirFunc::NormalBasePrefixOpr(vd_base_prefix_opr_signature) => todo!(),
                VdMirFunc::NormalBaseSeparator(vd_base_separator_signature) => todo!(),
                VdMirFunc::NormalBaseBinaryOpr(vd_base_binary_opr_signature) => todo!(),
                VdMirFunc::Power(vd_power_signature) => {
                    assert_eq!(arguments.len(), 2);
                    let Some(base) = self.expr_fld(arguments.first().unwrap()).term().num() else {
                        todo!()
                    };
                    let Some(exponent) = self.expr_fld(arguments.last().unwrap()).term().num()
                    else {
                        todo!()
                    };
                    match base.product_or_non_product() {
                        Either::Left(base) => todo!(),
                        Either::Right(base) => {
                            VdBsqTerm::new_power(base, exponent, self.floater_db())
                        }
                    }
                }
                VdMirFunc::InSet => todo!(),
                VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
                VdMirFunc::NormalBaseFrac(vd_base_binary_opr_signature) => todo!(),
            },
            VdMirExprData::FoldingSeparatedList {
                leader,
                ref followers,
            } => {
                todo!()
            }
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_separator_and_signature,
            } => match joined_separator_and_signature {
                Some(joined_separator_and_signature) => todo!(),
                None => {
                    use VdBsqNumRelationshipPropTermKind::*;

                    let (func, follower) = *followers.first().unwrap();
                    let num_relationship = |slf: &Self, kind| {
                        VdBsqTerm::new_num_relationship(
                            slf.expr_fld(leader).term().num().unwrap(),
                            kind,
                            slf.expr_fld(follower).term().num().unwrap(),
                            slf.floater_db(),
                        )
                    };
                    match func {
                        VdMirFunc::NormalBasePrefixOpr(signature) => todo!(),
                        VdMirFunc::NormalBaseSeparator(signature) => match signature.opr() {
                            VdBaseSeparator::Space => todo!(),
                            VdBaseSeparator::Comma => todo!(),
                            VdBaseSeparator::Semicolon => todo!(),
                            VdBaseSeparator::Add => todo!(),
                            VdBaseSeparator::Mul => todo!(),
                            VdBaseSeparator::Dot => todo!(),
                            VdBaseSeparator::Eq => num_relationship(self, Eq),
                            VdBaseSeparator::Ne => num_relationship(self, Ne),
                            VdBaseSeparator::Lt => num_relationship(self, Lt),
                            VdBaseSeparator::Gt => num_relationship(self, Gt),
                            VdBaseSeparator::Le => num_relationship(self, Le),
                            VdBaseSeparator::Ge => num_relationship(self, Ge),
                            VdBaseSeparator::Subset => todo!(),
                            VdBaseSeparator::Supset => todo!(),
                            VdBaseSeparator::Subseteq => todo!(),
                            VdBaseSeparator::Supseteq => todo!(),
                            VdBaseSeparator::Subseteqq => todo!(),
                            VdBaseSeparator::Supseteqq => todo!(),
                            VdBaseSeparator::Subsetneq => todo!(),
                            VdBaseSeparator::Supsetneq => todo!(),
                            VdBaseSeparator::In => todo!(),
                            VdBaseSeparator::Notin => todo!(),
                            VdBaseSeparator::Times => todo!(),
                            VdBaseSeparator::Otimes => todo!(),
                        },
                        VdMirFunc::NormalBaseBinaryOpr(signature) => todo!(),
                        VdMirFunc::Power(signature) => todo!(),
                        VdMirFunc::InSet => todo!(),
                        VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
                        VdMirFunc::NormalBaseFrac(vd_base_binary_opr_signature) => todo!(),
                    }
                }
            },
            VdMirExprData::ItemPath(vd_item_path) => todo!(),
        }
    }
}
