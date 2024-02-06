use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssocFnEthTemplate {
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
}
