use crate::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SynExprRangeRegion {
    item_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_expr_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    stmt_ranges: SynStmtMap<RegionalTokenIdxRange>,
}

#[salsa::tracked(jar = SynExprJar, return_ref)]
pub(crate) fn syn_expr_range_region(
    db: &::salsa::Db,
    expr_region: SynExprRegion,
) -> SynExprRangeRegion {
    SynExprRangeCalculator::new(db, expr_region).calc_all()
}

impl std::ops::Index<PrincipalEntityPathSynExprIdx> for SynExprRangeRegion {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: PrincipalEntityPathSynExprIdx) -> &Self::Output {
        &self.item_path_expr_ranges[index.index()]
    }
}

impl std::ops::Index<SynPatternExprIdx> for SynExprRangeRegion {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.index()]
    }
}

impl std::ops::Index<SynExprIdx> for SynExprRangeRegion {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

struct SynExprRangeCalculator<'a> {
    syn_expr_region_data: &'a SynExprRegionData,
    principal_entity_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_expr_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    stmt_ranges: SynStmtMap<RegionalTokenIdxRange>,
}

impl<'a> std::ops::Index<PrincipalEntityPathSynExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: PrincipalEntityPathSynExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<&PrincipalEntityPathSynExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: &PrincipalEntityPathSynExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SynPatternExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SynExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<&SynExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: &SynExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SynStmtIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynStmtIdx) -> &Self::Output {
        &self.stmt_ranges[index]
    }
}

impl<'a> SynExprRangeCalculator<'a> {
    fn new(db: &'a ::salsa::Db, syn_expr_region: SynExprRegion) -> Self {
        let syn_expr_region_data = syn_expr_region.data(db);
        SynExprRangeCalculator {
            syn_expr_region_data,
            principal_entity_path_expr_ranges: Default::default(),
            pattern_expr_ranges: Default::default(),
            expr_ranges: Default::default(),
            stmt_ranges: SynStmtMap::new(syn_expr_region_data.stmt_arena()),
        }
    }

    fn calc_all(mut self) -> SynExprRangeRegion {
        // order matters
        self.principal_entity_path_expr_ranges.reserve(
            self.syn_expr_region_data
                .principal_item_path_expr_arena()
                .len(),
        );
        for principal_entity_path_expr in self
            .syn_expr_region_data
            .principal_item_path_expr_arena()
            .iter()
        {
            self.principal_entity_path_expr_ranges
                .push(self.calc_principal_entity_path_expr_range(principal_entity_path_expr))
        }
        self.pattern_expr_ranges
            .reserve(self.syn_expr_region_data.pattern_expr_arena().len());
        for pattern_expr in self.syn_expr_region_data.pattern_expr_arena().iter() {
            self.pattern_expr_ranges
                .push(self.calc_pattern_expr_range(pattern_expr))
        }
        self.expr_ranges
            .reserve(self.syn_expr_region_data.expr_arena().len());
        for expr in self.syn_expr_region_data.expr_arena().iter() {
            let expr_range = self.calc_expr_range(expr);
            self.expr_ranges.push(expr_range)
        }
        assert_eq!(
            self.syn_expr_region_data.expr_arena().len(),
            self.expr_ranges.len()
        );
        SynExprRangeRegion {
            item_path_expr_ranges: self.principal_entity_path_expr_ranges,
            pattern_expr_ranges: self.pattern_expr_ranges,
            expr_ranges: self.expr_ranges,
            stmt_ranges: self.stmt_ranges,
        }
    }

    fn calc_principal_entity_path_expr_range(
        &self,
        expr: &SynPrincipalEntityPathExpr,
    ) -> RegionalTokenIdxRange {
        match expr {
            SynPrincipalEntityPathExpr::Root {
                path_name_token, ..
            } => RegionalTokenIdxRange::new_single(path_name_token.regional_token_idx()),
            SynPrincipalEntityPathExpr::Subitem {
                parent,
                colon_colon_token,
                ident_token,
                ..
            } => match ident_token {
                Ok(ident_token) => self[parent].to(RegionalTokenIdxRangeEnd::new_after(
                    ident_token.regional_token_idx(),
                )),
                Err(_) => self[parent].to(RegionalTokenIdxRangeEnd::new_after(
                    colon_colon_token.regional_token_idx(),
                )),
            },
        }
    }

