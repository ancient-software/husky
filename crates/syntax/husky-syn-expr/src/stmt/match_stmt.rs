use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SynCaseBranch {
    pub vertical_token: VerticalRegionalToken,
    pub case_pattern_syn_obelisk: SynExprResult<CasePatternSyndicate>,
    pub heavy_arrow_token: SynExprResult<HeavyArrowRegionalToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl<'a> SynExprContext<'a> {
    pub(super) fn parse_case_branches(
        &mut self,
        case_branches: ItemDefnAstIdxRange,
    ) -> Vec<SynCaseBranch> {
        case_branches
            .into_iter()
            .map(|elif_branch| self.parse_case_branch(elif_branch))
            .collect()
    }

    fn parse_case_branch(&mut self, if_branch: ItemDefnAstIdx) -> SynCaseBranch {
        match self.asts()[if_branch] {
            ItemDefnAst::BasicStmtOrBranch {
                regional_token_verse_idx,
                body,
            } => {
                let access_end = self
                    .item_defn_tokra_region_data()
                    .ast_token_idx_range(if_branch)
                    .end();
                let mut parser = self.token_verse_expr_parser(regional_token_verse_idx);
                let vertical_token = parser
                    .try_parse_option()
                    .expect("guaranteed by ast")
                    .expect("guaranteed by ast");
                let case_pattern = parser.parse_case_pattern_expected(access_end);
                SynCaseBranch {
                    vertical_token,
                    case_pattern_syn_obelisk: case_pattern,
                    heavy_arrow_token: parser.try_parse_expected(
                        OriginalSynExprError::ExpectedHeavyArrowAfterCasePattern,
                    ),
                    stmts: match body {
                        Some(body) => Ok(self.parse_stmts(body)),
                        None => parser.parse_inline_stmt(access_end),
                    },
                }
            }
            _ => unreachable!(),
        }
    }
}
