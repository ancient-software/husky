pub use context::*;

use super::*;

/// representing term `x -> y`
#[salsa::interned(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TermRitchie {
    pub ritchie_kind: TermRitchieKind,
    #[return_ref]
    pub parameter_liasoned_tys: Vec<TermRitchieParameterLiasonedType>,
    pub return_ty: Term,
}

#[test]
fn term_ritchie_size_works() {
    assert_eq!(
        std::mem::size_of::<TermRitchie>(),
        std::mem::size_of::<u32>()
    );
}

impl TermRitchie {
    //// this constructor guarantees that the result is reduced and first-order valid
    /// returns Term instead of TermApplication because it might reduce to a non application term
    pub fn new(
        db: &dyn TermDb,
        ritchie_kind: TermRitchieKind,
        parameter_tys: impl IntoIterator<Item = TermRitchieParameterLiasonedType>,
        return_ty: Term,
    ) -> TermResult<TermRitchie> {
        todo!("check_application_validity(db, function, argument, shift)?");
        Ok(Self::new_unchecked(
            db,
            ritchie_kind,
            parameter_tys,
            return_ty,
        ))
    }

    //// this constructor guarantees that the result is reduced, not necessarily valid
    /// returns Term instead of TermApplication because it might reduce to a non application term
    pub(crate) fn new_unchecked(
        db: &dyn TermDb,
        ritchie_kind: TermRitchieKind,
        parameter_tys: impl IntoIterator<Item = TermRitchieParameterLiasonedType>,
        return_ty: Term,
    ) -> TermRitchie {
        Self::new_inner(
            db,
            ritchie_kind,
            parameter_tys
                .into_iter()
                .map(|parameter_liasoned_ty| parameter_liasoned_ty.reduce(db))
                .collect(),
            return_ty.reduce(db),
        )
    }
    //// this constructor guarantees that the result is reduced, not necessarily valid
    /// returns Term instead of TermApplication because it might reduce to a non application term
    fn new_unchecked2<E>(
        db: &dyn TermDb,
        ritchie_kind: TermRitchieKind,
        parameter_tys: impl IntoIterator<Item = Result<TermRitchieParameterLiasonedType, E>>,
        return_ty: Term,
    ) -> TermResult<TermRitchie>
    where
        TermError: From<E>,
    {
        Ok(Self::new_inner(
            db,
            ritchie_kind,
            parameter_tys
                .into_iter()
                .map(|parameter_liasoned_ty| Ok(parameter_liasoned_ty?.reduce(db)))
                .collect::<TermResult<Vec<_>>>()?,
            return_ty.reduce(db),
        ))
    }

    pub(super) fn reduce(self, db: &dyn TermDb) -> TermRitchie {
        todo!()
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
        for parameter_liasoned_ty in self.parameter_liasoned_tys(db) {
            parameter_liasoned_ty.check(db)?
        }
        self.return_ty(db).check_is_ins_ty0(db)
    }

    #[inline(always)]
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        raw_term_ritchie: RawTermRitchie,
    ) -> TermResult<Self> {
        term_ritchie_from_raw_unchecked(db, raw_term_ritchie)
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self.ritchie_kind(db) {
            TermRitchieKind::Fp => f.write_str("Fp(")?,
            TermRitchieKind::Fn => f.write_str("Fn(")?,
            TermRitchieKind::FnMut => f.write_str("FnMut(")?,
        }
        for (i, parameter_liasoned_ty) in self.parameter_liasoned_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_liasoned_ty.show_with_db_fmt(f, db, ctx)?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).show_with_db_fmt(f, db, ctx)
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_ritchie_from_raw_unchecked(
    db: &dyn TermDb,
    raw_term_ritchie: RawTermRitchie,
) -> TermResult<TermRitchie> {
    let t = |raw_term| {
        Term::from_raw_unchecked(db, raw_term, TermTypeExpectation::FinalDestinationEqsSort)
    };
    TermRitchie::new_unchecked2(
        db,
        raw_term_ritchie.ritchie_kind(db),
        raw_term_ritchie
            .parameter_tys(db)
            .iter()
            .map(|parameter_liasoned_ty| -> TermResult<_> {
                Ok(TermRitchieParameterLiasonedType {
                    ty: t(parameter_liasoned_ty.ty())?,
                })
            }),
        t(raw_term_ritchie.return_ty(db))?,
    )
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn check_term_ritchie_validity(
    db: &dyn TermDb,
    term_ritchie: TermRitchie,
) -> TermResult<()> {
    todo!()
}

impl<Db> salsa::DisplayWithDb<Db> for TermRitchie
where
    Db: TermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        match self.ritchie_kind(db) {
            TermRitchieKind::Fp => f.write_str("Fp(")?,
            TermRitchieKind::Fn => f.write_str("Fn(")?,
            TermRitchieKind::FnMut => f.write_str("FnMut(")?,
        }
        for (i, parameter_ty) in self.parameter_liasoned_tys(db).iter().enumerate() {
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
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct TermRitchieParameterLiasonedType {
    ty: Term,
}

impl TermRitchieParameterLiasonedType {
    fn check(self, db: &dyn TermDb) -> TermResult<()> {
        self.ty.check_is_ins_ty0(db)
    }

    fn reduce(self, db: &dyn TermDb) -> Self {
        Self {
            ty: self.ty.reduce(db),
        }
    }

    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl<Db> salsa::DisplayWithDb<Db> for TermRitchieParameterLiasonedType
where
    Db: TermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl TermRitchieParameterLiasonedType {
    pub fn new(ty: Term) -> Self {
        Self { ty }
    }

    pub fn ty(&self) -> Term {
        self.ty
    }
}

impl TermRitchie {
    fn substitute(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Term {
        todo!()
    }
}
