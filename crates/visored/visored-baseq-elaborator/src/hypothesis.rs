pub mod construction;
pub mod constructor;
pub mod region;
pub mod stack;

use self::construction::VdBaseqHypothesisConstruction;
use crate::expr::VdMirExprFld;
use idx_arena::{Arena, ArenaIdx};

pub struct VdBaseqHypothesisEntry<'sess> {
    expr: VdMirExprFld<'sess>,
    construction: VdBaseqHypothesisConstruction<'sess>,
}

pub type VdBaseqHypothesisIdx<'sess> = ArenaIdx<VdBaseqHypothesisEntry<'sess>>;
pub type VdBaseqHypothesisArena<'sess> = Arena<VdBaseqHypothesisEntry<'sess>>;
