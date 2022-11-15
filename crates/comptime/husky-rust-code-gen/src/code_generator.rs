mod context;
mod gen_entity_route;
mod impl_expr;
mod impl_init;
mod impl_lib;
mod impl_mod;
mod impl_registration;
mod impl_routine_defn;
mod impl_stmt;
mod impl_ty_defn;
mod impl_write;
mod utils;

use crate::*;
use context::*;
use fold::LocalStack;
use husky_entity_kind::TyKind;
use husky_entity_semantics::{EntityDefn, EntityDefnVariant};
use husky_package_semantics::Package;
use std::sync::Arc;

pub(crate) struct RustCodeGenerator<'a> {
    db: &'a dyn RustCodeGenQueryGroup,
    result: String,
    target_entrance: FileItd,
    entity_route_uses: LocalStack<Ty>,
    context: RustCodeGenContext,
}

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn new(db: &'a dyn RustCodeGenQueryGroup, module: Ty) -> Self {
        let target_entrance = db
            .module_target_entrance(db.module_file(module).unwrap())
            .unwrap();
        let entity_defn = db.entity_defn(module).unwrap();
        let mut symbols = LocalStack::new();
        for entity_defn in entity_defn.subentities.iter() {
            symbols.push(entity_defn.base_route)
        }
        Self {
            db,
            target_entrance: target_entrance,
            result: Default::default(),
            entity_route_uses: symbols,
            context: RustCodeGenContext::Normal,
        }
    }

    pub(crate) fn new_lib(
        db: &'a dyn RustCodeGenQueryGroup,
        target_entrance: FileItd,
        use_crate_all: bool,
    ) -> Self {
        let mut symbols = LocalStack::new();
        let package = db.package(target_entrance).unwrap();
        if use_crate_all {
            for entity_defn in package.subentities.iter() {
                symbols.push(entity_defn.base_route)
            }
        }
        Self {
            db,
            target_entrance: target_entrance,
            result: Default::default(),
            entity_route_uses: symbols,
            context: RustCodeGenContext::Normal,
        }
    }

    pub(crate) fn package(&self) -> Arc<Package> {
        self.db.package(self.target_entrance).unwrap()
    }

    fn exec_within_context(&mut self, new_context: RustCodeGenContext, f: impl FnOnce(&mut Self)) {
        let old_context = std::mem::replace(&mut self.context, new_context);
        f(self);
        self.context = old_context;
    }
}
