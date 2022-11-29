use husky_entity_path::EntityPathDb;
use salsa::{storage::HasJar, DbWithJar};

use crate::*;

pub trait SymbolDb: DbWithJar<SymbolJar> + WordDb + EntityPathDb {
    fn symbol_jar(&self) -> &SymbolJar;
    fn new_symbol_ctx<'a>(&'a self) -> SymbolContext<'a>;
    fn preludes(&self) -> &[Symbol];
    fn collect_preludes(&self) -> Vec<Symbol>;
}

impl<T> SymbolDb for T
where
    T: DbWithJar<SymbolJar> + WordDb + EntityPathDb,
{
    fn symbol_jar(&self) -> &SymbolJar {
        &<Self as HasJar<SymbolJar>>::jar(&self).0
    }
    fn new_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
        SymbolContext::new(self.preludes())
    }

    fn preludes(&self) -> &[Symbol] {
        self.symbol_jar()
            .preludes_place()
            .get_or_init(|| self.collect_preludes())
    }

    fn collect_preludes(&self) -> Vec<Symbol> {
        let entity_path_menu = self.entity_path_menu();
        let mut preludes: Vec<Symbol> = vec![
            Symbol {
                ident: self.it_ident_borrowed("core"),
                kind: SymbolKind::EntityPath(entity_path_menu.core()),
            },
            Symbol {
                ident: self.it_ident_borrowed("std"),
                kind: SymbolKind::EntityPath(entity_path_menu.std()),
            },
        ];
        // ad hoc; todo: load preludes from core::prelude
        preludes
    }
}
