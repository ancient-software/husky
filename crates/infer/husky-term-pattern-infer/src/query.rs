use crate::*;
use husky_ast::AstQueryGroup;
use upcast::Upcast;

#[salsa::query_group(TermPatternInferQueryGroupStorage)]
pub trait TermPatternInferQueryGroup: TermDb + Upcast<dyn TermDb> {}

#[salsa::query_group(TermPatternInferSheetQueryGroupStorage)]
pub trait TermPatternInferSheetQueryGroup: TermPatternInferQueryGroup + AstQueryGroup {
    fn term_pattern_infer_sheet(&self, file: PathItd) -> FileResultArc<TermPatternInferSheet>;
}

fn term_pattern_infer_sheet(
    _db: &dyn TermPatternInferSheetQueryGroup,
    _file: PathItd,
) -> FileResultArc<TermPatternInferSheet> {
    todo!()
}
