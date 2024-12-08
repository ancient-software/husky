use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use lean_coword::ident::LnIdent;
use lean_opr::opr::binary::LnBinaryOpr;
use lean_term::instantiation::LnInstantiation;
use smallvec::SmallVec;

use crate::expr::LnMirExprIdx;

#[derive(Debug, PartialEq, Eq)]
pub enum LnMirTacticData {
    Intro {},
    Obtain,
    Exact,
    Cases,
    Rcases,
    Have {
        // TODO: pattern??
        ident: LnIdent,
        ty: LnMirExprIdx,
        construction: LnMirExprIdx,
    },
    Show,
    Calc {
        leader: LnMirExprIdx,
        followers: SmallVec<[((LnBinaryOpr, LnInstantiation), LnMirExprIdx); 4]>,
    },
    Sorry,
    Obvious,
}

pub type LnMirTacticArena = Arena<LnMirTacticData>;
pub type LnMirTacticArenaRef<'a> = ArenaRef<'a, LnMirTacticData>;
pub type LnMirTacticIdx = ArenaIdx<LnMirTacticData>;
pub type LnMirTacticIdxRange = ArenaIdxRange<LnMirTacticData>;
