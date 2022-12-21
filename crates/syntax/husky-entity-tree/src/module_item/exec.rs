use super::*;

#[derive(Debug, Clone, Copy)]
pub(super) enum CollectorAction<'a> {
    ResolveUseExpr {
        sheet: &'a AstSheet,
        module: ModulePath,
        accessibility: Accessibility,
        ident: Identifier,
        use_expr_idx: UseExprIdx,
    },
    UseAll(UseAll),
}

impl<'a> ModuleItemCollector<'a> {
    pub(super) fn repeat_exec_all_util_stable(&mut self) {
        loop {
            self.exec_all();
            if !self.state.has_changed() {
                break;
            } else {
                self.state.reset_change_flag()
            }
        }
    }

    fn exec_all(&mut self) {
        for action in self.available_actions() {
            self.exec(action)
        }
    }

    fn exec(&mut self, action: CollectorAction<'a>) {
        match action {
            CollectorAction::ResolveUseExpr {
                sheet: _,
                module,
                accessibility: _,
                ident,
                use_expr_idx: _,
            } => {
                if let Some(_item) = self.state.module_item_maps()[module].1.get_entry(ident) {
                    todo!()
                }
            }
            CollectorAction::UseAll(_use_all) => todo!(),
        }
    }

    fn available_actions(&self) -> Vec<CollectorAction<'a>> {
        let mut actions: Vec<CollectorAction> = vec![];
        for (module, unresolved_use_exprs) in self.state.unresolved_use_exprs() {
            for unresolved_use_expr in unresolved_use_exprs.exprs.iter() {
                actions.push(CollectorAction::ResolveUseExpr {
                    sheet: unresolved_use_exprs.sheet,
                    module: *module,
                    accessibility: unresolved_use_expr.accessibility,
                    ident: unresolved_use_expr.ident,
                    use_expr_idx: unresolved_use_expr.use_expr_idx,
                })
            }
        }
        for use_all in self.state.use_alls() {
            actions.push(CollectorAction::UseAll(*use_all))
        }
        actions
    }
}
