use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn ty_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TypeImplBlockDecl,
) -> SignatureResult<TypeImplBlockSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &signature_term_region,
        term_menu,
    );
    let ty = decl.ty(db);
    p!(
        expr_region.data(db).roots(),
        expr_region.data(db).path().debug(db)
    );
    let ty = match signature_term_region.expr_term(ty.expr()) {
        Ok(ty) => ty,
        Err(_) => todo!(),
    };
    Ok(TypeImplBlockSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(
            decl.implicit_parameters(db),
            signature_term_region,
            term_menu,
        ),
        ty,
    ))
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeImplBlockSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    pub ty: Term,
}
