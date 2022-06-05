mod struct_item_context;

use entity_syntax::EntitySyntaxQueryGroup;
pub use struct_item_context::*;

use file::FilePtr;
use word::Paradigm;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AstContext {
    Package(FilePtr),
    Module(EntityRoutePtr),
    Stmt(Paradigm),
    Match(Paradigm),
    Visual,
    Struct {
        opt_base_ty: Option<EntityRoutePtr>,
        item_context: StructItemContext,
    },
    Record,
    Props,
    Enum(EntityRoutePtr),
}

impl AstContext {
    pub fn opt_subroute(
        self,
        db: &dyn EntitySyntaxQueryGroup,
        ident: CustomIdentifier,
    ) -> Option<EntityRoutePtr> {
        Some(match self {
            AstContext::Package(main) => db.make_subroute(db.module(main).unwrap(), ident, vec![]),
            AstContext::Module(route) => db.make_subroute(route, ident, Vec::new()),
            AstContext::Stmt(_) => todo!(),
            AstContext::Match(_) => todo!(),
            AstContext::Struct { opt_base_ty, .. } => db.make_subroute(opt_base_ty?, ident, vec![]),
            AstContext::Enum(_) => todo!(),
            AstContext::Record => todo!(),
            AstContext::Props => todo!(),
            AstContext::Visual => todo!(),
        })
    }
}

impl From<Paradigm> for AstContext {
    fn from(paradigm: Paradigm) -> Self {
        match paradigm {
            Paradigm::EagerProcedural => AstContext::Stmt(Paradigm::EagerProcedural),
            Paradigm::EagerFunctional => AstContext::Stmt(Paradigm::EagerFunctional),
            Paradigm::LazyFunctional => todo!(),
        }
    }
}

impl std::fmt::Display for AstContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AstContext::Package(_) => "package",
            AstContext::Module(_) => "module",
            AstContext::Stmt(Paradigm::LazyFunctional) => "def",
            AstContext::Stmt(Paradigm::EagerFunctional) => "func",
            AstContext::Stmt(Paradigm::EagerProcedural) => "proc",
            AstContext::Visual => "visual",
            AstContext::Struct { .. } => "struct",
            AstContext::Enum(_) => "enum",
            AstContext::Record => "record",
            AstContext::Props => "props",
            AstContext::Match(_) => todo!(),
        })
    }
}
