use crate::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TypeAssocValDecTemplate {
    pub path: TypeItemPath,
}

impl TypeAssocValDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeAssocValSynDecl,
    ) -> DecSignatureResult<TypeAssocValDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let _declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        Ok(TypeAssocValDecTemplate::new(db, path))
    }
}
