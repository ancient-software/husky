#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityClass {
    Module,
    Type,
    MajorFunctionRitchie,
    TypeAlias,
    Val,
    Trait,
    TypeVariant,
    MethodRitchie,
    AssocRitchie,
    MemoizedField,
    AssocVal,
    AssocType,
    ImplBlock,
    Attr,
    Formal,
    AssocFormal,
    Const,
    Script,
    Static,
}
