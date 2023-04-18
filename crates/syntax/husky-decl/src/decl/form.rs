mod r#fn;
mod gn;
mod type_alias;
mod value;
mod var;

pub use gn::*;
use husky_entity_taxonomy::{EntityKind, FormKind};
pub use r#fn::*;
pub use var::*;

pub use type_alias::*;
pub use value::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum FormDecl {
    Fn(FnDecl),
    Val(ValDecl),
    Gn(GnDecl),
}

impl FormDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            FormDecl::Fn(decl) => decl.ast_idx(db),
            FormDecl::Val(decl) => decl.ast_idx(db),
            FormDecl::Gn(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        match self {
            FormDecl::Fn(decl) => decl.implicit_parameters(db),
            FormDecl::Val(_decl) => &[],
            FormDecl::Gn(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            FormDecl::Fn(decl) => decl.expr_region(db),
            FormDecl::Val(decl) => decl.expr_region(db),
            FormDecl::Gn(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> FormPath {
        match self {
            FormDecl::Fn(decl) => decl.path(db),
            FormDecl::Val(decl) => decl.path(db),
            FormDecl::Gn(decl) => decl.path(db),
        }
    }
}

impl HasDecl for FormPath {
    type Decl = FormDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        form_decl(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn form_decl(db: &dyn DeclDb, path: FormPath) -> DeclResult<FormDecl> {
    let parser = DeclParseContext::new(db, path.module_path(db))?;
    parser.parse_form_decl(path)
}

impl<'a> DeclParseContext<'a> {
    fn parse_form_decl(&self, path: FormPath) -> DeclResult<FormDecl> {
        let ast_idx: AstIdx = self.resolve_module_item_ast_idx(path);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                entity_kind,
                saved_stream_state,
                ..
            } => self.parse_form_decl_aux(
                ast_idx,
                path,
                entity_kind,
                token_group_idx,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_form_decl_aux(
        &self,
        ast_idx: AstIdx,
        path: FormPath,
        _entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
    ) -> Result<FormDecl, DeclError> {
        match path.form_kind(self.db()) {
            FormKind::Val => {
                self.parse_feature_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FormKind::Fn => self.parse_fn_decl(ast_idx, token_group_idx, saved_stream_state, path),
            FormKind::Value => todo!(),
            FormKind::TypeAlias => {
                todo!()
            }
            FormKind::Gn => self.parse_gn_decl(ast_idx, token_group_idx, saved_stream_state, path),
        }
    }
}
