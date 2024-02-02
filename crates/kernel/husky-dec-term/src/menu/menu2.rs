use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermMenu2 {
    static_str_ref: DeclarativeTerm,
    ex_co_lifetime_to_ex_co_ty0_to_ty0: CurryDeclarativeTerm,
    ex_co_lifetime_to_ex_ct_ty0_to_ty0: CurryDeclarativeTerm,
    ex_co_lifetime_to_ex_inv_ty0_to_ty0: CurryDeclarativeTerm,
    parent: DeclarativeTermMenu1,
}

impl std::ops::Deref for DeclarativeTermMenu2 {
    type Target = DeclarativeTermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DeclarativeTermMenu2 {
    pub(crate) fn new(
        db: &::salsa::Db,
        toolchain: Toolchain,
        menu1: DeclarativeTermMenu1,
    ) -> DeclarativeTermResult<Self> {
        // db.it_item_path_term(item_path_menu(db,toolchain).as_ref()?.r32());
        Ok(DeclarativeTermMenu2 {
            static_str_ref: ApplicationDeclarativeTerm::new(
                db,
                menu1.static_ref_ty(),
                menu1.str_ty_path(),
            )
            .into(),
            ex_co_lifetime_to_ex_co_ty0_to_ty0: CurryDeclarativeTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                menu1.lifetime_ty().into(),
                menu1.explicit_covariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_ct_ty0_to_ty0: CurryDeclarativeTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                menu1.lifetime_ty().into(),
                menu1.explicit_contravariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_inv_ty0_to_ty0: CurryDeclarativeTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                menu1.lifetime_ty().into(),
                menu1.ex_inv_ty0_to_ty0().into(),
            ),
            parent: menu1,
        })
    }

    pub fn static_str_ref(&self) -> DeclarativeTerm {
        self.static_str_ref
    }

    pub fn ex_co_lifetime_to_ex_co_ty0_to_ty0(&self) -> CurryDeclarativeTerm {
        self.ex_co_lifetime_to_ex_co_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_ct_ty0_to_ty0(&self) -> CurryDeclarativeTerm {
        self.ex_co_lifetime_to_ex_ct_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_inv_ty0_to_ty0(&self) -> CurryDeclarativeTerm {
        self.ex_co_lifetime_to_ex_inv_ty0_to_ty0
    }
}
