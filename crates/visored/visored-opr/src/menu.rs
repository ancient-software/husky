use crate::opr::binary::VdBaseBinaryOpr;
use crate::separator::VdBaseSeparator;
use crate::*;
use interned::{db::InternerDb, memo};
use opr::prefix::VdBasePrefixOpr;

#[derive(Debug, PartialEq, Eq)]
pub struct VdOprMenu {
    // ## prefix
    pub pos: VdBasePrefixOpr,
    pub neg: VdBasePrefixOpr,
    // ## binary
    pub sub: VdBaseBinaryOpr,
    // ## separator
    pub add: VdBaseSeparator,
    pub space: VdBaseSeparator,
    pub eq: VdBaseSeparator,
    pub ne: VdBaseSeparator,
    pub lt: VdBaseSeparator,
    pub gt: VdBaseSeparator,
    pub le: VdBaseSeparator,
    pub ge: VdBaseSeparator,
    pub r#in: VdBaseSeparator,
}

#[memo]
pub fn vd_opr_menu(db: &InternerDb) -> VdOprMenu {
    VdOprMenu {
        pos: VdBasePrefixOpr::POS,
        neg: VdBasePrefixOpr::NEG,
        sub: VdBaseBinaryOpr::SUB,
        add: VdBaseSeparator::ADD,
        space: VdBaseSeparator::SPACE,
        eq: VdBaseSeparator::EQ,
        ne: VdBaseSeparator::NE,
        lt: VdBaseSeparator::LT,
        gt: VdBaseSeparator::GT,
        le: VdBaseSeparator::LE,
        ge: VdBaseSeparator::GE,
        r#in: VdBaseSeparator::IN,
    }
}
