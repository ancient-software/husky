mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TraitItemDeclarativeSignatureTemplate {
    AssociatedFn(TraitAssociatedFnDeclarativeSignatureTemplate),
    MethodFn(TraitMethodFnDeclarativeSignatureTemplate),
    AssociatedType(TraitAssociatedTypeDeclarativeSignatureTemplate),
    AssociatedVal(TraitAssociatedValDeclarativeSignatureTemplate),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TraitItemDeclarativeSignatureTemplates {
    AssociatedFn(SmallVecImpl<TraitAssociatedFnDeclarativeSignatureTemplate>),
    MethodFn(SmallVecImpl<TraitMethodFnDeclarativeSignatureTemplate>),
    AssociatedType(SmallVecImpl<TraitAssociatedTypeDeclarativeSignatureTemplate>),
    AssociatedVal(SmallVecImpl<TraitAssociatedValDeclarativeSignatureTemplate>),
    // MemoizedField(SmallVecImpl<TraitMemoizedFieldDeclarativeSignatureTemplate>),
}

impl HasDeclarativeSignatureTemplate for TraitItemPath {
    type DeclarativeSignatureTemplate = TraitItemDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        trai_item_syn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_item_syn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: TraitItemPath,
) -> DeclarativeSignatureResult<TraitItemDeclarativeSignatureTemplate> {
    let decl = path.decl(db)?;
    match decl {
        TraitItemDecl::AssociatedFn(decl) => {
            trai_associated_form_fn_declarative_signature_template(db, decl).map(Into::into)
        }
        TraitItemDecl::MethodFn(decl) => {
            trai_method_fn_declarative_signature_template(db, decl).map(Into::into)
        }
        TraitItemDecl::AssociatedType(decl) => {
            trai_associated_ty_declarative_signature_template(db, decl).map(Into::into)
        }
        TraitItemDecl::AssociatedVal(decl) => {
            trai_associated_val_declarative_signature_template(db, decl).map(Into::into)
        }
    }
}

impl TraitItemDeclarativeSignatureTemplate {
    pub fn generic_parameters(
        self,
        _db: &dyn DeclarativeSignatureDb,
    ) -> &[DeclarativeGenericParameter] {
        match self {
            TraitItemDeclarativeSignatureTemplate::AssociatedFn(_) => todo!(),
            TraitItemDeclarativeSignatureTemplate::MethodFn(_) => todo!(),
            TraitItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
            TraitItemDeclarativeSignatureTemplate::AssociatedVal(_) => todo!(),
        }
    }
}
