mod action;
mod use_all_rule;
mod use_expr_rule;

pub use use_all_rule::*;
pub use use_expr_rule::*;

pub(crate) use action::*;

use husky_token::TokenSheetData;

use crate::*;

use vec_like::AsVecMapEntry;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_presheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> VfsResult<EntityTreePresheet> {
    Ok(EntityTreePresheetBuilder::new(db, module_path)?.build())
}

#[test]
fn entity_tree_presheet_works() {
    DB::default().ast_expect_test_debug_with_db("entity_tree_presheet", |db, module_path| {
        entity_tree_presheet(db, module_path)
    })
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntityTreePresheet {
    module_path: ModulePath,
    native_symbol_table: EntityNodeTable,
    use_one_trackers: UseExprRules,
    use_all_trackers: UseAllRules,
    use_expr_arena: UseExprArena,
    errors: Vec<EntityTreeError>,
}

impl std::ops::Index<UseExprIdx> for EntityTreePresheet {
    type Output = UseExpr;

    fn index(&self, index: UseExprIdx) -> &Self::Output {
        &self.use_expr_arena[index]
    }
}

impl EntityTreePresheet {
    pub(crate) fn presheet_mut<'a>(
        &'a self,
        _db: &'a dyn EntityTreeDb,
    ) -> EntityTreePresheetMut<'a> {
        EntityTreePresheetMut {
            module_path: self.module_path,
            symbols: self.native_symbol_table.entity_symbol_table(),
            use_expr_rules: self.use_one_trackers.clone(),
            use_all_rules: self.use_all_trackers.clone(),
            errors: self.errors.clone(),
            use_expr_arena: &self.use_expr_arena,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EntityTreePresheetMut<'a> {
    module_path: ModulePath,
    symbols: EntitySymbolTable,
    use_expr_rules: UseExprRules,
    use_all_rules: UseAllRules,
    errors: Vec<EntityTreeError>,
    use_expr_arena: &'a UseExprArena,
}

impl<'a> EntityTreePresheetMut<'a> {
    pub(crate) fn module_path(&self) -> ModulePath {
        self.module_path
    }

    /// symbols in module except those from prelude
    pub(crate) fn module_specific_symbols(&'a self) -> EntitySymbolTableRef<'a> {
        self.symbols.as_ref()
    }

    pub(crate) fn into_sheet(self, impl_blocks: Vec<ImplBlockNode>) -> EntityTreeSheet {
        EntityTreeSheet::new(
            self.module_path,
            self.symbols,
            self.use_expr_rules,
            self.use_all_rules,
            self.errors,
            impl_blocks,
        )
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &dyn EntityTreeDb) {
        self.use_expr_rules.check_done(db)
    }

    #[cfg(test)]
    pub(crate) fn use_all_rules(&self) -> &UseAllRules {
        &self.use_all_rules
    }
}

impl<'a> AsVecMapEntry for EntityTreePresheetMut<'a> {
    type K = ModulePath;
    fn key(&self) -> ModulePath {
        self.module_path
    }

    fn key_ref(&self) -> &ModulePath {
        &self.module_path
    }
}

struct EntityTreePresheetBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    module_path: ModulePath,
    ast_sheet: &'a AstSheet,
    token_sheet_data: &'a TokenSheetData,
    entity_node_table: EntityNodeTable,
    use_expr_arena: UseExprArena,
    entity_use_trackers: UseExprRules,
    registry: EntityNodeRegistry,
}

impl<'a> EntityTreePresheetBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            module_path,
            ast_sheet: db.ast_sheet(module_path)?,
            token_sheet_data: db.token_sheet_data(module_path).unwrap(),
            use_expr_arena: Default::default(),
            entity_node_table: Default::default(),
            entity_use_trackers: Default::default(),
            registry: Default::default(),
        })
    }

    fn build(mut self) -> EntityTreePresheet {
        for (ast_idx, ast) in self.ast_sheet.all_ast_indexed_iter() {
            self.process(ast_idx, ast)
        }
        EntityTreePresheet {
            module_path: self.module_path,
            native_symbol_table: self.entity_node_table,
            use_one_trackers: self.entity_use_trackers,
            use_all_trackers: Default::default(),
            use_expr_arena: self.use_expr_arena,
            errors: self.registry.finish_with_errors(),
        }
    }

    fn process(&mut self, ast_idx: AstIdx, ast: &Ast) {
        match ast {
            Ast::Use {
                token_group_idx,
                visibility_expr,
                state_after_visibility_expr,
            } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(*token_group_idx, *state_after_visibility_expr);
                let Ok(use_expr_root) =
                    parse_use_expr_root(&mut token_stream, &mut self.use_expr_arena) else {
                        return
                    };
                if let Some(new_rule) = UseExprRule::new_root(
                    ast_idx,
                    use_expr_root,
                    visibility_expr,
                    &self.use_expr_arena,
                    self.module_path,
                ) {
                    self.entity_use_trackers.push(new_rule)
                }
            }
            Ast::Defn {
                visibility_expr,
                ident_token,
                block,
                ..
            } => {
                let visibility = visibility_expr.visibility();
                let ident = ident_token.ident();
                if let Some(entity_path) = block.entity_path() {
                    self.entity_node_table.try_add_new_node(
                        self.db,
                        &mut self.registry,
                        visibility,
                        ast_idx,
                        *ident_token,
                        entity_path,
                    )
                }
            }
            Ast::Err { .. }
            | Ast::Attr { .. }
            | Ast::Decr { .. }
            | Ast::BasicStmtOrBranch { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. }
            | Ast::TypeVariant { .. }
            | Ast::ImplBlock { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => (),
        }
    }
}
