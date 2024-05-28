pub mod condition;
mod if_else_stmt;
mod loop_stmt;
mod match_stmt;

pub use self::if_else_stmt::*;
pub use self::loop_stmt::*;
pub use self::match_stmt::*;

use crate::*;
use husky_item_defn_ast::{ItemDefnAst, ItemDefnAstIdx, ItemDefnAstIdxRange};
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use parsec::IsStreamParser;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum SynStmtData {
    Let {
        let_token: LetRegionalToken,
        let_variables_pattern: SynExprResult<LetPatternSyndicate>,
        assign_token: SynExprResult<EqRegionalToken>,
        initial_value: SynExprIdx,
    },
    Return {
        return_token: ReturnRegionalToken,
        result: SynExprIdx,
    },
    Require {
        require_token: RequireRegionalToken,
        condition: SynExprIdx,
    },
    Assert {
        assert_token: AssertRegionalToken,
        condition: SynExprIdx,
    },
    Break {
        break_token: BreakRegionalToken,
    },
    Eval {
        expr_idx: SynExprIdx,
        // todo: change this to EolOrEolSemicolonToken
        eol_semicolon: TokenDataResult<Option<EolSemicolonRegionalToken>>,
    },
    ForBetween {
        for_token: StmtForRegionalToken,
        particulars: SynForBetweenParticulars,
        for_loop_var_symbol_idx: CurrentVariableIdx,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynStmtIdxRange,
    },
    ForIn {
        for_token: StmtForRegionalToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynStmtIdxRange,
    },
    ForExt {
        forext_token: ForextRegionalToken,
        particulars: SynForextParticulars,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynStmtIdxRange,
    },
    While {
        while_token: WhileRegionalToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynStmtIdxRange,
    },
    DoWhile {
        do_token: DoRegionalToken,
        while_token: WhileRegionalToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolRegionalToken>,
        block: SynStmtIdxRange,
    },
    IfElse {
        if_branch: SynIfBranch,
        elif_branches: Vec<SynElifBranch>,
        else_branch: Option<SynElseBranch>,
    },
    Match {
        match_token: MatchRegionalToken,
        match_expr: SynExprResult<SynExprIdx>,
        eol_with_token: SynExprResult<EolWithRegionalToken>,
        case_branches: Vec<SynCaseBranch>,
    },
    Narrate {
        narrate_token: NarrateRegionalToken,
    },
}

pub type SynStmtArena = Arena<SynStmtData>;
pub type SynStmtIdx = ArenaIdx<SynStmtData>;
pub type SynStmtIdxRange = ArenaIdxRange<SynStmtData>;
pub type SynStmtMap<V> = ArenaMap<SynStmtData, V>;

impl<'a> SynExprContext<'a> {
    pub(crate) fn parse_stmts(&mut self, body: ItemDefnAstIdxRange) -> SynStmtIdxRange {
        let block_end = self.form_body_end(body);
        let stmts = body
            .into_iter()
            .map(|ast_idx| self.parse_stmt(ast_idx, block_end))
            .collect();
        self.alloc_stmts(stmts)
    }

    pub fn parse_block_expr(&mut self, body: ItemDefnAstIdxRange) -> SynExprIdx {
        let stmts = self.parse_stmts(body);
        let expr = self.alloc_expr(SynExprData::Block { stmts });
        self.add_expr_root(SynExprRootKind::BlockExpr, expr);
        expr
    }

    fn parse_stmt(
        &mut self,
        ast_idx: ItemDefnAstIdx,
        block_end: RegionalTokenIdxRangeEnd,
    ) -> SynStmtData {
        match self.defn_tokra_region_data()[ast_idx] {
            ItemDefnAst::BasicStmtOrBranch {
                regional_token_verse_idx: token_verse_idx,
                body,
            } => self.parse_basic_stmt(token_verse_idx, block_end, body),
            ItemDefnAst::IfElseStmts {
                if_branch,
                elif_branches,
                else_branch,
            } => SynStmtData::IfElse {
                if_branch: self.parse_if_branch(if_branch),
                elif_branches: self.parse_elif_branches(elif_branches),
                else_branch: self.parse_else_branch(else_branch),
            },
            ItemDefnAst::MatchStmt {
                regional_token_verse_idx,
                case_branches,
                ..
            } => {
                let mut parser = self.token_verse_expr_parser(regional_token_verse_idx);
                SynStmtData::Match {
                    match_token: parser.try_parse_option().unwrap().unwrap(),
                    match_expr: parser.parse_expr_expected(
                        Some(ExprEnvironment::Condition(block_end)),
                        OriginalSynExprError::ExpectedMatchExpr,
                    ),
                    eol_with_token: parser
                        .try_parse_expected(OriginalSynExprError::ExpectedEolWithInMatchHead),
                    case_branches: self.parse_case_branches(case_branches),
                }
            }
            ItemDefnAst::Err => todo!(),
        }
    }

