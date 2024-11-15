use crate::menu::VdGlobalDispatchMenu;

use super::*;
use visored_opr::{menu::VdOprMenu, separator::VdBaseSeparator};
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;
use visored_zfc_ty::{menu::VdZfcTypeMenu, ty::VdZfcType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VdSeparatorGlobalDispatch {
    Normal {
        base_separator: VdBaseSeparator,
        signature: VdBaseSeparatorSignature,
    },
    InSet {
        expr_ty: VdZfcType,
    },
}

impl VdSeparatorGlobalDispatch {
    pub(crate) fn standard_defaults<'db>(
        zfc_ty_menu: &'db VdZfcTypeMenu,
        vd_opr_menu: &'db VdOprMenu,
        global_dispatch_menu: &'db VdGlobalDispatchMenu,
    ) -> impl IntoIterator<
        Item = (
            (VdZfcType, VdBaseSeparator, VdZfcType),
            &'db VdSeparatorGlobalDispatch,
        ),
    > {
        let VdZfcTypeMenu {
            nat,
            int,
            rat,
            real,
            complex,
            set,
            prop,
        } = *zfc_ty_menu;
        let VdOprMenu {
            add,
            space,
            eq,
            le,
            ge,
            r#in,
        } = *vd_opr_menu;
        let VdGlobalDispatchMenu {
            nat_add,
            int_add,
            rat_add,
            real_add,
            complex_add,
            nat_space_mul,
            int_space_mul,
            rat_space_mul,
            real_space_mul,
            complex_space_mul,
            nat_to_the_power_of_nat,
            int_to_the_power_of_nat,
            rat_to_the_power_of_nat,
            real_to_the_power_of_nat,
            complex_to_the_power_of_nat,
            nat_eq,
            int_eq,
            rat_eq,
            real_eq,
            complex_eq,
            nat_le,
            int_le,
            rat_le,
            real_le,
            nat_ge,
            int_ge,
            rat_ge,
            real_ge,
            in_set,
        } = global_dispatch_menu;
        [
            // ## add
            // ### nat
            ((nat, add, nat), nat_add),
            // ### int
            ((nat, add, int), int_add),
            ((int, add, nat), int_add),
            ((int, add, int), int_add),
            // ### rat
            ((nat, add, rat), rat_add),
            ((int, add, rat), rat_add),
            ((rat, add, nat), rat_add),
            ((rat, add, int), rat_add),
            ((rat, add, rat), rat_add),
            // ### real
            ((nat, add, real), real_add),
            ((int, add, real), real_add),
            ((rat, add, real), real_add),
            ((real, add, nat), real_add),
            ((real, add, int), real_add),
            ((real, add, rat), real_add),
            ((real, add, real), real_add),
            // ### complex
            ((nat, add, complex), complex_add),
            ((int, add, complex), complex_add),
            ((rat, add, complex), complex_add),
            ((real, add, complex), complex_add),
            ((complex, add, nat), complex_add),
            ((complex, add, int), complex_add),
            ((complex, add, rat), complex_add),
            ((complex, add, rat), complex_add),
            ((complex, add, complex), complex_add),
            // ## mul
            // ### nat
            ((nat, space, nat), nat_space_mul),
            // ### int
            ((nat, space, int), int_space_mul),
            ((int, space, nat), int_space_mul),
            ((int, space, int), int_space_mul),
            // ### rat
            ((nat, space, rat), rat_space_mul),
            ((int, space, rat), rat_space_mul),
            ((rat, space, nat), rat_space_mul),
            ((rat, space, int), rat_space_mul),
            ((rat, space, rat), rat_space_mul),
            // ### real
            ((nat, space, real), real_space_mul),
            ((int, space, real), real_space_mul),
            ((rat, space, real), real_space_mul),
            ((real, space, nat), real_space_mul),
            ((real, space, int), real_space_mul),
            ((real, space, rat), real_space_mul),
            ((real, space, real), real_space_mul),
            // ### complex
            ((nat, space, complex), complex_space_mul),
            ((int, space, complex), complex_space_mul),
            ((rat, space, complex), complex_space_mul),
            ((real, space, complex), complex_space_mul),
            ((complex, space, nat), complex_space_mul),
            ((complex, space, int), complex_space_mul),
            ((complex, space, rat), complex_space_mul),
            ((complex, space, real), complex_space_mul),
            ((complex, space, complex), complex_space_mul),
            // ## eq
            // ### nat
            ((nat, eq, nat), nat_eq),
            // ### int
            ((nat, eq, int), int_eq),
            ((int, eq, nat), int_eq),
            ((int, eq, int), int_eq),
            // ### rat
            ((nat, eq, rat), rat_eq),
            ((int, eq, rat), rat_eq),
            ((rat, eq, nat), rat_eq),
            ((rat, eq, int), rat_eq),
            ((rat, eq, rat), rat_eq),
            // ### real
            ((nat, eq, real), real_eq),
            ((int, eq, real), real_eq),
            ((rat, eq, real), real_eq),
            ((real, eq, nat), real_eq),
            ((real, eq, int), real_eq),
            ((real, eq, rat), real_eq),
            ((real, eq, real), real_eq),
            // ### complex
            ((nat, eq, complex), real_eq),
            ((rat, eq, complex), real_eq),
            ((int, eq, complex), real_eq),
            ((real, eq, complex), real_eq),
            ((complex, eq, nat), real_eq),
            ((complex, eq, rat), real_eq),
            ((complex, eq, int), real_eq),
            ((complex, eq, real), real_eq),
            ((complex, eq, complex), real_eq),
            // ## le
            // ### nat
            ((nat, le, nat), nat_le),
            // ### int
            ((nat, le, int), int_le),
            ((int, le, nat), int_le),
            ((int, le, int), int_le),
            // ### rat
            ((nat, le, rat), rat_le),
            ((int, le, rat), rat_le),
            ((rat, le, nat), rat_le),
            ((rat, le, int), rat_le),
            ((rat, le, rat), rat_le),
            // ### real
            ((nat, le, real), real_le),
            ((int, le, real), real_le),
            ((rat, le, real), real_le),
            ((real, le, nat), real_le),
            ((real, le, int), real_le),
            ((real, le, rat), real_le),
            ((real, le, real), real_le),
            // ## ge
            // ### nat
            ((nat, ge, nat), nat_ge),
            // ### int
            ((nat, ge, int), int_ge),
            ((int, ge, nat), int_ge),
            ((int, ge, int), int_ge),
            // ### rat
            ((nat, ge, rat), rat_ge),
            ((int, ge, rat), rat_ge),
            ((rat, ge, nat), rat_ge),
            ((rat, ge, int), rat_ge),
            ((rat, ge, rat), rat_ge),
            // ### real
            ((nat, ge, real), real_ge),
            ((int, ge, real), real_ge),
            ((rat, ge, real), real_ge),
            ((real, ge, nat), real_ge),
            ((real, ge, int), real_ge),
            ((real, ge, rat), real_ge),
            ((real, ge, real), real_ge),
            // ## in
            ((nat, r#in, set), in_set),
            ((int, r#in, set), in_set),
            ((rat, r#in, set), in_set),
            ((real, r#in, set), in_set),
            ((complex, r#in, set), in_set),
        ]
    }
}
