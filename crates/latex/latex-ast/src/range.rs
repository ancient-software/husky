use latex_token::idx::{math::LxMathTokenIdxRange, rose::LxRoseTokenIdxRange};

use crate::ast::{
    math::{LxMathAstArenaMap, LxMathAstData},
    rose::{LxRoseAstArenaMap, LxRoseAstData},
    LxAstArena, LxAstArenaMap, LxAstArenaRef, LxAstData, LxAstIdx,
};

#[derive(Debug)]
pub struct LxAstTokenIdxRangeMap {
    pub(crate) math: LxMathAstArenaMap<LxMathTokenIdxRange>,
    pub(crate) rose: LxRoseAstArenaMap<LxRoseTokenIdxRange>,
}

pub fn calc_ast_token_idx_range_map(db: &salsa::Db, arena: &LxAstArena) -> LxAstTokenIdxRangeMap {
    let mut calculator = LxAstTokenIdxRangeCalculator::new(db, arena);
    calculator.infer_all();
    LxAstTokenIdxRangeMap {
        math: calculator.math_data,
        rose: calculator.rose_data,
    }
}

struct LxAstTokenIdxRangeCalculator<'a> {
    db: &'a ::salsa::Db,
    ast_arena: LxAstArenaRef<'a>,
    math_data: LxMathAstArenaMap<LxMathTokenIdxRange>,
    rose_data: LxRoseAstArenaMap<LxRoseTokenIdxRange>,
}

impl<'a> LxAstTokenIdxRangeCalculator<'a> {
    fn new(db: &'a ::salsa::Db, arena: &'a LxAstArena) -> Self {
        Self {
            db,
            ast_arena: arena.as_arena_ref(),
            math_data: LxMathAstArenaMap::new(&arena.math),
            rose_data: LxRoseAstArenaMap::new(&arena.rose),
        }
    }
}

impl<'a> LxAstTokenIdxRangeCalculator<'a> {
    fn infer_all(&mut self) {
        self.ast_arena.math().indexed_iter().for_each(|(idx, ast)| {
            self.math_data.insert_new(idx, self.calc_math_ast(ast));
        });
        self.ast_arena.rose().indexed_iter().for_each(|(idx, ast)| {
            self.rose_data.insert_new(idx, self.calc_rose_ast(ast));
        });
    }

    fn calc_math_ast(&self, data: &LxMathAstData) -> LxMathTokenIdxRange {
        match *data {
            LxMathAstData::Letter(idx, _) => LxMathTokenIdxRange::new_single(idx),
            LxMathAstData::Opr(idx, _) => LxMathTokenIdxRange::new_single(idx),
            LxMathAstData::Digit(idx, _) => LxMathTokenIdxRange::new_single(idx),
            LxMathAstData::TextEdit { ref buffer } => todo!(),
            LxMathAstData::Attach { base, ref scripts } => todo!(),
            LxMathAstData::Delimited {
                left_delimiter_token_idx,
                left_delimiter,
                asts,
                right_delimiter_token_idx,
                right_delimiter,
            } => todo!(),
        }
    }

    fn calc_rose_ast(&self, data: &LxRoseAstData) -> LxRoseTokenIdxRange {
        todo!()
    }
}
