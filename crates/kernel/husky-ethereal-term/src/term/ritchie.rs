mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;

/// representing term `x -> y`
#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub struct EtherealTermRitchie {
    pub ritchie_kind: RitchieKind,
    #[return_ref]
    pub parameter_contracted_tys: Vec<EtherealTermRitchieParameter>,
    pub return_ty: EtherealTerm,
}

#[test]
fn term_ritchie_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermRitchie>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermRitchie {
    //// this constructor guarantees that the result is reduced and first-order valid
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    pub fn new(
        db: &dyn EtherealTermDb,
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: impl IntoIterator<Item = EtherealTermRitchieParameter>,
        return_ty: EtherealTerm,
    ) -> EtherealTermResult<EtherealTermRitchie> {
        // todo!("check_application_validity(db, function, argument, shift)?");
        Ok(Self::new_unchecked(
            db,
            ritchie_kind,
            parameter_contracted_tys,
            return_ty,
        ))
    }

    /// this constructor guarantees that the result is reduced, not necessarily valid;
    ///
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    pub(crate) fn new_unchecked(
        db: &dyn EtherealTermDb,
        ritchie_kind: RitchieKind,
        parameter_tys: impl IntoIterator<Item = EtherealTermRitchieParameter>,
        return_ty: EtherealTerm,
    ) -> EtherealTermRitchie {
        Self::new_inner(
            db,
            ritchie_kind,
            parameter_tys
                .into_iter()
                .map(|parameter_contracted_ty| parameter_contracted_ty.reduce(db))
                .collect(),
            return_ty.reduce(db),
        )
    }
    /// this constructor guarantees that the result is reduced, not necessarily valid
    ///
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    fn new_unchecked2<E>(
        db: &dyn EtherealTermDb,
        ritchie_kind: RitchieKind,
        parameter_tys: impl IntoIterator<Item = Result<EtherealTermRitchieParameter, E>>,
        return_ty: EtherealTerm,
    ) -> EtherealTermResult<EtherealTermRitchie>
    where
        EtherealTermError: From<E>,
    {
        Ok(Self::new_inner(
            db,
            ritchie_kind,
            parameter_tys
                .into_iter()
                .map(|parameter_contracted_ty| Ok(parameter_contracted_ty?.reduce(db)))
                .collect::<EtherealTermResult<Vec<_>>>()?,
            return_ty.reduce(db),
        ))
    }

    pub(super) fn reduce(self, db: &dyn EtherealTermDb) -> EtherealTermRitchie {
        todo!()
    }

    #[inline(always)]
    pub(crate) fn from_declarative(
        db: &dyn EtherealTermDb,
        declarative_term_ritchie: DeclarativeTermRitchie,
    ) -> EtherealTermResult<Self> {
        ethereal_term_ritchie_from_declarative_term_ritchie(db, declarative_term_ritchie)
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self.ritchie_kind(db) {
            RitchieKind::FnType => f.write_str("fn(")?,
            RitchieKind::FnTrait => f.write_str("Fn(")?,
            RitchieKind::FnMutTrait => f.write_str("FnMut(")?,
            RitchieKind::GnType => f.write_str("gn(")?,
        }
        for (i, parameter_contracted_ty) in self.parameter_contracted_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_contracted_ty.show_with_db_fmt(f, db, ctx)?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).show_with_db_fmt(f, db, ctx)
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_ritchie_from_declarative_term_ritchie(
    db: &dyn EtherealTermDb,
    declarative_term_ritchie: DeclarativeTermRitchie,
) -> EtherealTermResult<EtherealTermRitchie> {
    EtherealTermRitchie::new_unchecked2(
        db,
        declarative_term_ritchie.ritchie_kind(db),
        declarative_term_ritchie
            .params(db)
            .iter()
            .map(|param| -> EtherealTermResult<_> {
                Ok(match param {
                    DeclarativeTermRitchieParameter::Regular(param) => {
                        EtherealTermRitchieRegularParameter::from_declarative(db, param)?.into()
                    }
                    DeclarativeTermRitchieParameter::Variadic(_) => todo!(),
                    DeclarativeTermRitchieParameter::Keyed(_) => todo!(),
                })
            }),
        EtherealTerm::ty_from_declarative(db, declarative_term_ritchie.return_ty(db))?,
    )
}

impl<Db> salsa::DisplayWithDb<Db> for EtherealTermRitchie
where
    Db: EtherealTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        match self.ritchie_kind(db) {
            RitchieKind::FnType => f.write_str("fn(")?,
            RitchieKind::FnTrait => f.write_str("Fn(")?,
            RitchieKind::FnMutTrait => f.write_str("FnMut(")?,
            RitchieKind::GnType => f.write_str("gn(")?,
        }
        for (i, parameter_ty) in self.parameter_contracted_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_ty.display_with_db_fmt(f, db, level.next())?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).display_with_db_fmt(f, db, level.next())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
#[enum_class::from_variants]
pub enum EtherealTermRitchieParameter {
    Regular(EtherealTermRitchieRegularParameter),
}

impl EtherealTermRitchieParameter {
    fn reduce(self, db: &dyn EtherealTermDb) -> Self {
        match self {
            EtherealTermRitchieParameter::Regular(param) => param.reduce(db).into(),
        }
    }

    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self {
            EtherealTermRitchieParameter::Regular(param) => param.show_with_db_fmt(f, db, ctx),
        }
    }
}

impl<Db> salsa::DisplayWithDb<Db> for EtherealTermRitchieParameter
where
    Db: EtherealTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        match self {
            EtherealTermRitchieParameter::Regular(param) => param.display_with_db_fmt(f, db, level),
        }
    }
}

impl EtherealTermRitchieParameter {
    // pub fn new(contract: Contract, ty: EtherealTerm) -> Self {
    //     Self { contract, ty }
    // }

    // pub fn contract(&self) -> Contract {
    //     self.contract
    // }

    pub fn ty(&self) -> EtherealTerm {
        match self {
            EtherealTermRitchieParameter::Regular(param) => param.ty(),
        }
    }
}

impl EtherealTermRitchie {
    fn substitute(self, db: &dyn EtherealTermDb, substituation: &TermSubstitution) -> EtherealTerm {
        todo!()
    }
}
