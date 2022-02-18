mod func;
mod module;
mod pattern;
mod proc;
pub mod ty;

use crate::*;

pub use func::Func;
pub use module::Module;
pub use pattern::Pattern;
pub use proc::Proc;
use syntax_types::InputType;
pub use ty::Ty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Main {
    pub stmts: Vec<Arc<DeclStmt>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityKind {
    Module(Module),
    Feature(Vec<DeclStmt>),
    Pattern(Pattern),
    Func {
        inputs: Vec<(CustomIdentifier, InputType)>,
        stmts: Vec<Arc<DeclStmt>>,
    },
    Proc(Proc),
    Ty(Ty),
}
