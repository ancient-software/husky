use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeImplBlockEthTemplate {
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    pub self_ty: EtherealTerm,
}

impl HasEthTemplate for TypeImplBlockPath {
    type EthTemplate = TypeImplBlockEthTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        ty_impl_block_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_impl_block_ethereal_signature_template(
    db: &::salsa::Db,
    path: TypeImplBlockPath,
) -> EtherealSignatureResult<TypeImplBlockEthTemplate> {
    let declarative_signature_template = path.declarative_signature_template(db)?;
    let template_parameters = EtherealTemplateParameters::from_declarative(
        db,
        declarative_signature_template.template_parameters(db),
    )?;
    let ty = EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty(db))?;
    Ok(TypeImplBlockEthTemplate::new(db, template_parameters, ty))
}