    fn parse_basic_stmt(
        &mut self,
        token_verse_idx: RegionalTokenVerseIdx,
        block_end: RegionalTokenIdxRangeEnd,
        body: Option<ItemDefnAstIdxRange>,
    ) -> SynStmtData {
        let mut parser = self.token_verse_expr_parser(token_verse_idx);
        match parser.try_parse_option::<BasicStmtKeywordRegionalToken>() {
            Ok(Some(basic_stmt_keyword_token)) => match basic_stmt_keyword_token {
                BasicStmtKeywordRegionalToken::Let(let_token) => SynStmtData::Let {
                    let_token,
                    let_variables_pattern: parser.parse_let_variables_pattern_expected(block_end),
                    assign_token: parser.try_parse_expected(OriginalSynExprError::ExpectedAssign),
                    initial_value: parser.parse_expr_expected2(
                        None,
                        SynExprRootKind::LetStmtInitialValue,
                        OriginalSynExprError::ExpectedInitialValue,
                    ),
                },
                BasicStmtKeywordRegionalToken::Return(return_token) => SynStmtData::Return {
                    return_token,
                    result: parser.parse_expr_expected2(
                        None,
                        SynExprRootKind::ReturnExpr,
                        OriginalSynExprError::ExpectedResult,
                    ),
                },
                BasicStmtKeywordRegionalToken::Require(require_token) => SynStmtData::Require {
                    require_token,
                    condition: parser.parse_condition(block_end),
                },
                BasicStmtKeywordRegionalToken::Assert(assert_token) => SynStmtData::Assert {
                    assert_token,
                    condition: parser.parse_expr_expected2(
                        Some(ExprEnvironment::Condition(block_end)),
                        SynExprRootKind::Condition,
                        OriginalSynExprError::ExpectedCondition,
                    ),
                },
                BasicStmtKeywordRegionalToken::Break(break_token) => {
                    SynStmtData::Break { break_token }
                }
                BasicStmtKeywordRegionalToken::For(for_token) => {
                    let expr = match parser
                        .parse_expr_expected(None, OriginalSynExprError::ExpectedCondition)
                    {
                        Ok(expr) => expr,
                        Err(_) => todo!(),
                    };
                    let eol_colon =
                        parser.try_parse_expected(OriginalSynExprError::ExpectedEolColon);
                    self.parse_for_loop_stmt(
                        token_verse_idx,
                        for_token,
                        expr,
                        eol_colon,
                        body.expect("should be checked in `husky_ast`"),
                    )
                    .into()
                }
                BasicStmtKeywordRegionalToken::ForExt(forext_token) => {
                    let expr = match parser
                        .parse_expr_expected(None, OriginalSynExprError::ExpectedCondition)
                    {
                        Ok(expr) => expr,
                        Err(_) => todo!(),
                    };
                    let eol_colon =
                        parser.try_parse_expected(OriginalSynExprError::ExpectedEolColon);
                    self.parse_forext_loop_stmt(
                        token_verse_idx,
                        forext_token,
                        expr,
                        eol_colon,
                        body.expect("should be checked in `husky_ast`"),
                    )
                    .into()
                }
                BasicStmtKeywordRegionalToken::While(while_token) => SynStmtData::While {
                    while_token,
                    condition: parser.parse_expr_expected(
                        Some(ExprEnvironment::Condition(block_end)),
                        OriginalSynExprError::ExpectedCondition,
                    ),
                    eol_colon: parser.try_parse_expected(OriginalSynExprError::ExpectedEolColon),
                    block: self.parse_stmts(body.expect("should be checked in `husky_ast`")),
                },
                BasicStmtKeywordRegionalToken::Do(do_token) => {
                    match parser.try_parse_option::<WhileRegionalToken>() {
                        Ok(Some(while_token)) => SynStmtData::DoWhile {
                            do_token,
                            while_token,
                            condition: parser.parse_expr_expected(
                                Some(ExprEnvironment::Condition(block_end)),
                                OriginalSynExprError::ExpectedCondition,
                            ),
                            eol_colon: parser
                                .try_parse_expected(OriginalSynExprError::ExpectedEolColon),
                            block: self
                                .parse_stmts(body.expect("should be checked in `husky_ast`")),
                        },
                        Ok(None) => todo!(),
                        Err(_) => todo!(),
                    }
                }
                BasicStmtKeywordRegionalToken::Narrate(narrate_token) => {
                    SynStmtData::Narrate { narrate_token }
                }
            },
            Ok(None) => match parser.parse_expr_root(None, SynExprRootKind::EvalExpr) {
                Some(expr_idx) => SynStmtData::Eval {
                    expr_idx,
                    eol_semicolon: parser.try_parse_option(),
                },
                None => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl<'a, 'b> SynExprParser<'a, &'b mut SynExprContext<'a>> {
    pub fn parse_inline_stmt(
        &mut self,
        block_end: RegionalTokenIdxRangeEnd,
    ) -> SynExprResult<SynStmtIdxRange> {
        let syn_stmt = match self.try_parse_option::<BasicStmtKeywordRegionalToken>()? {
            Some(basic_stmt_keyword_token) => match basic_stmt_keyword_token {
                BasicStmtKeywordRegionalToken::Return(return_token) => SynStmtData::Return {
                    return_token,
                    result: self.parse_expr_expected2(
                        None,
                        SynExprRootKind::ReturnExpr,
                        OriginalSynExprError::ExpectedResult,
                    ),
                },
                BasicStmtKeywordRegionalToken::Require(require_token) => SynStmtData::Require {
                    require_token,
                    condition: self.parse_condition(block_end),
                },
                BasicStmtKeywordRegionalToken::Assert(assert_token) => SynStmtData::Assert {
                    assert_token,
                    condition: self.parse_expr_expected2(
                        Some(ExprEnvironment::Condition(block_end)),
                        SynExprRootKind::Condition,
                        OriginalSynExprError::ExpectedCondition,
                    ),
                },
                BasicStmtKeywordRegionalToken::Break(break_token) => {
                    SynStmtData::Break { break_token }
                }
                _ => todo!("err"),
            },
            None => match self.parse_expr_root(None, SynExprRootKind::EvalExpr) {
                Some(expr_idx) => SynStmtData::Eval {
                    expr_idx,
                    eol_semicolon: self.try_parse_option(),
                },
                None => todo!(),
            },
        };
        Ok(self.context.alloc_inline_stmt(syn_stmt))
    }
}
