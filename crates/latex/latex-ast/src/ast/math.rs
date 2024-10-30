use latex_token::{
    data::math::{digit::LxMathDigit, LxMathDelimiter},
    idx::LxTokenIdx,
};

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxMathAstData {
    Letter(LxTokenIdx, LxMathLetter),
    Opr(LxTokenIdx, LxMathPunctuation),
    Digit(LxTokenIdx, LxMathDigit),
    /// not obtained through parsing, but through ui
    TextEdit {
        buffer: String,
    },
    Attach {
        base: LxMathAstIdx,
        scripts: Vec<(LxScriptKind, LxMathAstIdx)>,
    },
    Delimited {
        left_delimiter_token_idx: LxTokenIdx,
        left_delimiter: LxMathDelimiter,
        asts: LxMathAstIdxRange,
        right_delimiter_token_idx: LxTokenIdx,
        right_delimiter: LxMathDelimiter,
    },
}

pub type LxMathAstArena = Arena<LxMathAstData>;
pub type LxMathAstArenaRef<'a> = ArenaRef<'a, LxMathAstData>;
pub type LxMathAstArenaMap<T> = ArenaMap<LxMathAstData, T>;
pub type LxMathAstIdx = ArenaIdx<LxMathAstData>;
pub type LxMathAstIdxRange = ArenaIdxRange<LxMathAstData>;

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_atomic_math_ast(&mut self) -> Option<LxMathAstData> {
        match self.peek_char()? {
            '}' | '$' => return None,
            _ => (),
        };
        let (idx, LxTokenData::Math(token)) = self.next_token()? else {
            unreachable!()
        };
        Some(match token {
            LxMathTokenData::Command(_) => todo!(),
            LxMathTokenData::LeftDelimiter(delimiter) => self.parse_delimited(idx, delimiter),
            LxMathTokenData::RightDelimiter(_) => unreachable!(),
            LxMathTokenData::Letter(letter) => LxMathAstData::Letter(idx, letter),
            LxMathTokenData::Punctuation(opr) => LxMathAstData::Opr(idx, opr), // it's not constructed into a tree yet in the ast stage
            LxMathTokenData::Digit(digit) => LxMathAstData::Digit(idx, digit),
            LxMathTokenData::Other(_) => todo!(),
            LxMathTokenData::Subscript => todo!(),
            LxMathTokenData::Superscript => todo!(),
            LxMathTokenData::Error(_) => todo!(),
        })
    }

    // here we differ from the latex syntax, we see all possible delimiters as latex delimiters
    fn parse_delimited(
        &mut self,
        left_delimiter_token_idx: LxTokenIdx,
        left_delimiter: LxMathDelimiter,
    ) -> LxMathAstData {
        let asts = self.parse_math_asts();
        let Some((idx, token)) = self.next_token() else {
            todo!()
        };
        match token {
            LxTokenData::Math(token) => match token {
                LxMathTokenData::Command(_) => todo!(),
                LxMathTokenData::LeftDelimiter(_) => todo!(),
                LxMathTokenData::RightDelimiter(right_delimiter) => LxMathAstData::Delimited {
                    left_delimiter_token_idx,
                    left_delimiter,
                    asts,
                    right_delimiter_token_idx: idx,
                    right_delimiter,
                },
                LxMathTokenData::Letter(_) => todo!(),
                LxMathTokenData::Punctuation(_) => todo!(),
                LxMathTokenData::Digit(_) => todo!(),
                LxMathTokenData::Other(_) => todo!(),
                LxMathTokenData::Subscript => todo!(),
                LxMathTokenData::Superscript => todo!(),
                LxMathTokenData::Error(_) => todo!(),
            },
            LxTokenData::Rose(_) => todo!(),
        }
    }
}
