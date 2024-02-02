use crate::*;
use either::*;
use husky_fluffy_term::{deref::DerefFlyCoersion, trival::TrivialFlyCoersion, FlyCoersion};
use husky_hir_ty::{lifetime::HirLifetime, place::HirPlace};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirEagerCoersion {
    Trivial(TrivialHirEagerCoersion),
    Never,
    WrapInSome,
    PlaceToLeash,
    Deref(DerefHirEagerCoersion),
}

impl HirEagerCoersion {
    pub const TRIVIAL_TRANSIENT: Self = HirEagerCoersion::Trivial(TrivialHirEagerCoersion {
        expectee_place: HirPlace::Transient,
    });

    pub fn place_after_coersion(self) -> HirPlace {
        match self {
            HirEagerCoersion::Trivial(slf) => slf.place_after_coersion(),
            HirEagerCoersion::Deref(slf) => slf.place_after_coersion(),
            HirEagerCoersion::Never
            | HirEagerCoersion::WrapInSome
            | HirEagerCoersion::PlaceToLeash => HirPlace::Transient,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TrivialHirEagerCoersion {
    expectee_place: HirPlace,
}
impl TrivialHirEagerCoersion {
    fn place_after_coersion(self) -> HirPlace {
        self.expectee_place
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DerefHirEagerCoersion {
    Leash,
    Ref { lifetime: HirLifetime },
}
impl DerefHirEagerCoersion {
    fn place_after_coersion(self) -> HirPlace {
        match self {
            DerefHirEagerCoersion::Leash => HirPlace::Leashed,
            DerefHirEagerCoersion::Ref { lifetime } => HirPlace::Ref {
                guard: Right(lifetime),
            },
        }
    }
}

impl ToHirEager for FlyCoersion {
    type Output = HirEagerCoersion;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        match self {
            FlyCoersion::Trivial(slf) => HirEagerCoersion::Trivial(slf.to_hir_eager(builder)),
            FlyCoersion::Never => HirEagerCoersion::Never,
            FlyCoersion::WrapInSome => HirEagerCoersion::WrapInSome,
            FlyCoersion::PlaceToLeash => HirEagerCoersion::PlaceToLeash,
            FlyCoersion::Deref(slf) => HirEagerCoersion::Deref(slf.to_hir_eager(builder)),
        }
    }
}

impl ToHirEager for TrivialFlyCoersion {
    type Output = TrivialHirEagerCoersion;

    fn to_hir_eager(&self, _builder: &mut HirEagerExprBuilder) -> Self::Output {
        TrivialHirEagerCoersion {
            expectee_place: HirPlace::from_fluffy(self.expectee_place),
        }
    }
}

impl ToHirEager for DerefFlyCoersion {
    type Output = DerefHirEagerCoersion;

    fn to_hir_eager(&self, _builder: &mut HirEagerExprBuilder) -> Self::Output {
        match *self {
            DerefFlyCoersion::Leash => DerefHirEagerCoersion::Leash,
            DerefFlyCoersion::Ref { lifetime } => DerefHirEagerCoersion::Ref {
                lifetime: HirLifetime::from_fluffy(lifetime),
            },
        }
    }
}
