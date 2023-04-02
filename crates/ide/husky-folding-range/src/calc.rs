use crate::*;
use husky_ast::{Ast, AstIdx, AstSheet, AstTokenIdxRangeSheet};

use husky_token::RangedTokenSheet;
use lsp_types::FoldingRangeKind;

pub(crate) fn calc_folding_ranges(
    ast_sheet: &AstSheet,
    ast_range_sheet: &AstTokenIdxRangeSheet,
    ranged_token_sheet: &RangedTokenSheet,
) -> Vec<FoldingRange> {
    FoldingRangeCalculator {
        ast_range_sheet,
        ast_sheet,
        ranged_token_sheet,
    }
    .calc_all()
}

struct FoldingRangeCalculator<'a> {
    ast_sheet: &'a AstSheet,
    ast_range_sheet: &'a AstTokenIdxRangeSheet,
    ranged_token_sheet: &'a RangedTokenSheet,
}

impl<'a> FoldingRangeCalculator<'a> {
    fn calc_all(self) -> Vec<FoldingRange> {
        self.ast_sheet
            .all_ast_indexed_iter()
            .filter_map(|(idx, ast)| self.calc_ast(idx, ast))
            .collect()
    }

    fn calc_ast(&self, ast_idx: AstIdx, ast: &Ast) -> Option<FoldingRange> {
        let (ast_range, kind) = match ast {
            Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Attr { .. }
            | Ast::Decr { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. }
            | Ast::TypeVariant { .. } => None,
            Ast::Defn { block, .. } => block
                .children()?
                .last()
                .map(|_| (self.ast_range_sheet[ast_idx], FoldingRangeKind::Region)),
            Ast::ImplBlock { items, .. } => items
                .ast_idx_range()
                .last()
                .map(|_| (self.ast_range_sheet[ast_idx], FoldingRangeKind::Region)),
            Ast::BasicStmtOrBranch { body, .. }
            | Ast::Main { body, .. }
            | Ast::Config { body, .. } => body
                .children()
                .last()
                .map(|_| (self.ast_range_sheet[ast_idx], FoldingRangeKind::Region)),
        }?;
        let text_range = self.ranged_token_sheet.tokens_text_range(ast_range);
        Some(FoldingRange {
            start_line: text_range.start.i(),
            start_character: Some(text_range.start.j()),
            end_line: text_range.end.i(),
            end_character: Some(text_range.end.j()),
            kind: Some(kind),
        })
    }
}
