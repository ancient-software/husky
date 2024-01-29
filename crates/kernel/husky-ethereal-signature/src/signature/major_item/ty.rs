mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod record;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use crate::*;
use husky_coword::Ident;
use husky_declarative_signature::{
    HasDeclarativeSignatureTemplate, TypeDeclarativeSignatureTemplate,
};
use husky_entity_path::TypePath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[salsa::debug_with_db]
pub enum TypeEtherealSignatureTemplate {
    Enum(EnumTypeEtherealSignatureTemplate),
    PropsStruct(PropsStructTypeEtherealSignatureTemplate),
    UnitStruct(UnitStructTypeEtherealSignatureTemplate),
    TupleStruct(TupleStructTypeEtherealSignatureTemplate),
    Record(RecordTypeEtherealSignatureTemplate),
    Inductive(InductiveTypeEtherealSignatureTemplate),
    Structure(StructureTypeEtherealSignatureTemplate),
    Extern(ExternTypeEtherealSignatureTemplate),
    Union(UnionTypeEtherealSignatureTemplate),
}

impl TypeEtherealSignatureTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EtherealTemplateParameter] {
        match self {
            TypeEtherealSignatureTemplate::Enum(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::PropsStruct(template) => {
                template.template_parameters(db)
            }
            TypeEtherealSignatureTemplate::UnitStruct(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::TupleStruct(template) => {
                template.template_parameters(db)
            }
            TypeEtherealSignatureTemplate::Record(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::Inductive(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::Structure(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::Extern(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::Union(template) => template.template_parameters(db),
        }
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> Option<EtherealTerm> {
        match self {
            TypeEtherealSignatureTemplate::Enum(_) => None,
            TypeEtherealSignatureTemplate::PropsStruct(slf) => {
                Some(slf.instance_constructor_ty(db).into())
            }
            TypeEtherealSignatureTemplate::UnitStruct(slf) => {
                Some(slf.instance_constructor_ty(db).into())
            }
            TypeEtherealSignatureTemplate::TupleStruct(slf) => {
                Some(slf.instance_constructor_ty(db).into())
            }
            TypeEtherealSignatureTemplate::Record(_) => todo!(),
            TypeEtherealSignatureTemplate::Inductive(_) => todo!(),
            TypeEtherealSignatureTemplate::Structure(_) => todo!(),
            TypeEtherealSignatureTemplate::Extern(_) => todo!(),
            TypeEtherealSignatureTemplate::Union(_) => todo!(),
        }
    }
}

impl HasEtherealSignatureTemplate for TypePath {
    type EtherealSignatureTemplate = TypeEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        ty_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn ty_ethereal_signature_template(
    db: &::salsa::Db,
    path: TypePath,
) -> EtherealSignatureResult<TypeEtherealSignatureTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TypeDeclarativeSignatureTemplate::Enum(declarative_signature_template) => {
            EnumTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::PropsStruct(declarative_signature_template) => {
            PropsStructTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::UnitStruct(declarative_signature_template) => {
            UnitStructTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::TupleStruct(declarative_signature_template) => {
            TupleStructTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Record(declarative_signature_template) => {
            RecordTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Inductive(declarative_signature_template) => {
            InductiveTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Structure(declarative_signature_template) => {
            StructureTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Extern(declarative_signature_template) => {
            ExternTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Union(declarative_signature_template) => {
            UnionTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum PropsFieldEtherealSignature {
    PropsStruct(PropsStructFieldEtherealSignature),
}

pub trait HasPropsFieldEtherealSignature: Copy {
    fn regular_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature>;
}

impl HasPropsFieldEtherealSignature for TypePath {
    fn regular_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature> {
        self.ethereal_signature_template(db)?
            .regular_field_ethereal_signature(db, arguments, ident)
    }
}

impl HasPropsFieldEtherealSignature for TypeEtherealSignatureTemplate {
    fn regular_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature> {
        match self {
            TypeEtherealSignatureTemplate::Enum(_ethereal_signature_template) => Nothing,
            TypeEtherealSignatureTemplate::PropsStruct(ethereal_signature_template) => {
                ethereal_signature_template.regular_field_ethereal_signature(db, arguments, ident)
            }
            TypeEtherealSignatureTemplate::UnitStruct(_) => todo!(),
            TypeEtherealSignatureTemplate::TupleStruct(_) => todo!(),
            TypeEtherealSignatureTemplate::Record(_) => todo!(),
            TypeEtherealSignatureTemplate::Inductive(_) => todo!(),
            TypeEtherealSignatureTemplate::Structure(_) => todo!(),
            TypeEtherealSignatureTemplate::Extern(_) => Nothing,
            TypeEtherealSignatureTemplate::Union(_) => todo!(),
        }
    }
}

impl PropsFieldEtherealSignature {
    pub fn ty(self) -> EtherealTerm {
        match self {
            PropsFieldEtherealSignature::PropsStruct(signature) => signature.ty(),
        }
    }
}
