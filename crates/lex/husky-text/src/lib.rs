pub mod db;
mod line_map;
#[cfg(test)]
mod tests;

use self::db::*;
use husky_text_protocol::{line_map::*, range::*};
use husky_vfs::ModulePath;
use line_map::module_text_line_map;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Text<'a> {
    raw_text: &'a str,
    line_map: &'a LineMap,
}

pub trait HasText: Copy {
    fn text<'a>(self, db: &'a ::salsa::Db) -> Text<'a>;
}

impl HasText for ModulePath {
    fn text<'a>(self, db: &'a ::salsa::Db) -> Text<'a> {
        Text {
            raw_text: self.raw_text(db),
            line_map: module_text_line_map(db, self),
        }
    }
}

impl<'a> std::fmt::Debug for Text<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Text...")
    }
}

impl<'a> std::ops::Index<std::ops::Range<(u32, u32)>> for Text<'a> {
    type Output = str;

    fn index(&self, index: std::ops::Range<(u32, u32)>) -> &Self::Output {
        self.text_within(index.into())
    }
}

impl<'a> Text<'a> {
    pub fn text_within(self, range: TextRange) -> &'a str {
        &self.raw_text[self.line_map.offset_range(range)]
    }
}
