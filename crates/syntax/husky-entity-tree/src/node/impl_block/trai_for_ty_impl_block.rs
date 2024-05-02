use husky_entity_kind::{AssocItemKind, EntityKind};
use husky_entity_path::path::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockPath;
use vec_like::SmallVecPairMap;

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntityTreeJar)]
#[salsa::deref_id]
pub struct TraitForTypeImplBlockSynNodePath(ItemSynNodePathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntityTreeJar)]
pub struct TraitForTypeImplBlockSynNodePathData {
    pub(crate) path: TraitForTypeImplBlockPath,
}

impl TraitForTypeImplBlockSynNodePath {
    pub fn path(self, db: &::salsa::Db) -> TraitForTypeImplBlockPath {
        match self.0.unambiguous_item_path(db).expect("no ambiguity") {
            ItemPath::ImplBlock(ImplBlockPath::TraitForTypeImplBlock(path)) => path,
            _ => unreachable!(),
        }
    }

    pub fn trai_path(self, db: &::salsa::Db) -> TraitPath {
        self.path(db).trai_path(db)
    }

    pub fn ty_sketch(self, db: &::salsa::Db) -> TypeSketch {
        self.path(db).ty_sketch(db)
    }

    pub fn data(self, db: &::salsa::Db) -> TraitForTypeImplBlockSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TraitForTypeImplBlock(
                data,
            )) => data,
            _ => unreachable!(),
        }
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a TraitForTypeImplBlockSynNode {
        let module_path = self.module_path(db);
        let item_tree_sheet = db.item_syn_tree_sheet(module_path);
        item_tree_sheet.trai_for_ty_impl_block_syn_node(db, self)
    }

    pub(crate) fn assoc_items(
        self,
        db: &::salsa::Db,
    ) -> &[(Ident, TraitForTypeItemSynNodePath, TraitForTypeItemSynNode)] {
        trai_for_ty_impl_block_items(db, self)
    }

    pub fn item_syn_node_paths<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = TraitForTypeItemSynNodePath> + 'a {
        self.assoc_items(db)
            .iter()
            .map(|&(_, syn_node_path, _)| syn_node_path)
    }
}

impl TraitForTypeImplBlockSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> TraitForTypeImplBlockSynNodePath {
        TraitForTypeImplBlockSynNodePath(id)
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.path.module_path(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        TraitForTypeImplBlockSynNodePath(id).syn_node(db).ast_idx
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn trai_for_ty_impl_block_items(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> Vec<(Ident, TraitForTypeItemSynNodePath, TraitForTypeItemSynNode)> {
    syn_node_path.syn_node(db).trai_for_ty_impl_block_items(db)
}

impl From<TraitForTypeImplBlockSynNodePath> for ItemSynNodePath {
    fn from(id: TraitForTypeImplBlockSynNodePath) -> Self {
        ItemSynNodePath::ImplBlock(id.into())
    }
}

impl HasSynNodePath for TraitForTypeImplBlockPath {
    type SynNodePath = TraitForTypeImplBlockSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TraitForTypeImplBlockSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TraitForTypeImplBlock(
                TraitForTypeImplBlockSynNodePathData { path: self },
            )),
        ))
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct TraitForTypeImplBlockSynNode {
    syn_node_path: TraitForTypeImplBlockSynNodePath,
    pub(crate) ast_idx: AstIdx,
    pub(crate) impl_token: ImplToken,
    pub(crate) trai_expr: MajorItemPathExprIdx,
    pub(crate) for_token: TokenIdx,
    pub(crate) ty_sketch_expr: SelfTypeSketchExpr,
    pub(crate) items: Option<ImplBlockItems>,
}

impl TraitForTypeImplBlockSynNodePathData {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SelfTypeSketchExpr {
    Path(MajorItemPathExprIdx),
    DeriveAny {
        pound_token: PoundToken,
        derive_token: DeriveToken,
        underscore_token: UnderscoreToken,
    },
}

impl TraitForTypeImplBlockSynNode {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        impl_token: ImplToken,
        trai_expr: MajorItemPathExprIdx,
        trai_path: TraitPath,
        for_token: TokenIdx,
        ty_sketch_expr: SelfTypeSketchExpr,
        ty_sketch: TypeSketch,
        items: Option<ImplBlockItems>,
    ) -> Result<Self, ImplBlockIllForm> {
        // todo: check trai_path and ty_sketch
        // should check that if one of trai_path and ty_sketch must be always contained inside the crate
        Ok(TraitForTypeImplBlockSynNode {
            syn_node_path: TraitForTypeImplBlockSynNodePath(ItemSynNodePathId::new(
                db,
                ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePathData {
                        path: TraitForTypeImplBlockPath::new(
                            db,
                            registry,
                            module_path,
                            trai_path,
                            ty_sketch,
                        ),
                    },
                )),
            )),
            ast_idx,
            impl_token,
            trai_expr,
            for_token,
            ty_sketch_expr,
            items,
        })
    }
}

/// # getters
impl TraitForTypeImplBlockSynNode {
    pub fn syn_node_path(&self) -> TraitForTypeImplBlockSynNodePath {
        self.syn_node_path
    }

    pub fn module_path(&self, db: &::salsa::Db) -> ModulePath {
        self.syn_node_path.path(db).module_path(db)
    }

    pub(crate) fn trai_for_ty_impl_block_items(
        &self,
        db: &::salsa::Db,
    ) -> Vec<(Ident, TraitForTypeItemSynNodePath, TraitForTypeItemSynNode)> {
        let module_path = self.module_path(db);
        let ast_sheet = module_path.ast_sheet(db);
        let Some(items) = self.items else {
            return vec![];
        };
        let mut registry = ItemSynNodePathRegistry::default();
        items
            .ast_idx_range()
            .into_iter()
            .filter_map(|ast_idx| {
                let ast = &ast_sheet[ast_idx];
                match ast {
                    AstData::Identifiable {
                        visibility_expr,
                        item_kind,
                        ident_token,
                        is_generic,
                        ..
                    } => {
                        let item_kind = match item_kind {
                            EntityKind::AssocItem {
                                assoc_item_kind: AssocItemKind::TraitForTypeItem(ty_item_kind),
                            } => *ty_item_kind,
                            _ => unreachable!(),
                        };
                        let (syn_node_path, node) = TraitForTypeItemSynNode::new(
                            db,
                            &mut registry,
                            self.syn_node_path,
                            ast_idx,
                            ident_token.ident(),
                            item_kind,
                            visibility_expr.visibility(),
                            *is_generic,
                        );
                        Some((ident_token.ident(), syn_node_path, node))
                    }
                    AstData::Sorc { .. } | AstData::Attr { .. } | AstData::Err { .. } => None,
                    _ => unreachable!("Invariant"),
                }
            })
            .collect()
    }
}

impl HasAssocItemPaths for TraitForTypeImplBlockPath {
    type AssocItemPath = TraitForTypeItemPath;

    fn assoc_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssocItemPath)] {
        trai_for_ty_impl_block_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn trai_for_ty_impl_block_item_paths(
    db: &::salsa::Db,
    path: TraitForTypeImplBlockPath,
) -> SmallVecPairMap<Ident, TraitForTypeItemPath, 2> {
    path.syn_node_path(db)
        .assoc_items(db)
        .iter()
        .filter_map(|&(ident, syn_node_path, _)| Some((ident, syn_node_path.path(db)?)))
        .collect()
}
