use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct DeriveDecrHirDecl {
    pub trai_term: EtherealTerm,
}
