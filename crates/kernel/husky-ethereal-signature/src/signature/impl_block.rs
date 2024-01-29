mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum ImplBlockEtherealSignatureTemplate {
    TypeImpl(TypeImplBlockEtherealSignatureTemplate),
    TraitForTypeImpl(TraitForTypeImplBlockEtherealSignatureTemplate),
}

impl ImplBlockEtherealSignatureTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> EtherealTerm {
        match self {
            ImplBlockEtherealSignatureTemplate::TypeImpl(template) => template.self_ty(db),
            ImplBlockEtherealSignatureTemplate::TraitForTypeImpl(template) => template.self_ty(db),
        }
    }
}

impl HasEtherealSignatureTemplate for ImplBlockPath {
    type EtherealSignatureTemplate = ImplBlockEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        Ok(match self {
            ImplBlockPath::TypeImplBlock(path) => path.ethereal_signature_template(db)?.into(),
            ImplBlockPath::TraitForTypeImplBlock(path) => {
                path.ethereal_signature_template(db)?.into()
            }
        })
    }
}
