use super::*;
use husky_entity_kind::TypeKind;
use husky_print_utils::p;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntitySynTreeJar)]
#[salsa::deref_id]
pub struct TypeSynNodePath(ItemSynNodePathId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeSynNodePathData {
    maybe_ambiguous_path: MaybeAmbiguousPath<TypePath>,
}

impl TypeSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: TypePath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Type(TypeSynNodePathData {
                maybe_ambiguous_path: registry.issue_maybe_ambiguous_path(path),
            })),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TypeSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Type(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ty_kind(self, db: &::salsa::Db) -> TypeKind {
        self.data(db).maybe_ambiguous_path.path.ty_kind(db)
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a MajorItemSynNode {
        let module_path: ModulePath = todo!(); //= syn_node_path.module_path(db);
                                               // it's important to use presheet instead of sheet
                                               // otherwise cyclic when use all type variant paths
        let item_presheet = db.item_syn_tree_presheet(module_path);
        let Some(ItemSynNode::MajorItem(node)) = item_presheet.major_item_node(self.into()) else {
            unreachable!("should be some, must be some erros in library")
        };
        node
    }

    pub(crate) fn attr_syn_nodes<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> &'a [(AttrSynNodePath, AttrSynNode)] {
        ty_attrs(db, self)
    }
}

impl TypeSynNodePathData {
    pub fn path(self, db: &::salsa::Db) -> Option<TypePath> {
        self.maybe_ambiguous_path.unambiguous_path()
    }

    pub fn ty_kind(self, db: &::salsa::Db) -> TypeKind {
        self.maybe_ambiguous_path.path.ty_kind(db)
    }
}

impl HasSynNodePath for TypePath {
    type SynNodePath = TypeSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TypeSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Type(TypeSynNodePathData {
                maybe_ambiguous_path: MaybeAmbiguousPath::from_path(self),
            })),
        ))
    }
}

impl From<TypeSynNodePath> for ItemSynNodePath {
    fn from(id: TypeSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}

impl HasAttrPaths for TypePath {
    type AttrPath = AttrItemPath;

    fn attr_paths(self, db: &::salsa::Db) -> &[Self::AttrPath] {
        ty_attr_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn ty_attrs(
    db: &::salsa::Db,
    ty_syn_node_path: TypeSynNodePath,
) -> SmallVec<[(AttrSynNodePath, AttrSynNode); 2]> {
    let ast_sheet: AstSheet = todo!(); // = ty_syn_node_path.module_path(db).ast_sheet(db);
    let mut registry = ItemSynNodePathRegistry::default();
    ast_sheet.procure_attrs(
        ty_syn_node_path.data(db).maybe_ambiguous_path.path.into(),
        ty_syn_node_path.syn_node(db).ast_idx,
        move |attr_ast_idx, _, path| {
            AttrSynNode::new(
                ty_syn_node_path.into(),
                path,
                attr_ast_idx,
                &mut registry,
                db,
            )
        },
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn ty_attr_paths(db: &::salsa::Db, path: TypePath) -> SmallVec<[AttrItemPath; 2]> {
    path.syn_node_path(db)
        .attr_syn_nodes(db)
        .iter()
        .filter_map(|(attr_syn_node_path, _)| attr_syn_node_path.path(db))
        .collect()
}
