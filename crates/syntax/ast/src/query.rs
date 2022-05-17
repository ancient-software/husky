use crate::*;
use arena::map::ArenaKeyQuery;
use entity_route_query::{EntityRouteQueryGroup, EntitySyntaxResultArc};
use file::FilePtr;
use fold::Transformer;
use fold::{FoldStorage, FoldedList};
use lsp_types::FoldingRange;
use std::fmt::Write;
use std::sync::Arc;
use test_utils::TestDisplayConfig;
use text::{Text, TextQueryGroup};
use token::AbsSemanticToken;
use upcast::Upcast;

#[salsa::query_group(AstQueryGroupStorage)]
pub trait AstSalsaQueryGroup:
    EntityRouteQueryGroup + Upcast<dyn EntityRouteQueryGroup> + TextQueryGroup
{
    fn ast_text(&self, file: FilePtr) -> EntitySyntaxResultArc<AstText>;
}

pub trait AstQueryGroup: AstSalsaQueryGroup {}

fn ast_text(this: &dyn AstSalsaQueryGroup, id: FilePtr) -> EntitySyntaxResultArc<AstText> {
    let tokenized_text = this.tokenized_text(id)?;
    let mut parser = AstTransformer::new(this, this.module(id)?)?;
    parser.transform_all(tokenized_text.iter());
    Ok(Arc::new(parser.finish()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstText {
    pub file: FilePtr,
    pub arena: RawExprArena,
    pub folded_results: FoldedList<AstResult<Ast>>,
    pub semantic_tokens: Vec<AbsSemanticToken>,
    pub text: Arc<Text>,
}

impl AstText {
    pub fn errors(&self) -> Vec<&AstError> {
        self.folded_results
            .nodes
            .iter()
            .filter_map(|node| node.value.as_ref().err())
            .collect()
    }
}

impl ArenaKeyQuery<RawExpr> for AstText {
    fn write_key(&self, config: TestDisplayConfig, raw_expr_idx: RawExprIdx, result: &mut String) {
        let expr = &self.arena[raw_expr_idx];
        let range = expr.range();
        if config.colored {
            result.push_str(print_utils::GREEN);
        }
        write!(result, "{: <15?}", range).unwrap();
        if config.colored {
            result.push_str(print_utils::CYAN);
        }
        write!(result, "{: <20}", self.text.ranged(range)).unwrap();
        if config.colored {
            result.push_str(print_utils::RESET);
        }
    }
}
