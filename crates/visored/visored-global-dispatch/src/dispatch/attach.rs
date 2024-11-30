use super::*;
use crate::{default_table::VdAttachKey, menu::VdGlobalDispatchMenu, *};
use default_table::VdDefaultGlobalDispatchTable;
use interned::db::InternerDb;
use lisp_csv::{
    expr::LpCsvExprData,
    file::{LpCsvFile, LpCsvFileData},
    row::LpCsvRow,
};
use menu::vd_global_dispatch_menu;
use visored_signature::{
    signature::{attach::VdAttachSignature, VdSignature},
    table::VdSignatureTable,
};
use visored_term::{
    instantiation::VdInstantiation,
    menu::{vd_ty_menu, VdTypeMenu},
    ty::VdType,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdAttachGlobalDispatch {
    Normal { signature: VdAttachSignature },
}

impl VdAttachGlobalDispatch {
    pub fn standard_defaults<'db>(
        ty_menu: &'db VdTypeMenu,
        global_dispatch_menu: &'db VdGlobalDispatchMenu,
    ) -> impl IntoIterator<Item = (VdAttachKey, VdAttachGlobalDispatch)> {
        let VdTypeMenu {
            nat,
            int,
            rat,
            real,
            complex,
            ..
        } = *ty_menu;
        let VdGlobalDispatchMenu {
            nat_to_the_power_of_nat,
            int_to_the_power_of_nat,
            rat_to_the_power_of_nat,
            real_to_the_power_of_nat,
            complex_to_the_power_of_nat,
            ..
        } = *global_dispatch_menu;
        let power = |base_ty, exponent_ty| VdAttachKey::Power {
            base_ty,
            exponent_ty,
        };
        [
            (power(nat, nat), nat_to_the_power_of_nat),
            (power(int, nat), int_to_the_power_of_nat),
            (power(rat, nat), rat_to_the_power_of_nat),
            (power(real, nat), real_to_the_power_of_nat),
            (power(complex, nat), complex_to_the_power_of_nat),
        ]
    }

    pub fn collect_from_lisp_csv_files<'a>(
        power_file: &'a LpCsvFile,
        signature_table: &'a VdSignatureTable,
        db: &'a InternerDb,
    ) -> impl Iterator<Item = (VdAttachKey, VdAttachGlobalDispatch)> + 'a {
        let LpCsvFileData::Rows(rows) = power_file.data();
        rows.iter().map(|row| {
            let LpCsvRow::SeparatedExprs(exprs) = row else {
                todo!()
            };
            let &[ref base_ty, ref exponent_ty, ref signature_ident] = exprs as &[_] else {
                todo!()
            };
            let base_ty = VdType::from_lp_csv_expr(base_ty, db);
            let exponent_ty = VdType::from_lp_csv_expr(exponent_ty, db);
            let LpCsvExprData::Ident(ref signature_ident) = signature_ident.data else {
                todo!()
            };
            let VdSignature::Attach(signature) = signature_table[signature_ident] else {
                todo!()
            };
            let dispatch = VdAttachGlobalDispatch::Normal { signature };
            (
                VdAttachKey::Power {
                    base_ty,
                    exponent_ty,
                },
                dispatch,
            )
        })
    }

    pub fn expr_ty(self) -> VdType {
        match self {
            VdAttachGlobalDispatch::Normal { signature } => signature.expr_ty(),
        }
    }
}

#[test]
fn vd_attach_global_dispatch_standard_defaults_works() {
    let db = &InternerDb::default();
    let table = VdDefaultGlobalDispatchTable::from_standard_lisp_csv_file_dir(db);
    let vd_ty_menu = &vd_ty_menu(db);
    let global_dispatch_menu = &vd_global_dispatch_menu(db);
    for (key, dispatch) in
        VdAttachGlobalDispatch::standard_defaults(vd_ty_menu, global_dispatch_menu)
    {
        match key {
            VdAttachKey::Power {
                base_ty,
                exponent_ty,
            } => {
                assert_eq!(
                    table.power_default_dispatch(base_ty, exponent_ty),
                    Some(dispatch)
                );
            }
        }
    }
}
