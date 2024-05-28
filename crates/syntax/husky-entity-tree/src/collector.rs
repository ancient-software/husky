#[macro_use]
mod action;

use crate::*;
use husky_entity_path::path::impl_block::ImplBlockRegistry;
use husky_token::TokenDb;
use node::{
    impl_block::{ImplBlockSynNode, ImplBlockSynNodePath},
    ty_variant::HasTypeVariantPaths,
};
use vec_like::{VecMap, VecPairMap};

pub(crate) struct EntityTreeCollector<'a> {
    db: &'a ::salsa::Db,
    crate_path: CratePath,
    crate_root_path: ModulePath,
    impl_registry: ImplBlockRegistry,
    presheets: VecMap<EntityTreePresheetMut<'a>>,
    major_path_expr_arena: MajorPathExprArena,
}

impl<'a> EntityTreeCollector<'a> {
    pub(crate) fn new(db: &'a ::salsa::Db, crate_path: CratePath) -> Self {
        let crate_root = crate_path.root_module_path(db);
        let all_modules = db.all_modules_within_crate(crate_path);
        let presheets = VecMap::from_iter_assuming_no_repetitions(
            all_modules
                .into_iter()
                .map(|module_path| item_tree_presheet(db, *module_path).presheet_mut(db)),
        )
        .expect("no repetitions");
        let toolchain = crate_path.toolchain(db);
        let _path_menu = db.vfs_path_menu(toolchain);
        Self {
            db,
            crate_path,
            crate_root_path: crate_root,
            impl_registry: ImplBlockRegistry::default(),
            presheets,
            major_path_expr_arena: Default::default(),
        }
    }

    pub(crate) fn collect_all(mut self) -> EntityTreeCrateBundle {
        // for testing purposes
        let mut loop_idx = 0;
        const LOOP_LIMIT: usize = 100;
        loop {
            loop_idx += 1;
            if loop_idx >= LOOP_LIMIT {
                panic!();
            }
            let actions = self.collect_possible_actions();
            let _db = self.db;
            if actions.len() == 0 {
                break;
            }
            for action in actions {
                self.exec(action)
            }
        }
        #[cfg(test)]
        for presheet in self.presheets.iter() {
            presheet.check_done(self.db)
        }
        let impl_blocks_for_each_module = self.collect_impl_node_block_tables();
        let sheets = VecMap::from_iter_assuming_no_repetitions(
            std::iter::zip(
                self.presheets.into_iter(),
                impl_blocks_for_each_module.into_iter(),
            )
            .map(|(presheet, impl_block_syn_node_table)| {
                presheet.into_sheet(impl_block_syn_node_table)
            }),
        )
        .expect("no repetitions");
        EntityTreeCrateBundle::new(sheets, self.major_path_expr_arena)
    }

    fn collect_impl_node_block_tables(
        &mut self,
    ) -> Vec<VecPairMap<ImplBlockSynNodePath, ImplBlockSynNode>> {
        let db = self.db;
        let mut impl_blocks_for_each_module = vec![];
        for presheet in self.presheets.iter() {
            let module_path = presheet.module_path();
            let ast_sheet = module_path.ast_sheet(db);
            let impl_blocks = VecPairMap::from_iter_assuming_no_repetitions(
                ast_sheet
                    .all_ast_indexed_iter()
                    .filter_map(|(ast_idx, ast)| match ast {
                        AstData::ImplBlock {
                            token_verse_idx,
                            items,
                        } => Some(ImplBlockSynNode::parse_from_token_verse(
                            self.db,
                            self.crate_root_path,
                            context!(self, presheet),
                            module_path,
                            ast_idx,
                            *items,
                            self.db
                                .token_sheet_data(module_path)
                                .token_verse_token_stream(*token_verse_idx, None),
                            &mut self.impl_registry,
                            &mut self.major_path_expr_arena,
                        )),
                        _ => None,
                    })
                    .map(|impl_block_syn_node| {
                        (impl_block_syn_node.syn_node_path(), impl_block_syn_node)
                    }),
            )
            .expect("no repetitions");
            impl_blocks_for_each_module.push(impl_blocks);
        }
        impl_blocks_for_each_module
    }

    fn exec(&mut self, action: PresheetAction) {
        let db = self.db;
        match action {
            PresheetAction::ResolveUseExpr {
                module_path,
                rule_idx,
                path_name_token,
                symbol,
            } => self.presheets[module_path].resolve_use_expr_leading_symbol(
                db,
                rule_idx,
                path_name_token,
                symbol,
            ),
            PresheetAction::UpdateUseAllRule {
                rule_module_path,
                rule_idx,
            } => {
                let rule = &self.presheets[rule_module_path][rule_idx];
                let progress = rule
                    .progress()
                    .expect("should be okay otherwise there shouldn't be an action");
                let parent_symbols = rule
                    .parent_module_specific_symbols(db, &self.presheets)
                    .data();
                // &self.presheets[parent].module_specific_symbols().data();
                // only need to process those starting from progress
                let new_uses: Vec<EntitySymbolEntry> = parent_symbols[progress..]
                    .iter()
                    .filter_map(|entry| entry.export_via_use_all(db, rule_module_path, rule))
                    .collect();
                let progress = parent_symbols.len();
                self.presheets[rule_module_path]
                    .update_module_use_all_rule(rule_idx, new_uses, progress)
            }
            PresheetAction::UseAllTypeVariants {
                module_path,
                rule_idx,
                parent_ty_path,
            } => {
                let item_tree_presheet = &mut self.presheets[module_path];
                let new_uses: Vec<EntitySymbolEntry> = parent_ty_path
                    .ty_variant_paths(db)
                    .iter()
                    .map(|&(ident, ty_variant_path)| {
                        EntitySymbolEntry::new_use_ty_variant_entry(
                            db,
                            &item_tree_presheet[rule_idx],
                            ident,
                            ty_variant_path,
                        )
                    })
                    .collect();
                item_tree_presheet.update_use_all_ty_variants_rule(rule_idx, new_uses)
            }
            PresheetAction::Err {
                module_path,
                rule_idx,
                error,
            } => self.presheets[module_path].mark_once_use_rule_as_erroneous(rule_idx, error),
        }
    }
}
