mod r#fn;
mod gn;
mod ti;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ti::*;
pub use self::val::*;

use super::*;
use husky_entity_kind::FugitiveKind;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum FugitiveSynNodeDecl {
    FunctionFn(MajorFnSynNodeDecl),
    Val(MajorValSynNodeDecl),
    FunctionGn(MajorGnSynNodeDecl),
}

impl FugitiveSynNodeDecl {
    pub fn syn_node_path(self, db: &::salsa::Db) -> FugitiveSynNodePath {
        match self {
            FugitiveSynNodeDecl::FunctionFn(decl) => decl.syn_node_path(db),
            FugitiveSynNodeDecl::Val(decl) => decl.syn_node_path(db),
            FugitiveSynNodeDecl::FunctionGn(decl) => decl.syn_node_path(db),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            FugitiveSynNodeDecl::FunctionFn(decl) => decl.syn_expr_region(db),
            FugitiveSynNodeDecl::Val(decl) => decl.syn_expr_region(db),
            FugitiveSynNodeDecl::FunctionGn(decl) => decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            FugitiveSynNodeDecl::FunctionFn(syn_node_decl) => syn_node_decl.errors(db),
            FugitiveSynNodeDecl::Val(syn_node_decl) => syn_node_decl.errors(db),
            FugitiveSynNodeDecl::FunctionGn(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasSynNodeDecl for FugitiveSynNodePath {
    type NodeDecl = FugitiveSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        fugitive_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn fugitive_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: FugitiveSynNodePath,
) -> FugitiveSynNodeDecl {
    DeclParser::new(db, syn_node_path.into()).parse_fugitive_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_fugitive_syn_node_decl(
        &self,
        syn_node_path: FugitiveSynNodePath,
    ) -> FugitiveSynNodeDecl {
        match syn_node_path.fugitive_kind(self.db()) {
            FugitiveKind::Val => self.parse_val_node_decl(syn_node_path).into(),
            FugitiveKind::FunctionFn => self.parse_fn_node_decl(syn_node_path).into(),
            FugitiveKind::TypeAlias => {
                todo!()
            }
            FugitiveKind::FunctionGn => self.parse_gn_node_decl(syn_node_path).into(),
            FugitiveKind::Formal => todo!(),
            FugitiveKind::Const => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum FugitiveSynDecl {
    FunctionFn(FunctionMajorFnSynDecl),
    Val(MajorValSynDecl),
    FunctionGn(MajorGnSynDecl),
    // todo: TypeAlias
}

impl FugitiveSynDecl {
    fn from_node_decl(
        db: &::salsa::Db,
        path: MajorFugitivePath,
        syn_node_decl: FugitiveSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            FugitiveSynNodeDecl::FunctionFn(syn_node_decl) => {
                FunctionMajorFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FugitiveSynNodeDecl::Val(syn_node_decl) => {
                MajorValSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FugitiveSynNodeDecl::FunctionGn(syn_node_decl) => {
                MajorGnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            FugitiveSynDecl::FunctionFn(decl) => decl.template_parameters(db),
            FugitiveSynDecl::Val(_decl) => &[],
            FugitiveSynDecl::FunctionGn(decl) => decl.template_parameters(db),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            FugitiveSynDecl::FunctionFn(decl) => decl.syn_expr_region(db),
            FugitiveSynDecl::Val(decl) => decl.syn_expr_region(db),
            FugitiveSynDecl::FunctionGn(decl) => decl.syn_expr_region(db),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> MajorFugitivePath {
        match self {
            FugitiveSynDecl::FunctionFn(decl) => decl.path(db),
            FugitiveSynDecl::Val(decl) => decl.path(db),
            FugitiveSynDecl::FunctionGn(decl) => decl.path(db),
        }
    }
}

impl HasSynDecl for MajorFugitivePath {
    type Decl = FugitiveSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        fugitive_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn fugitive_syn_decl(
    db: &::salsa::Db,
    path: MajorFugitivePath,
) -> DeclResult<FugitiveSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    FugitiveSynDecl::from_node_decl(db, path, syn_node_decl)
}
