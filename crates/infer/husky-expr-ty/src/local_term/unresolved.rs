use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum UnresolvedTerm {
    ImplicitLifetime {},
    Curry {
        x: LocalTerm,
        y: LocalTerm,
    },
    Application {
        m: LocalTerm,
        n: LocalTerm,
    },
    Abstraction {
        x: TermSymbol,
        m: LocalTerm,
    },
    Durant {
        kind: TermDurantKind,
        params: Vec<UnresolvedTermDurantParameter>,
        y: LocalTerm,
    },
    Subentity {},
    AsTraitSubentity {},
    TraitConstraint {},
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermDurantParameter {
    ty: LocalTerm,
}
