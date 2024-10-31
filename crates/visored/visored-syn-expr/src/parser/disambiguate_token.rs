use latex_ast::ast::math::LxMathAstIdx;
use visored_annotation::annotation::space::VdSpaceAnnotation;
use visored_opr::{
    delimiter::{VdLeftDelimiter, VdRightDelimiter},
    opr::VdOpr,
    separator::VdSeparator,
};

use super::expr::{VdSynExprClass, VdSynExprIdx};

pub struct DisambiguatedMathAst {
    ast: LxMathAstIdx,
    preceding_space_annotation: Option<VdSpaceAnnotation>,
}

pub enum DisambiguatedToken {
    Expr(VdSynExprIdx, VdSynExprClass),
    Opr(VdOpr),
    Separator(VdSeparator),
    LeftDelimiter(VdLeftDelimiter),
    RightDelimiter(VdRightDelimiter),
}
