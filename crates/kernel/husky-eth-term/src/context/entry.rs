use vec_like::AsVecMapEntry;

use super::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct TermSymbolShowEntry {
    symbol: EthSymbol,
    show_kind: TermSymbolShowKind,
    idx: u8,
    /// number of lambdas using this symbol
    /// level 0 means this symbol is external
    level: u8,
    external_symbol_ident: Option<Ident>,
}

impl TermSymbolShowEntry {
    #[inline(never)]
    pub(crate) fn show(
        &self,
        _db: &::salsa::Db,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(_external_symbol_ident) = self.external_symbol_ident
            && self.level == 0
        {
            todo!()
        } else {
            match self.show_kind {
                TermSymbolShowKind::Lifetime => match self.idx {
                    0 => f.write_str("'a"),
                    1 => f.write_str("'b"),
                    2 => f.write_str("'c"),
                    3 => f.write_str("'d"),
                    4 => f.write_str("'e"),
                    5 => f.write_str("'f"),
                    idx => f.write_fmt(format_args!("'a{}", idx)),
                },
                TermSymbolShowKind::Place => match self.idx {
                    0 => f.write_str("'α"),
                    1 => f.write_str("'β"),
                    2 => f.write_str("'γ"),
                    3 => f.write_str("'δ"),
                    4 => f.write_str("'ϵ"),
                    5 => f.write_str("'ζ"),
                    6 => f.write_str("'η"),
                    idx => f.write_fmt(format_args!("'α{}", idx)),
                },
                TermSymbolShowKind::Prop => match self.idx {
                    0 => f.write_str("p"),
                    1 => f.write_str("q"),
                    idx => f.write_fmt(format_args!("p{}", idx)),
                },
                TermSymbolShowKind::Type => match self.idx {
                    0 => f.write_str("t"),
                    1 => f.write_str("s"),
                    idx => f.write_fmt(format_args!("t{}", idx)),
                },
                TermSymbolShowKind::Kind => match self.idx {
                    0 => f.write_str("α"),
                    1 => f.write_str("β"),
                    2 => f.write_str("γ"),
                    3 => f.write_str("δ"),
                    4 => f.write_str("ϵ"),
                    5 => f.write_str("ζ"),
                    6 => f.write_str("η"),
                    idx => f.write_fmt(format_args!("α{}", idx)),
                },
                TermSymbolShowKind::Other => match self.idx {
                    0 => f.write_str("a"),
                    1 => f.write_str("b"),
                    idx => f.write_fmt(format_args!("a{}", idx)),
                },
            }
        }
    }
}

impl AsVecMapEntry for TermSymbolShowEntry {
    type K = EthSymbol;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.symbol
    }

    fn key_ref(&self) -> &Self::K {
        &self.symbol
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum TermSymbolShowKind {
    Lifetime,
    Place,
    Prop,
    Type,
    Kind,
    Other,
}

impl TermShowContext {
    pub(super) fn new_external_entry(
        &self,
        db: &::salsa::Db,
        symbol: EthSymbol,
        external_symbol_ident: Option<Ident>,
    ) -> TermSymbolShowEntry {
        self.new_entry(db, symbol, 0, external_symbol_ident)
    }

    pub(super) fn new_internal_entry(
        &self,
        db: &::salsa::Db,
        symbol: EthSymbol,
    ) -> TermSymbolShowEntry {
        self.new_entry(db, symbol, 1, None)
    }

    fn new_entry(
        &self,
        db: &::salsa::Db,
        symbol: EthSymbol,
        level: u8,
        external_symbol_ident: Option<Ident>,
    ) -> TermSymbolShowEntry {
        let show_kind = symbol_show_kind(symbol, db);
        let idx = self.issue_idx(show_kind);
        TermSymbolShowEntry {
            symbol,
            show_kind,
            idx,
            level,
            external_symbol_ident,
        }
    }

    fn issue_idx(&self, show_kind: TermSymbolShowKind) -> u8 {
        let last_idx = self
            .entries
            .data()
            .iter()
            .rev()
            .find(|entry| entry.show_kind == show_kind)
            .map(|entry| entry.idx);
        match last_idx {
            Some(last_idx) => last_idx + 1,
            None => 0,
        }
    }

    // todo: put this into an internal table struct
    pub(super) fn enter_block(&mut self, db: &::salsa::Db, symbol: EthSymbol) {
        if let Some(entry) = self.entries.get_entry_mut(symbol) {
            entry.level += 1
        } else {
            let new_entry = self.new_internal_entry(db, symbol);
            self.entries.insert_new(new_entry).unwrap();
        }
    }

    pub(super) fn exit_block(&mut self, symbol: EthSymbol) {
        self.entries.get_entry_mut(symbol).unwrap().level -= 1
    }
}

fn symbol_show_kind(symbol: EthSymbol, db: &::salsa::Db) -> TermSymbolShowKind {
    match symbol.ty(db) {
        EthTerm::EntityPath(ItemPathTerm::TypeOntology(path)) if path.eqs_lifetime_ty_path(db) => {
            TermSymbolShowKind::Lifetime
        }
        EthTerm::Category(cat) if cat.universe().raw() == 0 => TermSymbolShowKind::Prop,
        EthTerm::Category(cat) if cat.universe().raw() == 1 => TermSymbolShowKind::Type,
        EthTerm::Category(_) => TermSymbolShowKind::Kind,
        _ => TermSymbolShowKind::Other,
    }
}
