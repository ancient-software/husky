use super::*;
use husky_eth_term::term::ritchie::{
    EthRitchie, EthRitchieRegularParameter, EtherealRitchieParameter,
};
use husky_fly_term::FlyRitchieRegularParameter;
use husky_term_prelude::{TermContract, TypeRitchieKind};

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar, constructor = new)]
pub struct HirRitchieType {
    pub ritchie_ty_kind: TypeRitchieKind,
    #[return_ref]
    pub parameters: HirRitchieParameters,
    pub return_ty: HirType,
}

impl HirRitchieType {
    pub fn from_eth(term: EthRitchie, db: &::salsa::Db) -> Self {
        hir_ty_from_eth_term_ritchie(db, term)
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_ty_from_eth_term_ritchie(db: &::salsa::Db, term_ritchie: EthRitchie) -> HirRitchieType {
    HirRitchieType::new(
        db,
        term_ritchie
            .ritchie_kind(db)
            .ritchie_ty_kind()
            .expect("should be type"),
        HirRitchieParameters::from_eth(term_ritchie.parameter_contracted_tys(db), db),
        HirType::from_eth(term_ritchie.return_ty(db), db).unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db]
pub struct HirRitchieParameters {
    data: SmallVec<[HirRitchieParameter; 4]>,
}

impl HirRitchieParameters {
    pub(crate) fn from_eth(params: &[EtherealRitchieParameter], db: &::salsa::Db) -> Self {
        HirRitchieParameters {
            data: params
                .iter()
                .copied()
                .map(|param| HirRitchieParameter::from_eth(param, db))
                .collect(),
        }
    }
}

impl std::ops::Deref for HirRitchieParameters {
    type Target = [HirRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db]
pub enum HirRitchieParameter {
    Ordinary(HirRitchieRegularParameter),
    Variadic(HirRitchieVariadicParameter),
    Keyed(HirRitchieKeyedParameter),
}

impl HirRitchieParameter {
    pub fn from_eth(param: EtherealRitchieParameter, db: &::salsa::Db) -> Self {
        match param {
            EtherealRitchieParameter::Regular(param) => Self::from_eth_regular(param, db),
            EtherealRitchieParameter::Variadic(_) => todo!(),
            EtherealRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub fn from_eth_regular(param: EthRitchieRegularParameter, db: &::salsa::Db) -> Self {
        HirRitchieRegularParameter {
            contract: HirEagerContract::from_term(param.contract()),
            ty: HirType::from_eth(param.ty(), db).unwrap(),
        }
        .into()
    }
}

#[salsa::debug_with_db]
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirRitchieRegularParameter {
    pub contract: HirEagerContract,
    pub ty: HirType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirEagerContract {
    Pure,
    Move,
    Borrow,
    BorrowMut,
    Const,
    Leash,
    At,
}

impl HirEagerContract {
    pub fn from_term(contract: TermContract) -> Self {
        match contract {
            TermContract::Pure => HirEagerContract::Pure,
            TermContract::Move => HirEagerContract::Move,
            TermContract::Borrow => HirEagerContract::Borrow,
            TermContract::BorrowMut => HirEagerContract::BorrowMut,
            TermContract::Const => HirEagerContract::Const,
            TermContract::Leash => HirEagerContract::Leash,
            TermContract::At => HirEagerContract::At,
        }
    }
}

impl HirRitchieRegularParameter {
    pub fn contract(&self) -> HirEagerContract {
        self.contract
    }

    pub fn ty(&self) -> HirType {
        self.ty
    }

    pub fn from_fly(
        param: &FlyRitchieRegularParameter,
        db: &::salsa::Db,
        fluffy_terms: &FlyTerms,
    ) -> Self {
        Self {
            contract: HirEagerContract::from_term(param.contract),
            ty: HirType::from_fly(param.ty, db, fluffy_terms).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirRitchieVariadicParameter {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirRitchieKeyedParameter {}
