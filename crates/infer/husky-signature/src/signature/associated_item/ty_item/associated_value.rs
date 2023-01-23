use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn ty_associated_value_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedValueDecl,
) -> SignatureOutcome<TypeAssociatedValueSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Success(TypeAssociatedValueSignature::new(db))
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAssociatedValueSignature {}
