use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyRitchieRegularParameter {
    pub contract: TermContract,
    pub ty: FluffyTerm,
}

impl FluffyRitchieRegularParameter {
    pub(super) fn resolve_as_ethereal(
        self,
        terms: &impl std::borrow::Borrow<HollowTerms>,
    ) -> Option<EtherealRitchieRegularParameter> {
        Some(EtherealRitchieRegularParameter::new(
            self.contract,
            self.ty.resolve_as_ethereal(terms)?,
        ))
    }
}

impl From<EtherealRitchieRegularParameter> for FluffyRitchieRegularParameter {
    fn from(param: EtherealRitchieRegularParameter) -> Self {
        Self {
            contract: param.contract(),
            ty: param.ty().into(),
        }
    }
}

impl FluffyInstantiate for EtherealRitchieRegularParameter {
    type Target = FluffyRitchieRegularParameter;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &FluffyInstantiation,
    ) -> Self::Target {
        FluffyRitchieRegularParameter {
            contract: self.contract(),
            ty: self.ty().instantiate(engine, expr_idx, instantiation),
        }
    }
}

impl FluffyRitchieRegularParameter {
    pub fn new(contract: TermContract, ty: FluffyTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> TermContract {
        self.contract
    }

    pub fn ty(&self) -> FluffyTerm {
        self.ty
    }

    pub fn ty_mut(&mut self) -> &mut FluffyTerm {
        &mut self.ty
    }
}
