use crate::*;

#[cfg(feature = "lsp_support")]
impl LineMap {
    #[cfg(feature = "lsp_support")]
    pub fn string_range(line_map: &LineMap, range: lsp_types::Range) -> std::ops::Range<usize> {
        let start = line_map.lsp_offset(range.start);
        let end = line_map.lsp_offset(range.end);
        start..end
    }

    pub(crate) fn lsp_offset(&self, position: lsp_types::Position) -> usize {
        let line_col = LineCol {
            line: position.line as usize,
            col: position.character as usize,
            // match line_map.encoding
            // OffsetEncoding::Utf8 => LineCol {
            //     line: position.line as u32,
            //     col: position.character as u32,
            // },
            // OffsetEncoding::Utf16 => {
            //     let line_col = LineColUtf16 {
            //         line: position.line as u32,
            //         col: position.character as u32,
            //     };
            //     line_map.index.to_utf8(line_col)
            // }
        };
        self.offset(line_col)
    }
}
