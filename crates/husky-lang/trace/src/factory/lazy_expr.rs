use feature::{LazyExpr, LazyExprKind};
use scope::RangedScope;

use super::expr::ExprTokenConfig;
use crate::*;

impl TraceFactory {
    pub(crate) fn lazy_expr_lines(
        &self,
        expr: &Arc<LazyExpr>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<LineProps> {
        vec![LineProps {
            indent: 0,
            idx: 0,
            tokens: self.lazy_expr_tokens(expr, text, config),
        }]
    }

    pub(crate) fn lazy_expr_tokens(
        &self,
        expr: &Arc<LazyExpr>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<TokenProps> {
        let associated_trace = if config.associated {
            Some(self.new_trace(None, 0, TraceKind::LazyExpr(expr.clone()), text))
        } else {
            None
        };
        return match expr.kind {
            LazyExprKind::Literal(value) => vec![literal!(value)],
            LazyExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.lazy_expr_tokens(lopd, text, config.subexpr()));
                tokens.push(special!(opr.spaced_code(), associated_trace));
                tokens.extend(self.lazy_expr_tokens(ropd, text, config.subexpr()));
                tokens
            }
            LazyExprKind::Variable { varname, .. } => vec![ident!(varname.0, associated_trace)],
            LazyExprKind::FuncCall {
                ranged_scope,
                ref inputs,
                ..
            } => self.routine_call_tokens(ranged_scope, inputs, associated_trace, text, &config),
            LazyExprKind::ProcCall {
                ranged_scope,
                ref inputs,
                ..
            } => self.routine_call_tokens(ranged_scope, inputs, associated_trace, text, &config),
        };
    }

    fn routine_call_tokens(
        &self,
        ranged_scope: RangedScope,
        inputs: &[Arc<LazyExpr>],
        associated_trace: Option<Arc<Trace>>,
        text: &Text,
        config: &ExprTokenConfig,
    ) -> Vec<TokenProps> {
        let mut tokens = vec![
            scope!(text.ranged(ranged_scope.range), associated_trace),
            special!("("),
        ];
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                tokens.push(special!(", "));
            }
            tokens.extend(self.lazy_expr_tokens(input, text, config.subexpr()));
        }
        tokens.push(special!(")"));
        tokens
    }
}
