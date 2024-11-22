pub mod attach;
pub mod binary_opr;
pub mod frac;
pub mod function;
pub mod prefix_opr;
pub mod separator;
pub mod sqrt;
pub mod suffix_opr;

use self::{
    attach::*,
    binary_opr::{base::*, composite::*, *},
    frac::*,
    function::*,
    prefix_opr::*,
    separator::*,
    sqrt::*,
};
use crate::*;
use lisp_csv::expr::{LpCsvExpr, LpCsvExprData};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VdSignature {
    Attach(VdAttachSignature),
    BinaryOpr(VdBinaryOprSignature),
    Frac(VdFracSignature),
    Function(VdFunctionSignature),
    PrefixOpr(VdPrefixOprSignature),
    Separator(VdSeparatorSignature),
    Sqrt(VdSqrtSignature),
}

impl VdSignature {
    pub fn from_lp_csv_exprs(exprs: &[LpCsvExpr], db: &::salsa::Db) -> Self {
        assert_eq!(exprs.len(), 2);
        let instantiation = VdInstantiation::from_lp_csv_expr(&exprs[0], db);
        let (
            LpCsvExpr {
                data: LpCsvExprData::Ident(variant_ident),
                ..
            },
            args,
        ) = exprs[1].application_expansion()
        else {
            unreachable!()
        };
        match variant_ident.as_str() {
            "base_binary_opr" => {
                assert_eq!(args.len(), 3);
                VdBaseBinaryOprSignature {
                    instantiation,
                    lopd_ty: VdType::from_lp_csv_expr(&args[0], db),
                    ropd_ty: VdType::from_lp_csv_expr(&args[1], db),
                    expr_ty: VdType::from_lp_csv_expr(&args[2], db),
                }
                .into()
            }
            s => todo!("s = {s:?} not handled"),
        }
    }
}
