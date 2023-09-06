use husky_token::{HasTokenIdxRange, RangedTokenSheet, TokenIdxRange, TokenSheetData};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = AstDb)]
pub struct AstTokenIdxRangeSheet {
    ast_token_idx_ranges: Vec<TokenIdxRange>,
}

#[salsa::tracked(jar = AstJar, return_ref)]
pub(crate) fn ast_token_idx_range_sheet(
    db: &dyn AstDb,
    module_path: ModulePath,
) -> VfsResult<AstTokenIdxRangeSheet> {
    let token_sheet_data = db.token_sheet_data(module_path)?;
    let ast_sheet = db.ast_sheet(module_path)?;
    Ok(AstTokenIdxRangeSheet {
        ast_token_idx_ranges: AstTokenIdxRangeCalculator {
            token_sheet_data,
            ast_sheet,
            ast_ranges: Default::default(),
        }
        .calc_all(),
    })
}

#[test]
fn ast_range_sheet_works() {
    use tests::*;
    DB::default()
        .token_expect_test_debug_with_db("ast_range_sheet", AstDb::ast_token_idx_range_sheet);
}

impl std::ops::Index<AstIdx> for AstTokenIdxRangeSheet {
    type Output = TokenIdxRange;

    fn index(&self, index: AstIdx) -> &Self::Output {
        &self.ast_token_idx_ranges[index.raw()]
    }
}

struct AstTokenIdxRangeCalculator<'a> {
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    ast_ranges: Vec<TokenIdxRange>,
}

impl<'a> AstTokenIdxRangeCalculator<'a> {
    fn calc_all(mut self) -> Vec<TokenIdxRange> {
        for ast in self.ast_sheet.ast_arena.data() {
            self.ast_ranges.push(self.calc_ast(ast))
        }
        self.ast_ranges
    }

    fn calc_ast(&self, ast: &Ast) -> TokenIdxRange {
        match ast {
            Ast::Err {
                token_group_idx, ..
            }
            | Ast::Use {
                token_group_idx, ..
            }
            | Ast::Sorc {
                token_group_idx, ..
            }
            | Ast::Decr {
                token_group_idx, ..
            }
            | Ast::TypeVariant {
                token_group_idx, ..
            } => self
                .token_sheet_data
                .token_group_token_idx_range(*token_group_idx),
            Ast::BasicStmtOrBranch {
                token_group_idx,
                body,
                ..
            } => self.calc_ast_group(*token_group_idx, body.map(|body| body.ast_idx_range())),
            Ast::Defn {
                token_group_idx,
                block,
                ..
            } => self.calc_ast_group(*token_group_idx, block.children()),
            Ast::ImplBlock {
                token_group_idx,
                items: body,
                ..
            } => self.calc_ast_group(*token_group_idx, body.map(|body| body.ast_idx_range())),
            Ast::Config {
                token_group_idx,
                body,
            } => self.calc_ast_group(*token_group_idx, body.ast_idx_range()),
            Ast::Main {
                token_group_idx,
                body,
            } => self.calc_ast_group(*token_group_idx, body.ast_idx_range()),
            Ast::IfElseStmts {
                if_branch: if_stmt,
                elif_branches: elif_stmts,
                else_branch: else_stmt,
            } => {
                let if_stmt_token_idx_range = self.ast_ranges[if_stmt.raw()].token_idx_range();
                let start = if_stmt_token_idx_range.start();
                let end = match else_stmt {
                    Some(else_stmt) => self.ast_ranges[else_stmt.raw()].end(),
                    None => {
                        if let Some(last) = elif_stmts.last() {
                            self.ast_ranges[last.raw()].end()
                        } else {
                            self.ast_ranges[if_stmt.raw()].end()
                        }
                    }
                };
                (start, end).into()
            }
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
                ..
            } => {
                let pattern_stmt_token_idx_range =
                    self.ast_ranges[pattern_stmt.raw()].token_idx_range();
                let start = pattern_stmt_token_idx_range.start();
                let end = {
                    if let Some(last) = case_stmts.last() {
                        self.ast_ranges[last.raw()].end()
                    } else {
                        pattern_stmt_token_idx_range.end()
                    }
                };
                (start, end).into()
            }
        }
    }

    fn calc_ast_group(
        &self,
        token_group_idx: TokenGroupIdx,
        ast_idx_range: impl Into<Option<AstIdxRange>>,
    ) -> TokenIdxRange {
        let token_group_token_idx_range = self
            .token_sheet_data
            .token_group_token_idx_range(token_group_idx);
        let start = token_group_token_idx_range.start();
        let ast_idx_range: Option<AstIdxRange> = ast_idx_range.into();
        let end = match ast_idx_range {
            Some(ast_idx_range) => match ast_idx_range.last() {
                Some(last) => self.ast_ranges[last.raw()].end(),
                None => token_group_token_idx_range.end(),
            },
            None => token_group_token_idx_range.end(),
        };
        (start, end).into()
    }
}
