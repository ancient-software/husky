use super::*;
use expr::{VdSynExprData, VdSynLeftDelimiter, VdSynPrefixOpr};
use expr_stack::TopVdSynExpr;
use incomplete_expr::IncompleteVdSynExprData;
use latex_token::idx::LxTokenIdx;

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub(super) fn calc_top_expr_first_token_idx(&self, top_expr: &TopVdSynExpr) -> LxTokenIdx {
        match top_expr {
            TopVdSynExpr::Complete(expr) => self.calc_expr_data_first_token_idx(expr),
            TopVdSynExpr::Incomplete(expr) => self.calc_incomplete_expr_first_token_idx(expr),
        }
    }

    pub(super) fn calc_expr_data_first_token_idx(&self, expr: &VdSynExprData) -> LxTokenIdx {
        match *expr {
            VdSynExprData::Literal {
                token_idx_range, ..
            }
            | VdSynExprData::Letter {
                token_idx_range, ..
            } => token_idx_range.start(),
            VdSynExprData::BaseOpr { opr } => todo!(),
            VdSynExprData::Binary { lopd, opr, ropd } => self.calc_expr_first_token_idx(lopd),
            VdSynExprData::Prefix { opr, opd } => self.calc_prefix_opr_first_token_idx(opr),
            VdSynExprData::Suffix { opd, opr } => self.calc_expr_first_token_idx(opd),
            VdSynExprData::SeparatedList {
                separator_class,
                items,
                ref separators,
            } => self.calc_expr_first_token_idx(items.first().unwrap()),
            VdSynExprData::LxDelimited {
                left_delimiter_token_idx,
                left_delimiter,
                item,
                right_delimiter_token_idx,
                right_delimiter,
            } => todo!(),
            VdSynExprData::Delimited { left_delimiter, .. } => {
                self.calc_left_delimiter_first_token_idx(left_delimiter)
            }
            VdSynExprData::Attach { base, ref scripts } => {
                self.calc_expr_first_token_idx(base).min(
                    scripts
                        .iter()
                        .map(|&(_, s)| self.calc_expr_first_token_idx(s))
                        .min()
                        .unwrap(),
                )
            }
            VdSynExprData::Fraction {
                command_token_idx,
                numerator,
                denominator,
                denominator_rcurl_token_idx,
            } => *command_token_idx,
            VdSynExprData::Sqrt {
                command_token_idx,
                radicand_lcurl_token_idx,
                radicand,
                radicand_rcurl_token_idx,
            } => *command_token_idx,
            VdSynExprData::UniadicChain => todo!(),
            VdSynExprData::VariadicChain => todo!(),
            VdSynExprData::UniadicArray => todo!(),
            VdSynExprData::VariadicArray => todo!(),
            VdSynExprData::Err(_) => todo!(),
        }
    }

    fn calc_expr_first_token_idx(&self, expr: VdSynExprIdx) -> LxTokenIdx {
        self.calc_expr_data_first_token_idx(&self.builder.expr_arena()[expr])
    }

    fn calc_incomplete_expr_first_token_idx(&self, expr: &IncompleteVdSynExprData) -> LxTokenIdx {
        match *expr {
            IncompleteVdSynExprData::Binary { lopd, opr } => todo!(),
            IncompleteVdSynExprData::Prefix { opr } => self.calc_prefix_opr_first_token_idx(opr),
            IncompleteVdSynExprData::SeparatedList {
                separator_class,
                ref items,
                ref separators,
            } => todo!(),
            IncompleteVdSynExprData::Delimited { left_delimiter } => {
                self.calc_left_delimiter_first_token_idx(left_delimiter)
            }
        }
    }

    fn calc_prefix_opr_first_token_idx(&self, opr: VdSynPrefixOpr) -> LxTokenIdx {
        match opr {
            VdSynPrefixOpr::Base(token_idx_range, _) => token_idx_range.start(),
            VdSynPrefixOpr::Composite(expr, _) => self.calc_expr_first_token_idx(expr),
        }
    }

    fn calc_left_delimiter_first_token_idx(
        &self,
        left_delimiter: VdSynLeftDelimiter,
    ) -> LxTokenIdx {
        match left_delimiter {
            VdSynLeftDelimiter::Base(token_idx_range, _) => token_idx_range.start(),
            VdSynLeftDelimiter::Composite(expr, _) => self.calc_expr_first_token_idx(expr),
        }
    }
}
