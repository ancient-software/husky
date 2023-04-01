mod entry;

use crate::*;
use entry::*;
use husky_entity_path::EntityPath;
use vec_like::{VecMap, VecPairMap, VecSet};

#[derive(Default)]
pub(crate) struct TermShowContext {
    entries: VecMap<TermSymbolShowEntry>,
}

impl TermShowContext {
    pub(crate) fn fmt_symbol(
        &mut self,
        db: &dyn TermDb,
        symbol: TermSymbol,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(entry) = self.entries.get_entry(symbol) {
            entry.show(db, f)
        } else {
            let new_entry = self.new_external_entry(db, symbol, None);
            new_entry.show(db, f);
            self.entries.insert_new(new_entry).unwrap();
            Ok(())
        }
    }

    pub(crate) fn fmt_variable(
        &mut self,
        db: &dyn TermDb,
        variable: TermPlaceholder,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        todo!()
    }

    pub(crate) fn fmt_with_symbol(
        &mut self,
        db: &dyn TermDb,
        symbol: TermSymbol,
        f: impl FnOnce(&mut Self) -> std::fmt::Result,
    ) -> std::fmt::Result {
        self.enter_block(db, symbol);
        f(self)?;
        self.exit_block(symbol);
        Ok(())
    }

    pub(crate) fn fmt_with_variable(
        &mut self,
        db: &dyn TermDb,
        variable: TermPlaceholder,
        f: impl FnOnce(&mut Self) -> std::fmt::Result,
    ) -> std::fmt::Result {
        todo!()
    }
}