    fn calc_pattern_expr_range(&self, expr: &SynPatternExprData) -> RegionalTokenIdxRange {
        match expr {
            SynPatternExprData::Literal {
                regional_token_idx, ..
            } => RegionalTokenIdxRange::new_single(*regional_token_idx),
            SynPatternExprData::Ident {
                symbol_modifier_tokens,
                ident_token,
            } => match symbol_modifier_tokens {
                Some(EphemSymbolModifierRegionalTokens::Owned(owned_token)) => {
                    RegionalTokenIdxRange::new_closed(
                        owned_token.regional_token_idx(),
                        ident_token.regional_token_idx(),
                    )
                }
                Some(EphemSymbolModifierRegionalTokens::Mut(mut_token)) => {
                    RegionalTokenIdxRange::new_closed(
                        mut_token.regional_token_idx(),
                        ident_token.regional_token_idx(),
                    )
                }
                Some(
                    EphemSymbolModifierRegionalTokens::Ref(ref_token)
                    | EphemSymbolModifierRegionalTokens::RefMut(ref_token, ..),
                ) => RegionalTokenIdxRange::new_closed(
                    ref_token.regional_token_idx(),
                    ident_token.regional_token_idx(),
                ),
                Some(_) => todo!(),
                None => RegionalTokenIdxRange::new_single(ident_token.regional_token_idx()),
            },
            SynPatternExprData::UnitTypeVariant { path_expr_idx, .. } => {
                self.principal_entity_path_expr_ranges[path_expr_idx.index()]
            }
            SynPatternExprData::Tuple { .. } => todo!(),
            SynPatternExprData::TupleStruct { .. } => todo!(),
            SynPatternExprData::TupleTypeVariant {
                path_expr_idx,
                rpar,
                ..
            } => self.principal_entity_path_expr_ranges[path_expr_idx.index()].to(
                RegionalTokenIdxRangeEnd::new_after(rpar.regional_token_idx()),
            ),
            SynPatternExprData::Props { .. } => todo!(),
            SynPatternExprData::OneOf { options } => {
                let fst = options.elements().first().unwrap().syn_pattern_expr_idx();
                let lst = options.elements().last().unwrap().syn_pattern_expr_idx();
                let fst_range = self.pattern_expr_ranges[fst.index()];
                let lst_range = self.pattern_expr_ranges[lst.index()];
                fst_range.join(lst_range)
            }
            SynPatternExprData::Binding { .. } => todo!(),
            SynPatternExprData::Range { .. } => todo!(),
        }
    }

