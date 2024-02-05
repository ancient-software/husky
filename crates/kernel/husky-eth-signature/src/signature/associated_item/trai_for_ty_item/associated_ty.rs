use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedTypeEthTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub associated_ty: EthTerm,
}

impl TraitForTypeAssociatedTypeEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        dec_template: TraitForTypeAssociatedTypeDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        let ty_term = EthTerm::ty_from_dec(db, dec_template.ty_term(db))?;
        Ok(Self::new(db, path, template_parameters, ty_term))
    }

    pub(super) fn inherit_instantiation_builder(
        self,
        db: &::salsa::Db,
        impl_block_signature_builder: TraitForTypeImplBlockEtherealSignatureBuilder,
    ) -> TraitForTypeAssociatedTypeEtherealSignatureBuilder {
        let instantiation_builder = impl_block_signature_builder
            .instantiation_builder(db)
            .merge_with_item_template_parameters(self.template_parameters(db));
        TraitForTypeAssociatedTypeEtherealSignatureBuilder::new(db, self, instantiation_builder)
    }
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = pub(super) new)]
pub struct TraitForTypeAssociatedTypeEtherealSignatureBuilder {
    pub template: TraitForTypeAssociatedTypeEthTemplate,
    #[return_ref]
    pub instantiation_builder: EtherealInstantiationBuilder,
}

impl TraitForTypeAssociatedTypeEtherealSignatureBuilder {
    pub fn try_into_signature(
        self,
        db: &::salsa::Db,
    ) -> Option<TraitForTypeAssociatedTypeEtherealSignature> {
        trai_for_ty_associated_ty_ethereal_signature_signature_builder_try_into_signature(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_associated_ty_ethereal_signature_signature_builder_try_into_signature(
    db: &::salsa::Db,
    signature_builder: TraitForTypeAssociatedTypeEtherealSignatureBuilder,
) -> Option<TraitForTypeAssociatedTypeEtherealSignature> {
    let instantiation = signature_builder
        .instantiation_builder(db)
        .try_into_instantiation()?;
    let template = signature_builder.template(db);
    Some(TraitForTypeAssociatedTypeEtherealSignature {
        path: template.path(db),
        ty_term: template.associated_ty(db).instantiate(db, &instantiation),
        instantiation,
    })
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitForTypeAssociatedTypeEtherealSignature {
    path: TraitForTypeItemPath,
    instantiation: EthInstantiation,
    ty_term: EthTerm,
}

impl TraitForTypeAssociatedTypeEtherealSignature {
    pub fn path(&self) -> TraitForTypeItemPath {
        self.path
    }

    pub fn ty_term(&self) -> EthTerm {
        self.ty_term
    }

    pub fn instantiation(&self) -> &EthInstantiation {
        &self.instantiation
    }
}
