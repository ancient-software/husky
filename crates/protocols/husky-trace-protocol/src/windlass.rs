use crate::*;
use husky_linket_impl::var::IsVarId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Windlass<VarId: IsVarId> {
    Specific(VarId),
    Generic { base: Option<VarId>, limit: usize },
}

impl<VarId: IsVarId> Windlass<VarId> {
    pub fn is_generic(self) -> _ {
        match self {
            Windlass::Generic { .. } => true,
            _ => false,
        }
    }
}