    fn calc_expr_range(&mut self, expr: &SynExprData) -> RegionalTokenIdxRange {
        match expr {
            SynExprData::Literal(regional_token_idx, _)
            | SynExprData::InheritedSynSymbol {
                regional_token_idx, ..
            }
            | SynExprData::CurrentSynSymbol {
                regional_token_idx, ..
            }
            | SynExprData::FrameVarDecl {
                regional_token_idx, ..
            }
            | SynExprData::SelfType(regional_token_idx)
            | SynExprData::SelfValue(regional_token_idx) => {
                RegionalTokenIdxRange::new_single(*regional_token_idx)
            }
            SynExprData::Binary { lopd, ropd, .. } => self[lopd].join(self[ropd]),
            SynExprData::PrincipalEntityPath { path_expr_idx, .. } => self[*path_expr_idx],
            SynExprData::AssocItem {
                parent_expr_idx,
                ident_token,
                ..
            } => {
                // todo: consider implicit(angular) arguments
                self[parent_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                    ident_token.regional_token_idx(),
                ))
            }
            SynExprData::Be {
                src,
                be_regional_token_idx,
                target,
            } => {
                let start = self[src].start().regional_token_idx();
                let end = if let Ok(target) = target {
                    self[target.syn_pattern_root().syn_pattern_expr_idx()].end()
                } else {
                    RegionalTokenIdxRangeEnd::new_after(*be_regional_token_idx)
                };
                RegionalTokenIdxRange::new(start, end)
            }
            SynExprData::Prefix {
                opr_regional_token_idx,
                opd,
                ..
            } => RegionalTokenIdxRange::new(*opr_regional_token_idx, self[opd].end()),
            SynExprData::Suffix {
                opd,
                opr_regional_token_idx,
                ..
            } => self[opd].to(RegionalTokenIdxRangeEnd::new_after(*opr_regional_token_idx)),
            SynExprData::FunctionApplicationOrCall {
                function: first_expr,
                rpar_regional_token_idx,
                ..
            }
            | SynExprData::FunctionCall {
                function: first_expr,
                rpar_regional_token_idx,
                ..
            }
            | SynExprData::MethodApplicationOrCall {
                self_argument: first_expr,
                rpar_regional_token_idx,
                ..
            } => self[first_expr].to(RegionalTokenIdxRangeEnd::new_after(
                *rpar_regional_token_idx,
            )),
            SynExprData::Field {
                owner, ident_token, ..
            } => self[owner].to(RegionalTokenIdxRangeEnd::new_after(
                ident_token.regional_token_idx(),
            )),
            SynExprData::TemplateInstantiation { .. } => todo!(),
            SynExprData::ExplicitApplication {
                function_expr_idx: function,
                argument_expr_idx: argument,
            } => self[function].join(self[argument]),
            SynExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => match place_label_regional_token {
                Some(_) => todo!(),
                None => RegionalTokenIdxRange::new_single(*at_regional_token_idx),
            },
            SynExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            }
            | SynExprData::Delimitered {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            }
            | SynExprData::NewTuple {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lpar_regional_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rpar_regional_token_idx),
            ),
            SynExprData::IndexOrCompositionWithList {
                owner,
                rbox_regional_token_idx,
                ..
            } => self[owner].to(RegionalTokenIdxRangeEnd::new_after(
                *rbox_regional_token_idx,
            )),
            SynExprData::List {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lbox_regional_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rbox_regional_token_idx),
            ),
            SynExprData::BoxColonList {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lbox_regional_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rbox_regional_token_idx),
            ),
            SynExprData::Block { stmts } => self.calc_block_range(*stmts),
            SynExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                empty_html_ket,
                ..
            } => RegionalTokenIdxRange::new_closed(
                *empty_html_bra_idx,
                empty_html_ket.regional_token_idx(),
            ),
            SynExprData::Ritchie {
                ritchie_kind_regional_token_idx,
                rpar_regional_token_idx,
                return_ty_syn_expr_idx: return_ty_expr,
                ..
            } => match return_ty_expr {
                Some(return_ty_expr) => RegionalTokenIdxRange::new(
                    *ritchie_kind_regional_token_idx,
                    self[*return_ty_expr].end(),
                ),
                None => RegionalTokenIdxRange::new_closed(
                    *ritchie_kind_regional_token_idx,
                    *rpar_regional_token_idx,
                ),
            },
            SynExprData::Sorry { regional_token_idx }
            | SynExprData::Todo { regional_token_idx }
            | SynExprData::Unreachable { regional_token_idx } => {
                RegionalTokenIdxRange::new_single(*regional_token_idx)
            }
            SynExprData::Err(error) => match error {
                SynExprError::Original(error) => error.regional_token_idx_range(),
                SynExprError::Derived(_) => todo!(),
            },
            SynExprData::NestedBlock {
                lcurl_regional_token_idx,
                stmts,
                rcurl_regional_token,
            } => todo!(),
        }
    }

    fn calc_block_range(&mut self, stmts: SynStmtIdxRange) -> RegionalTokenIdxRange {
        for stmt in stmts {
            self.save_stmt_range(stmt);
        }
        self[stmts.start()].join(self[stmts.end() - 1])
    }

    fn save_stmt_range(&mut self, stmt_idx: SynStmtIdx) {
        let range = self.calc_stmt_range(stmt_idx);
        // after calculation, all the child statements must have already been computed and cached
        // so that self.stmt_ranges.len() is equal to stmt_idx.raw()
        self.stmt_ranges.insert_new(stmt_idx, range)
    }

    fn calc_stmt_range(&mut self, stmt_idx: SynStmtIdx) -> RegionalTokenIdxRange {
        match self.syn_expr_region_data[stmt_idx] {
            SynStmtData::Let {
                let_token,
                ref initial_value, /* todo: other types of let initialization */
                ..
            } => {
                let start = let_token.regional_token_idx();
                let end = self[initial_value].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmtData::Return {
                return_token,
                ref result,
            } => {
                let start = return_token.regional_token_idx();
                let end = self[result].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmtData::Require {
                require_token,
                condition,
            } => {
                let start = require_token.regional_token_idx();
                let end = self[condition].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmtData::Assert {
                assert_token,
                ref condition,
            } => {
                let start = assert_token.regional_token_idx();
                let end = self[condition].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmtData::Break { break_token } => {
                RegionalTokenIdxRange::new_single(break_token.regional_token_idx())
            }
            SynStmtData::Eval { expr_idx, .. } => self[expr_idx],
            SynStmtData::ForBetween {
                for_token,
                ref block,
                ..
            } => {
                let start = for_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmtData::ForIn { .. } => todo!(),
            SynStmtData::ForExt {
                forext_token,
                /* todo: particulars */
                ref block,
                ..
            } => {
                let start = forext_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmtData::While {
                while_token,
                ref block,
                ..
            } => {
                let start = while_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmtData::DoWhile {
                do_token,
                ref block,
                ..
            } => {
                let start = do_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                let start = if_branch.if_token.regional_token_idx();
                // it's important that every branch is computed
                let if_branch_end: RegionalTokenIdxRangeEnd =
                    self.calc_block_range(if_branch.stmts()).end();
                let mut elif_branch_rev_iter = elif_branches.iter().rev();
                let elif_branches_end: Option<RegionalTokenIdxRangeEnd> = {
                    if let Some(last_elif_branch) = elif_branch_rev_iter.next() {
                        Some(self.calc_block_range(last_elif_branch.stmts()).end())
                    } else {
                        None
                    }
                };
                for elif_branch in elif_branch_rev_iter {
                    self.calc_block_range(elif_branch.stmts());
                }
                let else_block_end: Option<RegionalTokenIdxRangeEnd> =
                    if let Some(else_branch) = else_branch {
                        Some(self.calc_block_range(else_branch.stmts()).end())
                    } else {
                        None
                    };
                let end = else_block_end
                    .or(elif_branches_end)
                    .unwrap_or(if_branch_end);
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmtData::Match { match_token, .. } => {
                // ad hoc
                RegionalTokenIdxRange::new_single(match_token.regional_token_idx())
            }
        }
    }
}
