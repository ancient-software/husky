mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;
use husky_entity_path::ModuleItemPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum ModuleItemNodePath {
    Trait(TraitNodePath),
    Type(TypeNodePath),
    Fugitive(FugitiveNodePath),
}

impl ModuleItemNodePath {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: ModuleItemPath,
    ) -> Self {
        match path {
            ModuleItemPath::Type(path) => TypeNodePath::new(db, registry, path).into(),
            ModuleItemPath::Trait(path) => TraitNodePath::new(db, registry, path).into(),
            ModuleItemPath::Fugitive(path) => FugitiveNodePath::new(db, registry, path).into(),
        }
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> Option<ModuleItemPath> {
        match self {
            ModuleItemNodePath::Trait(node_path) => node_path
                .maybe_ambiguous_path(db)
                .unambiguous_path()
                .map(Into::into),
            ModuleItemNodePath::Type(node_path) => node_path
                .maybe_ambiguous_path(db)
                .unambiguous_path()
                .map(Into::into),
            ModuleItemNodePath::Fugitive(node_path) => node_path
                .maybe_ambiguous_path(db)
                .unambiguous_path()
                .map(Into::into),
        }
    }

    pub fn ident(self, db: &dyn EntityTreeDb) -> Ident {
        todo!("")
        // self.path(db).ident(db)
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            ModuleItemNodePath::Trait(node) => node.module_path(db),
            ModuleItemNodePath::Type(node) => node.module_path(db),
            ModuleItemNodePath::Fugitive(node) => node.module_path(db),
        }
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> ModuleItemNode {
        todo!()
    }
}

impl HasNodePath for ModuleItemPath {
    type NodePath = ModuleItemNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        match self {
            ModuleItemPath::Type(path) => path.node_path(db).into(),
            ModuleItemPath::Trait(path) => path.node_path(db).into(),
            ModuleItemPath::Fugitive(path) => path.node_path(db).into(),
        }
    }
}

// todo: change this to enum and create FugitiveNode etc.
#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct ModuleItemNode {
    #[id]
    pub node_path: ModuleItemNodePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}

impl ModuleItemNode {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        module_item_path: ModuleItemPath,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
    ) -> Self {
        ModuleItemNode::new_inner(
            db,
            ModuleItemNodePath::new(db, registry, module_item_path),
            visibility,
            ast_idx,
            ident_token,
        )
    }

    /// only gives a path when valid
    pub fn unambiguous_path(self, db: &dyn EntityTreeDb) -> Option<ModuleItemPath> {
        self.node_path(db).path(db)
    }
}
