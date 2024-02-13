use super::*;

const INDENT_INCR: u32 = 4;

#[derive(Debug, Default)]
pub(crate) struct Indent(u32);

impl Indent {
    fn incr(&mut self) {
        self.0 += INDENT_INCR
    }

    fn attr(&mut self) {
        self.0 -= INDENT_INCR
    }
}

impl<'a> AstParser<'a> {
    pub(super) fn indent(&self) -> u32 {
        self.indent.0
    }

    /// do something with indent increased
    pub(super) fn with_indent(&mut self, f: impl FnOnce(&mut Self) -> AstIdxRange) -> AstIdxRange {
        self.indent.incr();
        let result = f(self);
        self.indent.attr();
        result
    }
}
