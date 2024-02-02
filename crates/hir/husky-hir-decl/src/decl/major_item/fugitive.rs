mod r#fn;
mod gn;
mod ty_alias;
mod val;

use husky_syn_decl::FugitiveSynDecl;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ty_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum FugitiveHirDecl {
    FunctionFn(FunctionMajorFnHirDecl),
    Val(ValFugitiveHirDecl),
    FunctionGn(FunctionGnFugitiveHirDecl),
    TypeAlias(TypeAliasHirDecl),
}

impl FugitiveHirDecl {
    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> Option<&'a HirTemplateParameters> {
        match self {
            FugitiveHirDecl::FunctionFn(decl) => Some(decl.template_parameters(db)),
            FugitiveHirDecl::Val(_decl) => None,
            FugitiveHirDecl::FunctionGn(decl) => Some(decl.template_parameters(db)),
            FugitiveHirDecl::TypeAlias(_) => todo!(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            FugitiveHirDecl::FunctionFn(decl) => decl.hir_eager_expr_region(db).into(),
            FugitiveHirDecl::Val(decl) => decl.hir_eager_expr_region(db).into(),
            FugitiveHirDecl::FunctionGn(decl) => decl.hir_lazy_expr_region(db).into(),
            FugitiveHirDecl::TypeAlias(decl) => decl.hir_eager_expr_region(db).into(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> FugitivePath {
        match self {
            FugitiveHirDecl::FunctionFn(decl) => decl.path(db),
            FugitiveHirDecl::Val(decl) => decl.path(db),
            FugitiveHirDecl::FunctionGn(decl) => decl.path(db),
            FugitiveHirDecl::TypeAlias(decl) => decl.path(db),
        }
    }
}

impl HasHirDecl for FugitivePath {
    type HirDecl = FugitiveHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        fugitive_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn fugitive_hir_decl(db: &::salsa::Db, path: FugitivePath) -> Option<FugitiveHirDecl> {
    match path.syn_decl(db).expect("no errors for hir stage") {
        FugitiveSynDecl::FunctionFn(syn_decl) => {
            Some(FunctionMajorFnHirDecl::from_syn(path, syn_decl, db).into())
        }
        FugitiveSynDecl::Val(syn_decl) => {
            Some(ValFugitiveHirDecl::from_syn(path, syn_decl, db).into())
        }
        FugitiveSynDecl::FunctionGn(syn_decl) => {
            Some(FunctionGnFugitiveHirDecl::from_syn(path, syn_decl, db).into())
        }
    }
}
