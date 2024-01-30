use crate::*;

use salsa::DebugWithDb;
use vec_like::VecPairMap;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeSheet {
    module_path: ModulePath,
    major_item_node_table: MajorEntityNodeTable,
    item_symbol_table: EntitySymbolTable,
    // todo: split this into ty impl block and trai for ty impl block
    impl_block_syn_node_table: VecPairMap<ImplBlockSynNodePath, ImplBlockSynNode>,
    once_use_rules: OnceUseRules,
    use_all_rules: UseAllRules,
    errors: Vec<EntityTreeError>,
}

impl vec_like::AsVecMapEntry for EntityTreeSheet {
    type K = ModulePath;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.module_path
    }

    fn key_ref(&self) -> &Self::K {
        &self.module_path
    }
}

impl EntityTreeSheet {
    pub(crate) fn new(
        module_path: ModulePath,
        major_item_node_table: MajorEntityNodeTable,
        item_symbol_table: EntitySymbolTable,
        once_use_rules: OnceUseRules,
        use_all_rules: UseAllRules,
        errors: Vec<EntityTreeError>,
        impl_block_syn_node_table: VecPairMap<ImplBlockSynNodePath, ImplBlockSynNode>,
    ) -> Self {
        Self {
            module_path,
            major_item_node_table,
            item_symbol_table,
            impl_block_syn_node_table,
            once_use_rules,
            use_all_rules,
            errors,
        }
    }

    pub fn module_symbols<'a>(&'a self) -> EntitySymbolTableRef<'a> {
        self.item_symbol_table.as_ref()
    }

    pub fn errors(&self) -> &[EntityTreeError] {
        &self.errors
    }

    pub fn once_use_rule_indexed_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (OnceUseRuleIdx, &'a OnceUseRule)> + 'a {
        self.once_use_rules.indexed_iter()
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn major_item_syn_node_paths<'a>(&'a self) -> impl Iterator<Item = ItemSynNodePath> + 'a {
        self.major_item_node_table.node_paths()
    }

    pub fn major_paths<'a>(&'a self, db: &'a ::salsa::Db) -> impl Iterator<Item = ItemPath> + 'a {
        self.major_item_node_table
            .node_paths()
            .filter_map(|syn_node_path| syn_node_path.path(db))
    }

    pub(crate) fn major_item_node(&self, syn_node_path: ItemSynNodePath) -> Option<&ItemSynNode> {
        self.major_item_node_table.node(syn_node_path)
    }

    pub(crate) fn ty_impl_block_syn_node<'a>(
        &'a self,
        syn_node_path: TypeImplBlockSynNodePath,
    ) -> &'a TypeImplBlockSynNode {
        self.impl_block_syn_node_table
            .iter()
            .find_map(|(node_path1, node)| {
                (*node_path1 == syn_node_path.into()).then(|| match node {
                    ImplBlockSynNode::TypeImplBlock(node) => node,
                    _ => unreachable!(),
                })
            })
            .expect("valid node path")
    }

    pub(crate) fn trai_for_ty_impl_block_syn_node<'a>(
        &'a self,
        _db: &'a ::salsa::Db,
        syn_node_path: TraitForTypeImplBlockSynNodePath,
    ) -> &'a TraitForTypeImplBlockSynNode {
        self.impl_block_syn_node_table
            .iter()
            .find_map(|(node_path1, node)| {
                (*node_path1 == syn_node_path.into()).then(|| match node {
                    ImplBlockSynNode::TraitForTypeImplBlock(node) => node,
                    _ => unreachable!(),
                })
            })
            .expect("valid node path")
    }

    pub(crate) fn ill_formed_impl_block_syn_node<'a>(
        &'a self,
        _db: &'a ::salsa::Db,
        syn_node_path: IllFormedImplBlockSynNodePath,
    ) -> &'a IllFormedImplBlockSynNode {
        self.impl_block_syn_node_table
            .iter()
            .find_map(|(node_path1, node)| {
                (*node_path1 == syn_node_path.into()).then(|| match node {
                    ImplBlockSynNode::IllFormedImplBlock(node) => node,
                    _ => unreachable!(),
                })
            })
            .expect("valid node path")
    }

    pub fn impl_block_syn_node_paths<'a>(
        &'a self,
    ) -> impl Iterator<Item = ImplBlockSynNodePath> + 'a {
        self.impl_block_syn_node_table
            .iter()
            .map(|(syn_node_path, _)| *syn_node_path)
    }

    pub fn all_ty_impl_block_syn_node_paths<'a>(
        &'a self,
    ) -> impl Iterator<Item = TypeImplBlockSynNodePath> + 'a {
        self.impl_block_syn_node_table.iter().filter_map(
            |(syn_node_path, _)| match *syn_node_path {
                ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => Some(syn_node_path),
                _ => None,
            },
        )
    }

    pub fn all_trai_for_ty_impl_block_paths<'a>(
        &'a self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = TraitForTypeImplBlockPath> + 'a {
        self.impl_block_syn_node_table
            .iter()
            .filter_map(|(syn_node_path, _)| match syn_node_path.path(db)? {
                ImplBlockPath::TraitForTypeImplBlock(path) => Some(path),
                ImplBlockPath::TypeImplBlock(_) => None,
            })
    }

    pub fn all_impl_block_ill_forms<'a>(
        &'a self,
        _db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = &'a ImplBlockIllForm> + 'a {
        self.impl_block_syn_node_table
            .iter()
            .filter_map(|(_, impl_block)| match impl_block {
                ImplBlockSynNode::TypeImplBlock(_) => None,
                ImplBlockSynNode::TraitForTypeImplBlock(_) => None,
                ImplBlockSynNode::IllFormedImplBlock(impl_block) => Some(impl_block.ill_form()),
            })
    }
}

pub trait HasEntityTreeSheet: Copy {
    fn item_tree_sheet<'a>(self, db: &'a ::salsa::Db) -> &'a EntityTreeSheet;
}

impl HasEntityTreeSheet for ModulePath {
    fn item_tree_sheet<'a>(self, db: &'a ::salsa::Db) -> &'a EntityTreeSheet {
        item_tree_sheet(db, self)
    }
}

pub(crate) fn item_tree_sheet(db: &::salsa::Db, module_path: ModulePath) -> &EntityTreeSheet {
    let crate_path = module_path.crate_path(db);
    let item_tree_bundle = item_tree_crate_bundle(db, crate_path);
    let Some(sheet) = item_tree_bundle.get_sheet(module_path) else {
        unreachable!(
            "module {:?} should be guaranteed to be valid!!",
            module_path.debug(db)
        )
    };
    sheet
}

#[test]
fn item_tree_sheet_works() {
    DB::ast_expect_test_debug_with_db(
        |db, module_path| item_tree_sheet(db, module_path),
        &AstTestConfig::new(
            "item_tree_sheet",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    )
}
