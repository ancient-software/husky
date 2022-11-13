use crate::*;
use husky_ast::AstText;
use husky_expr_syntax::RawExprMap;
use husky_term_pattern::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferSheet {
    term_pattern_interner: TermPatternInterner,
    expr_results: RawExprMap<ExprTermPatternInferEntry>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExprTermPatternInferEntry {
    const_expr: TermPatternInferResult<Option<ConstExprPatternItd>>,
    ty: TermPatternInferResult<TermPatternItd>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExprTermPatternInferResult {
    pub(crate) const_expr: TermPatternInferResult<Option<ConstExprPattern>>,
    pub(crate) ty: TermPatternInferResult<TermPatternItd>,
}

impl TermPatternInferSheet {
    pub(crate) fn new(arena: &RawExprArena) -> Self {
        Self {
            term_pattern_interner: Default::default(),
            expr_results: RawExprMap::new(arena),
        }
    }

    pub(crate) fn term_itr_mut(&mut self) -> &mut TermPatternInterner {
        &mut self.term_pattern_interner
    }

    pub(crate) fn insert_result(&mut self, expr: RawExprIdx, result: ExprTermPatternInferResult) {
        self.expr_results.insert_new(
            expr,
            ExprTermPatternInferEntry {
                const_expr: result.const_expr.map(|const_expr| {
                    const_expr.map(|const_expr| ConstExprPatternItd::new(const_expr))
                }),
                ty: result.ty,
            },
        )
    }

    pub(crate) fn insert_term_infer_result(
        &mut self,
        _expr: RawExprIdx,
        _term: TermPatternInferResult<TermItd>,
    ) {
        todo!()
    }

    pub(crate) fn const_expr(
        &self,
        expr: RawExprIdx,
    ) -> &TermPatternInferResult<Option<ConstExprPatternItd>> {
        &self.expr_results.get(expr).unwrap().const_expr
    }
}

impl TermPatternInferSheet {
    pub fn ast_text(&self) -> &AstText {
        todo!()
    }

    pub fn expr_ty_result(&self, _expr: RawExprIdx) -> &TermPatternInferResult<Ty> {
        todo!()
    }
}
