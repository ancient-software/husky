use parsec::HasStreamState;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct LetVariableDecls {
    pattern_expr_idx: PatternSynExprIdx,
    variables: CurrentSynSymbolIdxRange,
    colon_token: SynExprResult<Option<ColonToken>>,
    ty: Option<SynExprIdx>,
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn parse_let_variables_pattern_expected(
        &mut self,
        access_end: TokenIdxRangeEnd,
    ) -> SynExprResult<LetVariableDecls> {
        let state = self.save_state();
        let Some(pattern) = self.parse_pattern_expr(
            PatternSynExprInfo::Let
        )? else {
            Err(OriginalExprError::ExpectedLetVariableDecls(state))?
        };
        let symbols = self.pattern_expr_region().pattern_expr_symbols(pattern);
        let access_start = self.save_state().next_token_idx();
        let symbols = symbols
            .iter()
            .map(|(ident, pattern_symbol)| {
                CurrentSynSymbol::new(
                    self.pattern_expr_region(),
                    access_start,
                    Some(access_end),
                    CurrentSynSymbolVariant::LetVariable {
                        ident: *ident,
                        pattern_symbol_idx: *pattern_symbol,
                    },
                )
            })
            .collect::<Vec<_>>();
        let colon_token = self.try_parse_option::<ColonToken>().map_err(|e| e.into());
        let ty = match colon_token {
            Ok(Some(_)) => Some(self.parse_expr_expected2(
                Some(ExprEnvironment::TypeBeforeEq),
                ExprRootKind::LetStmtType,
                OriginalExprError::ExpectedLetVariablesType,
            )),
            _ => None,
        };
        let ty_constraint = ty.map(|ty| PatternTypeConstraint::LetVariables { pattern, ty });
        let variables = self.define_symbols(symbols, ty_constraint);
        Ok(LetVariableDecls {
            pattern_expr_idx: pattern,
            variables,
            colon_token,
            ty,
        })
    }
}

impl LetVariableDecls {
    pub fn pattern_expr_idx(&self) -> PatternSynExprIdx {
        self.pattern_expr_idx
    }

    pub fn ty(&self) -> Option<SynExprIdx> {
        self.ty
    }

    pub fn variables(&self) -> CurrentSynSymbolIdxRange {
        self.variables
    }
}
