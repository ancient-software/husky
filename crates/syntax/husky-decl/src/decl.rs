mod form;
mod trai;
mod trai_item;
mod ty;
mod ty_item;
mod variant;

use crate::*;

pub use form::*;
pub use trai::*;
pub use trai_item::*;
pub use ty::*;
pub use ty_item::*;
pub use variant::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Decl {
    Type(TypeDecl),
    Form(FormDecl),
    Trait(TraitDecl),
    TypeItem(TypeItemDecl),
    TraitItem(TraitItemDecl),
    Variant(VariantDecl),
}

impl Decl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            Decl::Type(decl) => decl.ast_idx(db),
            Decl::Form(decl) => decl.ast_idx(db),
            Decl::Trait(decl) => decl.ast_idx(db),
            Decl::TypeItem(decl) => decl.ast_idx(db),
            Decl::TraitItem(decl) => decl.ast_idx(db),
            Decl::Variant(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        match self {
            Decl::Type(decl) => decl.implicit_parameters(db),
            Decl::Form(decl) => decl.implicit_parameters(db),
            Decl::Trait(decl) => decl.implicit_parameters(db),
            Decl::TypeItem(decl) => decl.implicit_parameters(db),
            Decl::TraitItem(decl) => decl.implicit_parameters(db),
            Decl::Variant(decl) => &[],
        }
    }

    pub fn expr_sheet(self, db: &dyn DeclDb) -> ExprSheet {
        match self {
            Decl::Type(decl) => decl.expr_sheet(db).into(),
            Decl::Form(decl) => decl.expr_sheet(db).into(),
            Decl::Trait(decl) => decl.expr_sheet(db).into(),
            Decl::TypeItem(decl) => decl.expr_sheet(db).into(),
            Decl::TraitItem(decl) => decl.expr_sheet(db).into(),
            Decl::Variant(decl) => todo!(),
        }
    }
}

impl From<TraitDecl> for Decl {
    fn from(v: TraitDecl) -> Self {
        Self::Trait(v)
    }
}

impl From<FormDecl> for Decl {
    fn from(v: FormDecl) -> Self {
        Self::Form(v)
    }
}

impl From<TypeDecl> for Decl {
    fn from(v: TypeDecl) -> Self {
        Self::Type(v)
    }
}

impl From<TraitItemDecl> for Decl {
    fn from(v: TraitItemDecl) -> Self {
        Self::TraitItem(v)
    }
}

impl From<TypeItemDecl> for Decl {
    fn from(v: TypeItemDecl) -> Self {
        Self::TypeItem(v)
    }
}

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for Decl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        match self {
            Decl::Type(decl) => f
                .debug_tuple("Type")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::Trait(decl) => f
                .debug_tuple("Trait")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::Form(decl) => f
                .debug_tuple("Form")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::Variant(decl) => f
                .debug_tuple("Variant")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::TypeItem(decl) => f
                .debug_tuple("TypeItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::TraitItem(decl) => f
                .debug_tuple("TraitItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
