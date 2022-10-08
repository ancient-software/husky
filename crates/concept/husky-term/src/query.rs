use std::sync::Arc;

use husky_entity_path::InternEntityPath;

use crate::*;

#[salsa::query_group(TermQueryStorage)]
pub trait TermQuery: InternTerm + AskDecl + InternEntityPath {
    fn term_menu(&self) -> Arc<TermMenu>;
    fn namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
    fn ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl>;
}

fn namespace_decl(db: &dyn TermQuery, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
    db.ask_namespace_decl(namespace)
}

fn ty_decl(db: &dyn TermQuery, ty: Ty) -> TermResultArc<TyDecl> {
    db.ask_ty_decl(ty)
}
