use crate::{
    data::LxTokenData,
    idx::{LxRoseTokenIdx, LxTokenIdxRange},
};
use crate::{
    data::{math::LxMathTokenData, rose::LxRoseTokenData},
    idx::{LxMathTokenIdx, LxTokenIdx},
};
use husky_text_protocol::range::TextRange;

#[salsa::derive_debug_with_db]
#[derive(Default, Debug)]
pub struct LxTokenStorage {
    ranged_tokens: Vec<((usize, usize), TextRange, LxTokenData)>,
}

impl std::ops::Index<LxMathTokenIdx> for LxTokenStorage {
    type Output = LxMathTokenData;

    fn index(&self, idx: LxMathTokenIdx) -> &Self::Output {
        match &self.ranged_tokens[idx.0.index()].2 {
            LxTokenData::Math(data) => data,
            _ => unreachable!(),
        }
    }
}

/// # getters

impl LxTokenStorage {
    pub fn ranged_tokens(&self) -> &[((usize, usize), TextRange, LxTokenData)] {
        &self.ranged_tokens
    }

    pub fn whole_token_idx_range(&self) -> LxTokenIdxRange {
        LxTokenIdxRange::new(0..self.ranged_tokens.len())
    }

    pub fn token_offset_range(
        &self,
        token_idx: impl std::borrow::Borrow<LxTokenIdx>,
    ) -> (usize, usize) {
        self.ranged_tokens[token_idx.borrow().index()].0
    }

    pub fn token_idx_range_offset_range(&self, range: LxTokenIdxRange) -> (usize, usize) {
        let first = self.token_offset_range(range.start());
        match range.last() {
            Some(last) => {
                let last = self.token_offset_range(last);
                (first.0, last.1)
            }
            None => (0, 0),
        }
    }
}

/// # actions

impl LxTokenStorage {
    pub(crate) fn alloc_math_token(
        &mut self,
        start_offset: usize,
        end_offset: usize,
        range: TextRange,
        token_data: LxMathTokenData,
    ) -> LxMathTokenIdx {
        let idx = LxMathTokenIdx(LxTokenIdx::from_index(self.ranged_tokens.len()));
        self.ranged_tokens
            .push(((start_offset, end_offset), range, token_data.into()));
        idx
    }

    pub(crate) fn alloc_rose_token(
        &mut self,
        start_offset: usize,
        end_offset: usize,
        range: TextRange,
        token_data: LxRoseTokenData,
    ) -> LxRoseTokenIdx {
        let idx = LxRoseTokenIdx(LxTokenIdx::from_index(self.ranged_tokens.len()));
        self.ranged_tokens
            .push(((start_offset, end_offset), range, token_data.into()));
        idx
    }
}
