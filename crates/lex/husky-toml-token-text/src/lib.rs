mod line_group;

use husky_toml_token::TomlToken;
use line_group::produce_line_group_starts;

#[derive(Debug, PartialEq, Eq)]
pub struct TomlTokenText {
    tokens: Vec<TomlToken>,
    line_group_starts: Vec<usize>,
}

impl TomlTokenText {
    pub fn new(tokens: Vec<TomlToken>) -> Self {
        Self {
            line_group_starts: produce_line_group_starts(&tokens),
            tokens,
        }
    }

    pub fn line_groups(&self) -> impl Iterator<Item = &[TomlToken]> {
        (0..self.line_group_starts.len()).map(|line_group_index| self.line_group(line_group_index))
    }

    fn line_group(&self, line_group_index: usize) -> &[TomlToken] {
        let start = self.line_group_starts[line_group_index];
        let end = if line_group_index < self.line_group_starts.len() {
            self.line_group_starts[line_group_index + 1]
        } else {
            self.tokens.len()
        };
        &self.tokens[start..end]
    }
}
