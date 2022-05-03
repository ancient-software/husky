use std::sync::Arc;

use file::URange;
use lsp_types::FoldingRange;
use print_utils::epin;
use word::WordAllocator;

use fold::{FoldedList, FoldingEnd};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedText {
    pub tokens: Vec<Token>,
    pub errors: Vec<LexError>,
    token_groups: FoldedList<URange>,
}

impl TokenizedText {
    pub fn new(
        line_groups: Vec<TokenGroup>,
        tokens: Vec<Token>,
        errors: Vec<LexError>,
    ) -> TokenizedText {
        TokenizedText {
            tokens,
            errors,
            token_groups: line_groups.into(),
        }
    }

    pub fn folding_ranges(&self) -> Vec<FoldingRange> {
        epin!();
        let result = self
            .token_groups
            .nodes
            .iter()
            .filter_map(|node| {
                let start = self.tokens[node.value.start].range.start;
                let end = match node.folding_end {
                    FoldingEnd::Sibling(idx) => {
                        let last_node = &self.token_groups.nodes[idx - 1];
                        self.tokens[last_node.value.end - 1].range.end
                    }
                    FoldingEnd::Elder(idx) => {
                        let last_node = &self.token_groups.nodes[idx - 1];
                        self.tokens[last_node.value.end - 1].range.end
                    }
                    FoldingEnd::EOF => self.tokens.last().unwrap().range.end,
                };
                Some(FoldingRange {
                    start_line: start.i(),
                    start_character: Some(start.j()),
                    end_line: end.i(),
                    end_character: Some(end.j()),
                    kind: None,
                })
            })
            .collect();
        epin!();
        result
    }
}

pub type TokenGroupIter<'a> = fold::FoldIter<'a, [Token], TokenizedText>;

impl fold::FoldStorage<[Token]> for TokenizedText {
    fn len(&self) -> usize {
        self.token_groups.len()
    }

    fn folding_end(&self, index: usize) -> FoldingEnd {
        self.token_groups.folding_end(index)
    }

    fn value(&self, index: usize) -> &[Token] {
        &self.tokens[self.token_groups.value(index).clone()]
    }

    fn this(&self) -> &TokenizedText {
        self
    }

    fn indent(&self, index: usize) -> fold::Indent {
        self.token_groups.indent(index)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenGroup {
    pub(crate) indent: TextIndent,
    pub(crate) tokens: URange,
}

impl fold::ItemToFold<URange> for TokenGroup {
    fn value(&self) -> std::ops::Range<usize> {
        self.tokens.clone()
    }

    fn indent(&self) -> fold::Indent {
        self.indent.raw
    }
}

impl TokenizedText {
    pub(crate) fn parse(word_unique_allocator: &WordAllocator, text: &str) -> Arc<Self> {
        epin!();
        let mut token_scanner = TokenScanner::new(word_unique_allocator);
        epin!();
        for (i, line) in text.lines().enumerate() {
            token_scanner.scan(i, line)
        }
        epin!();
        token_scanner.gen_tokenized_text()
    }
}
