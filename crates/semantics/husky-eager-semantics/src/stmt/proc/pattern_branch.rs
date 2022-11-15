use crate::*;
use husky_file::FileItd;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_text::TextRange;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcStmtPatternBranch {
    pub variant: ProcStmtPatternBranchVariant,
    pub stmts: Arc<Vec<Arc<ProcStmt>>>,
    pub range: TextRange,
    pub file: FileItd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcStmtPatternBranchVariant {
    Case { pattern: ProcStmtPattern },
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcStmtPattern {
    pub ty: Ty,
    pub variant: ProcStmtPatternVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcStmtPatternVariant {
    PrimitiveLiteral(RawLiteralData),
    OneOf { subpatterns: Vec<ProcStmtPattern> },
    EnumLiteral(Ty),
}
