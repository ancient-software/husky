use husky_syn_expr::*;
use std::{
    panic::RefUnwindSafe,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_VM_INSTRUCTION_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VmirId(pub(crate) usize);

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum VmirSource {
    Expr(SynExprIdx),
    Stmt(SynStmtIdx),
}

impl Default for VmirId {
    fn default() -> Self {
        let raw = NEXT_VM_INSTRUCTION_ID.fetch_add(1, Ordering::Relaxed);
        Self(raw)
    }
}
