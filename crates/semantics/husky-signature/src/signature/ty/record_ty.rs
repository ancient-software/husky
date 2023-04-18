use super::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn record_ty_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: RecordTypeDecl,
) -> DeclarativeSignatureResult<RecordTypeDeclarativeSignature> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(RecordTypeDeclarativeSignature::new(db, implicit_parameters))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct RecordTypeDeclarativeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl RecordTypeDeclarativeSignature {}
