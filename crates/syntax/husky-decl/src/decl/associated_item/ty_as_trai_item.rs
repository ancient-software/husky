mod assoc_ty;
mod assoc_val;
mod function;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use method::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub enum TypeAsTraitItemDecl {
    AssociatedFunction(TypeAsTraitAssociatedFunctionDecl),
    Method(TypeAsTraitMethodDecl),
    AssociatedType(TypeAsTraitAssociatedTypeDecl),
    AssociatedValue(TypeAsTraitAssociatedValueDecl),
}

impl From<TypeAsTraitAssociatedValueDecl> for TypeAsTraitItemDecl {
    fn from(v: TypeAsTraitAssociatedValueDecl) -> Self {
        Self::AssociatedValue(v)
    }
}

impl From<TypeAsTraitAssociatedTypeDecl> for TypeAsTraitItemDecl {
    fn from(v: TypeAsTraitAssociatedTypeDecl) -> Self {
        Self::AssociatedType(v)
    }
}

impl From<TypeAsTraitMethodDecl> for TypeAsTraitItemDecl {
    fn from(v: TypeAsTraitMethodDecl) -> Self {
        Self::Method(v)
    }
}

impl From<TypeAsTraitAssociatedFunctionDecl> for TypeAsTraitItemDecl {
    fn from(v: TypeAsTraitAssociatedFunctionDecl) -> Self {
        Self::AssociatedFunction(v)
    }
}

impl TypeAsTraitItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TypeAsTraitItemDecl::AssociatedFunction(decl) => decl.ast_idx(db),
            TypeAsTraitItemDecl::Method(decl) => decl.ast_idx(db),
            TypeAsTraitItemDecl::AssociatedType(decl) => decl.ast_idx(db),
            TypeAsTraitItemDecl::AssociatedValue(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self {
            TypeAsTraitItemDecl::AssociatedFunction(_) => todo!(),
            TypeAsTraitItemDecl::Method(_) => todo!(),
            TypeAsTraitItemDecl::AssociatedType(_) => todo!(),
            TypeAsTraitItemDecl::AssociatedValue(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TypeAsTraitItemDecl::AssociatedFunction(decl) => decl.expr_region(db),
            TypeAsTraitItemDecl::Method(decl) => decl.expr_region(db),
            TypeAsTraitItemDecl::AssociatedType(decl) => decl.expr_region(db),
            TypeAsTraitItemDecl::AssociatedValue(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<TypeAsTraitItemPath> {
        match self {
            TypeAsTraitItemDecl::AssociatedFunction(_) => todo!(),
            TypeAsTraitItemDecl::Method(decl) => decl.path(db),
            TypeAsTraitItemDecl::AssociatedType(_) => todo!(),
            TypeAsTraitItemDecl::AssociatedValue(_) => todo!(),
        }
    }
}
