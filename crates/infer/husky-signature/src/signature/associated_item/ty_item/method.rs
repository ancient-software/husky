use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_method_signature(
    db: &dyn SignatureDb,
    decl: TypeMethodDecl,
) -> TypeMethodSignature {
    let impl_block = decl.associated_item(db).impl_block(db);
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();

    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        signature_term_region,
        term_menu,
    );

    let parameters = ParameterSignatures::from_decl(decl.parameters(db), signature_term_region);
    let output_ty = match decl.output_ty(db) {
        Ok(output_ty) => match signature_term_region.expr_term(*output_ty) {
            Success(output_ty) => output_ty,
            Failure(_) => todo!(),
            Abort(_) => todo!(),
        },
        Err(_) => todo!(), // Abort(SignatureTermAbortion::ExprError),
    };
    // TypeMethodSignature::new(
    //     db,
    //     implicit_parameters,
    //     parameters,
    //     output_ty,
    //     engine.finish(),
    // )
    todo!()
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: ParameterSignatures,
    pub output_ty: Term,
}
