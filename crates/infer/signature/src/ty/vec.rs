use vm::InputContract;

use super::*;

pub(crate) fn vec_signature_template(db: &dyn InferSignatureQueryGroup) -> Arc<TySignature> {
    let element_ty_ident = db.custom_ident("T");
    let element_ty = db.intern_scope(Scope {
        kind: ScopeKind::Generic {
            ident: element_ty_ident,
            raw_entity_kind: RawEntityKind::Type(RawTyKind::Other),
        },
        generics: vec![],
    });
    let mut members = IdentMap::default();
    members.insert_new(
        db.custom_ident("push"),
        MembSignature {
            kind: MembSignatureKind::Routine(MembCallSignature {
                this_contract: InputContract::BorrowMut,
                inputs: vec![InputSignature {
                    contract: InputContract::Move,
                    ty: element_ty,
                }],
                output: db.scope_menu().void_type,
                args: Default::default(),
            }),
        },
    );
    let mut generic_placeholders = IdentMap::default();
    generic_placeholders.insert_new(
        element_ty_ident,
        GenericPlaceholderKind::Type { traits: vec![] },
    );
    Arc::new(TySignature {
        generic_placeholders,
        members,
        kind: TySignatureKind::Vec { element_ty },
        traits: vec![db.scope_menu().clone_trait],
    })
}
