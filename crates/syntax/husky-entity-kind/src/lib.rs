use serde::{Deserialize, Serialize};
// use husky_token_syntax::{Special, Token, TokenKind};
use husky_word::TyKeyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TyKind {
    Enum,
    Record,
    Struct,
    Primitive,
    Vec,
    Slice,
    CyclicSlice,
    Array,
    Tuple,
    Mor,
    ThickFp,
    AssociatedAny,
    ThisAny,
    TargetOutputAny,
    SpatialPlaceholderAny,
    BoxAny,
    HigherKind,
    Ref,
    Option,
}

impl From<TyKeyword> for TyKind {
    fn from(keyword: TyKeyword) -> Self {
        match keyword {
            TyKeyword::Struct => TyKind::Struct,
            TyKeyword::Enum => TyKind::Enum,
            TyKeyword::Record => TyKind::Record,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityKind {
    Module,
    Type(TyKind),
    Trait,
    Member(MemberKind),
    Function { requires_lazy: bool },
    Feature,
    EnumVariant,
    Main,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumVariantKind {
    Constant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutineKind {
    Normal,
    TypeCall,
    TypeAssociated,
    TraitAssociated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawMembRoutineKind {
    Proc,
    Func,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemberKind {
    Field,
    Method { is_lazy: bool },
    Call,
    TraitAssociatedType,
    TraitAssociatedConstSize,
    TraitAssociatedAny,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldKind {
    StructRegular,
    StructDefault,
    StructDerived,
    StructMemo, // property is not store along with struct
    RecordRegular,
    RecordProperty,
}
