use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_associated_ty_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitAssociatedTypeDecl,
) -> TypeAsTraitAssociatedTypeSignature {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    TypeAsTraitAssociatedTypeSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedTypeSignature {}
