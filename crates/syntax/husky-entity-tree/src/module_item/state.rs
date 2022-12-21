use vec_like::{VecMap, VecPairMap};

use super::*;

#[derive(Debug)]
pub(super) struct CollectorState<'a> {
    module_item_maps: VecPairMap<ModulePath, IdentMap<ModuleItem>>,
    unresolved_use_exprs: VecPairMap<ModulePath, UnresolvedEntityUseExprs<'a>>,
    use_alls: Vec<UseAll>,
    has_changed: bool,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct UseAll {
    accessibility: Accessibility,
    module: EntityPath,
    use_all_parent: EntityPath,
}

#[derive(Debug)]
pub(super) struct UnresolvedEntityUseExprs<'a> {
    pub(super) sheet: &'a AstSheet,
    pub(super) exprs: Vec<UnresolvedUseExpr>,
}

#[derive(Debug)]
pub(super) struct UnresolvedUseExpr {
    pub(super) accessibility: Accessibility,
    pub(super) ident: Identifier,
    pub(super) use_expr_idx: UseExprIdx,
}

impl<'a> CollectorState<'a> {
    pub(super) fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> EntityTreeResult<Self> {
        let all_modules = all_modules_within_crate(db, crate_path).as_ref()?;
        Ok(Self {
            module_item_maps: all_modules
                .iter()
                .map(|module| -> EntityTreeResult<(_, _)> {
                    let entity_tree_sheet = db.entity_tree_sheet(*module)?;
                    let module_defn_items: IdentMap<ModuleItem> = entity_tree_sheet
                        .top_level_entities()
                        .filter_map(|(tree_idx, accessibility, entity_kind, entity_path)| {
                            match entity_kind {
                                EntityCard::Module
                                | EntityCard::Type
                                | EntityCard::Trait
                                | EntityCard::Form => Some(ModuleItem::Defn {
                                    ident: entity_path.ident(db),
                                    accessibility,
                                    tree_idx,
                                }),
                                EntityCard::Use => None,
                                EntityCard::EnumVariant => unreachable!(),
                            }
                        })
                        .collect();
                    Ok((*module, module_defn_items))
                })
                .collect::<EntityTreeResult<VecMap<_, _>>>()?,
            unresolved_use_exprs: all_modules
                .iter()
                .map(|module| -> EntityTreeResult<(_, _)> {
                    let sheet = db.ast_sheet(*module).as_ref()?;
                    todo!()
                    // let unresolved_use_exprs: Vec<UnresolvedUseExpr> = sheet
                    //     .use_exprs()
                    //     .map(
                    //         |(accessibility, ident, use_expr_idx, _ast_idx)| UnresolvedUseExpr {
                    //             accessibility: *accessibility,
                    //             ident: *ident,
                    //             use_expr_idx: *use_expr_idx,
                    //         },
                    //     )
                    //     .collect();
                    // Ok((
                    //     *module,
                    //     UnresolvedEntityUseExprs {
                    //         sheet,
                    //         exprs: unresolved_use_exprs,
                    //     },
                    // ))
                })
                .collect::<EntityTreeResult<VecMap<_, _>>>()?,
            use_alls: vec![],
            has_changed: false,
        })
    }

    pub(super) fn finish(self, db: &dyn EntityTreeDb) -> VecPairMap<ModulePath, ModuleItemMap> {
        self.module_item_maps
            .into_iter()
            .map(|(module_path, map)| (module_path, ModuleItemMap::new(db, map)))
            .collect()
    }

    pub(super) fn has_changed(&self) -> bool {
        self.has_changed
    }

    pub(super) fn reset_change_flag(&mut self) {
        self.has_changed = false
    }

    pub(super) fn unresolved_use_exprs(&self) -> &[(ModulePath, UnresolvedEntityUseExprs<'a>)] {
        &self.unresolved_use_exprs
    }

    pub(super) fn use_alls(&self) -> &[UseAll] {
        self.use_alls.as_ref()
    }

    pub(super) fn module_item_maps(&self) -> &VecPairMap<ModulePath, IdentMap<ModuleItem>> {
        &self.module_item_maps
    }
}
