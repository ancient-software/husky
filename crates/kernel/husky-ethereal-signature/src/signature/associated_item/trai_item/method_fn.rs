use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitMethodFnEtherealSignatureTemplate {
    pub path: TraitItemPath,
    pub template_parameters: EtherealTermTemplateParameters,
    /// `Self` as generic parameter
    pub self_ty_template_parameter: EtherealTermTemplateParameter,
}
