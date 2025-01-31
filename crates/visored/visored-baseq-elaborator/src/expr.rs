mod helpers;

use floated_sequential::{db::FloaterDb, floated};
use latex_math_letter::letter::LxMathLetter;
use smallvec::*;
use visored_entity_path::path::VdItemPath;
use visored_mir_expr::{
    expr::{
        application::VdMirFunc, VdMirExprArena, VdMirExprArenaRef, VdMirExprData, VdMirExprEntry,
        VdMirExprIdx, VdMirExprIdxRange, VdMirExprMap, VdMirExprOrderedMap,
    },
    hypothesis::constructor::VdMirHypothesisConstructor,
    region::VdMirExprRegionDataRef,
    symbol::local_defn::{
        storage::VdMirSymbolLocalDefnStorage, VdMirSymbolLocalDefnHead, VdMirSymbolLocalDefnIdx,
    },
};
use visored_mir_opr::separator::VdMirBaseSeparator;
use visored_opr::precedence::{VdPrecedence, VdPrecedenceRange};
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;
use visored_term::{
    term::literal::{VdLiteral, VdLiteralData},
    ty::VdType,
};

use crate::{
    elaborator::VdBsqElaboratorInner,
    hypothesis::VdBsqHypothesisIdx,
    session::VdBsqSession,
    term::{litnum::VdBsqLitnumTerm, VdBsqTerm},
};

#[floated]
pub struct VdBsqExprFld<'sess> {
    #[return_ref]
    pub data: VdBsqExprFldData<'sess>,
    pub ty: VdType,
    pub term: VdBsqTerm<'sess>,
    pub expected_ty: Option<VdType>,
}

