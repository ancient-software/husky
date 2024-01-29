mod enum_props_ty_variant;
mod enum_tuple_ty_variant;
mod enum_unit_ty_variant;

pub use self::enum_props_ty_variant::*;
pub use self::enum_tuple_ty_variant::*;
pub use self::enum_unit_ty_variant::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TypeVariantEtherealSignatureTemplate {
    Props(EnumPropsVariantEtherealSignatureTemplate),
    Unit(EnumUnitTypeVariantEtherealSignatureTemplate),
    Tuple(EnumTupleVariantEtherealSignatureTemplate),
}

impl TypeVariantEtherealSignatureTemplate {
    pub fn self_ty(self, _db: &::salsa::Db) -> EtherealTerm {
        match self {
            TypeVariantEtherealSignatureTemplate::Props(_) => todo!(),
            TypeVariantEtherealSignatureTemplate::Unit(_) => todo!(),
            TypeVariantEtherealSignatureTemplate::Tuple(_) => todo!(),
        }
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EtherealTerm {
        match self {
            TypeVariantEtherealSignatureTemplate::Props(slf) => slf.instance_constructor_ty(db),
            TypeVariantEtherealSignatureTemplate::Unit(slf) => slf.instance_constructor_ty(db),
            TypeVariantEtherealSignatureTemplate::Tuple(slf) => slf.instance_constructor_ty(db),
        }
    }
}

impl HasEtherealSignatureTemplate for TypeVariantPath {
    type EtherealSignatureTemplate = TypeVariantEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        ty_variant_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn ty_variant_ethereal_signature_template(
    db: &::salsa::Db,
    path: TypeVariantPath,
) -> EtherealSignatureResult<TypeVariantEtherealSignatureTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TypeVariantDeclarativeSignatureTemplate::Props(_) => todo!(),
        TypeVariantDeclarativeSignatureTemplate::Unit(declarative_signature_template) => {
            EnumUnitTypeVariantEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeVariantDeclarativeSignatureTemplate::Tuple(declarative_signature_template) => {
            EnumTupleVariantEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
    })
}
