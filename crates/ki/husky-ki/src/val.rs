use crate::*;

use husky_entity_path::{FugitivePath, TypeVariantPath};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_linkage::linkage::Linkage;
use husky_term_prelude::literal::Literal;
use smallvec::SmallVec;

#[salsa::interned(db = ValDb, jar = ValJar)]
pub struct Ki {
    pub domain: ValDomain,
    pub opn: ValOpn,
    #[return_ref]
    pub arguments: SmallVec<[ValArgument; 4]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValArgument {
    Simple(Ki),
    Keyed(Option<Ki>),
    Variadic(Vec<Ki>),
    Branch {
        condition: Option<Ki>,
        stmts: SmallVec<[Ki; 4]>,
    },
    RuntimeConstants(SmallVec<[ValRuntimeConstant; 4]>),
}

#[salsa::interned(jar = ValJar)]
pub struct ValRuntimeConstant {
    pub data: ValRuntimeConstantData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ValRuntimeConstantData {
    TypeVariantPath(TypeVariantPath),
}

#[test]
fn val_runtime_constants_works() {
    use husky_task_interface::val_repr::ValRuntimeConstantInterface;

    assert_eq!(
        std::mem::size_of::<ValRuntimeConstant>(),
        std::mem::size_of::<ValRuntimeConstantInterface>(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValDomain {
    ConditionSatisfied(Ki),
    ConditionNotSatisfied(Ki),
    StmtNotReturned(Ki),
    Omni,
}

impl Ki {
    pub unsafe fn from_raw(_raw: u32) -> Self {
        todo!()
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ValOpn {
    Return,
    Require,
    Assert,
    ValItemLazilyDefined(FugitivePath),
    FunctionGn(FugitivePath),
    Prefix(HirPrefixOpr),
    Suffix(HirSuffixOpr),
    Binary(HirBinaryOpr),
    Linkage(Linkage),
    EvalDiscarded,
    Literal(Literal),
    Branches,
    TypeVariant(TypeVariantPath),
    /// use pattern_data instead of pattern for efficiency
    Be {
        pattern_data: ValPatternData,
    },
    Unwrap {},
    // ad hoc
    Index,
}

#[salsa::interned(jar = ValJar)]
pub struct ValPattern {
    data: ValPatternData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ValPatternData {
    None,
    Some,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ValSuffixOpr;

impl ValSuffixOpr {
    pub fn from_hir(_opr: HirSuffixOpr) -> Self {
        Self
    }
}