impl<'sess> std::fmt::Debug for VdBsqExprFld<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VdMirExprFld(`")?;
        self.show_fmt(VdPrecedenceRange::ANY, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqExprFld<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if precedence_range.contains(self.data().outer_precedence()) {
            self.show_fmt_inner(f)
        } else {
            f.write_str("(")?;
            self.show_fmt_inner(f)?;
            f.write_str(")")
        }
    }

    fn show_fmt_inner(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data() {
            VdBsqExprFldData::Literal(literal) => literal.show(f),
            VdBsqExprFldData::Variable(letter, _) => {
                write!(f, "{}", letter.unicode())
            }
            VdBsqExprFldData::Application {
                function,
                arguments,
            } => match function {
                VdMirFunc::NormalBasePrefixOpr(signature) => todo!(),
                VdMirFunc::NormalBaseSeparator(signature) => todo!(),
                VdMirFunc::NormalBaseBinaryOpr(signature) => {
                    let opr = signature.opr;
                    arguments[0].show_fmt(opr.left_precedence_range(), f)?;
                    f.write_str(" ")?;
                    f.write_str(opr.unicode())?;
                    f.write_str(" ")?;
                    arguments[1].show_fmt(opr.right_precedence_range(), f)?;
                    Ok(())
                }
                VdMirFunc::Power(signature) => {
                    match arguments[1].data() {
                        VdBsqExprFldData::Literal(literal) => match *literal.data() {
                            VdLiteralData::Int128(i) if i >= 0 && i < 10 => {
                                use husky_unicode_symbols::superscript::superscript;

                                // use unicode to show the superscript
                                let superscript = superscript(i as u8).unwrap();
                                arguments[0].show_fmt(VdPrecedenceRange::ATOM, f)?;
                                write!(f, "{}", superscript)?;
                                return Ok(());
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                    arguments[0].show_fmt(VdPrecedenceRange::ATOM, f)?;
                    write!(f, "^{{")?;
                    arguments[1].show_fmt(VdPrecedenceRange::ANY, f)?;
                    f.write_str("}}")
                }
                VdMirFunc::InSet => todo!(),
                VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
            },
            VdBsqExprFldData::FoldingSeparatedList { leader, followers } => {
                let VdMirFunc::NormalBaseSeparator(signature) = followers.first().unwrap().0 else {
                    todo!("maybe non base separator?")
                };
                let precedence_range = signature.opr().left_precedence_range();
                leader.show_fmt(precedence_range, f)?;
                for (func, follower) in followers {
                    let VdMirFunc::NormalBaseSeparator(signature) = func else {
                        todo!("maybe non base separator?")
                    };
                    f.write_str(" ")?;
                    signature.opr().show_fmt(f)?;
                    f.write_str(" ")?;
                    follower.show_fmt(precedence_range, f)?;
                }
                Ok(())
            }
            VdBsqExprFldData::ChainingSeparatedList {
                leader,
                followers,
                joined_signature,
            } => {
                let VdMirFunc::NormalBaseSeparator(signature) = followers.first().unwrap().0 else {
                    todo!("maybe non base separator?")
                };
                let precedence_range = signature.opr().left_precedence_range();
                leader.show_fmt(precedence_range, f)?;
                for (func, follower) in followers {
                    let VdMirFunc::NormalBaseSeparator(signature) = func else {
                        todo!("maybe non base separator?")
                    };
                    f.write_str(" ")?;
                    signature.opr().show_fmt(f)?;
                    f.write_str(" ")?;
                    follower.show_fmt(precedence_range, f)?;
                }
                Ok(())
            }
            VdBsqExprFldData::ItemPath(vd_item_path) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqExprFldData<'sess> {
    Literal(VdLiteral),
    Variable(LxMathLetter, VdMirSymbolLocalDefnIdx),
    Application {
        function: VdMirFunc,
        arguments: VdMirExprFlds<'sess>,
    },
    FoldingSeparatedList {
        leader: VdBsqExprFld<'sess>,
        /// TODO: should we use VdBaseSeparatorSignature instead?
        followers: SmallVec<[(VdMirFunc, VdBsqExprFld<'sess>); 4]>,
    },
    ChainingSeparatedList {
        leader: VdBsqExprFld<'sess>,
        followers: SmallVec<[(VdMirFunc, VdBsqExprFld<'sess>); 4]>,
        joined_signature: Option<VdBaseSeparatorSignature>,
    },
    ItemPath(VdItemPath),
}

pub type VdBsqExprFollowers<'sess> = SmallVec<[(VdMirFunc, VdBsqExprFld<'sess>); 4]>;

impl<'sess> VdBsqExprFldData<'sess> {
    pub fn outer_precedence(&self) -> VdPrecedence {
        match self {
            VdBsqExprFldData::Literal(_) => VdPrecedence::ATOM,
            VdBsqExprFldData::Variable(_, _) => VdPrecedence::ATOM,
            VdBsqExprFldData::Application { function, .. } => function.outer_precedence(),
            VdBsqExprFldData::FoldingSeparatedList { leader, followers } => {
                followers[0].0.outer_precedence()
            }
            VdBsqExprFldData::ChainingSeparatedList {
                leader,
                followers,
                joined_signature,
            } => followers.first().unwrap().0.outer_precedence(),
            VdBsqExprFldData::ItemPath(vd_item_path) => todo!(),
        }
    }
}

pub type VdMirExprFlds<'sess> = SmallVec<[VdBsqExprFld<'sess>; 4]>;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn cache_expr_fld(&mut self, expr_idx: VdMirExprIdx, region_data: VdMirExprRegionDataRef) {
        if self.expr_to_fld_map().has(expr_idx) {
            return;
        }
        let expr_entry = &region_data.expr_arena[expr_idx];
        let symbol_local_defn_storage = region_data.symbol_local_defn_storage;
        let expr_data = self.calc_expr_fld_data(expr_entry, symbol_local_defn_storage);
        let ty = expr_entry.ty();
        let term = self.calc_expr_term(&expr_data, ty);
        let db = self.session().floater_db();
        let expected_ty = expr_entry.expected_ty();
        let expr_fld = VdBsqExprFld::new_inner(expr_data, ty, term, expected_ty, db);
        self.save_expr_fld(expr_idx, expr_fld);
    }

    fn calc_expr_fld_data(
        &self,
        entry: &VdMirExprEntry,
        symbol_local_defn_storage: &VdMirSymbolLocalDefnStorage,
    ) -> VdBsqExprFldData<'sess> {
        match *entry.data() {
            VdMirExprData::Literal(vd_literal) => VdBsqExprFldData::Literal(vd_literal),
            VdMirExprData::Variable(local_defn_idx) => {
                let lx_math_letter =
                    match *symbol_local_defn_storage.defn_arena()[local_defn_idx].head() {
                        VdMirSymbolLocalDefnHead::Letter(lx_math_letter) => lx_math_letter,
                    };
                VdBsqExprFldData::Variable(lx_math_letter, local_defn_idx)
            }
            VdMirExprData::Application {
                function,
                arguments,
            } => VdBsqExprFldData::Application {
                function,
                arguments: arguments
                    .into_iter()
                    .map(|arg| self.expr_fld(arg))
                    .collect(),
            },
            VdMirExprData::FoldingSeparatedList {
                leader,
                ref followers,
            } => VdBsqExprFldData::FoldingSeparatedList {
                leader: self.expr_fld(leader),
                followers: followers
                    .iter()
                    .map(|&(func, follower)| (func, self.expr_fld(follower)))
                    .collect(),
            },
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_signature,
            } => VdBsqExprFldData::ChainingSeparatedList {
                leader: self.expr_fld(leader),
                followers: followers
                    .iter()
                    .map(|&(func, follower)| (func, self.expr_fld(follower)))
                    .collect(),
                joined_signature,
            },
            VdMirExprData::ItemPath(vd_item_path) => VdBsqExprFldData::ItemPath(vd_item_path),
        }
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn mk_expr(
        &self,
        expr_data: VdBsqExprFldData<'sess>,
        ty: VdType,
        expected_ty: Option<VdType>,
    ) -> VdBsqExprFld<'sess> {
        let term = self.calc_expr_term(&expr_data, ty);
        let db = self.session().floater_db();
        VdBsqExprFld::new_inner(expr_data, ty, term, expected_ty, db)
    }

    pub(crate) fn mk_zero(&self, expected_ty: Option<VdType>) -> VdBsqExprFld<'sess> {
        self.mk_expr(
            VdBsqExprFldData::Literal(self.term_menu().zero),
            self.ty_menu().nat,
            expected_ty,
        )
    }

    pub(crate) fn mk_lit(
        &self,
        litnum: VdBsqLitnumTerm<'sess>,
        ty: VdType,
        expected_ty: Option<VdType>,
    ) -> VdBsqExprFld<'sess> {
        let db = self.session().eterner_db();
        let lit = match litnum {
            VdBsqLitnumTerm::Int128(i) => VdLiteral::new(VdLiteralData::Int128(i), db),
            VdBsqLitnumTerm::BigInt(vd_bsq_big_int) => todo!(),
            VdBsqLitnumTerm::Frac128(vd_bsq_frac128) => todo!(),
        };
        self.mk_expr(VdBsqExprFldData::Literal(lit), ty, expected_ty)
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn transcribe_expr(
        &self,
        expr: VdBsqExprFld<'sess>,
        hypothesis_constructor: &mut VdMirHypothesisConstructor<'db, VdBsqHypothesisIdx<'sess>>,
    ) -> VdMirExprIdx {
        let data = match *expr.data() {
            VdBsqExprFldData::Literal(lit) => VdMirExprData::Literal(lit),
            VdBsqExprFldData::Variable(_, symbol) => VdMirExprData::Variable(symbol),
            VdBsqExprFldData::Application {
                function,
                ref arguments,
            } => todo!(),
            VdBsqExprFldData::FoldingSeparatedList {
                leader,
                ref followers,
            } => todo!(),
            VdBsqExprFldData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_signature,
            } => VdMirExprData::ChainingSeparatedList {
                leader: self.transcribe_expr(leader, hypothesis_constructor),
                followers: followers
                    .iter()
                    .map(|&(func, follower)| {
                        (func, self.transcribe_expr(follower, hypothesis_constructor))
                    })
                    .collect(),
                joined_signature,
            },
            VdBsqExprFldData::ItemPath(vd_item_path) => todo!(),
        };
        let ty = expr.ty();
        let expected_ty = expr.expected_ty();
        hypothesis_constructor.construct_new_expr(data, ty, expected_ty)
    }
}
