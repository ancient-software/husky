mod context;
mod db;

pub use context::*;
pub use db::*;

use husky_entity_path::EntityPath;
use husky_identifier::*;
use husky_text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Symbol {
    pub ident: Identifier,
    pub kind: SymbolKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolKind {
    EntityPath(EntityPath),
    LocalVariable { init_range: TextRange },
    FrameVariable { init_range: TextRange },
    Unrecognized,
    ThisValue,
    ThisMethod,
    ThisField,
}
