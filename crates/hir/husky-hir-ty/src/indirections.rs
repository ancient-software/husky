use crate::place::HirPlace;
use husky_fly_term::dispatch::{FlyIndirection, FlyIndirections};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirIndirections {
    initial_place: HirPlace,
    indirections: SmallVec<[HirIndirection; 2]>,
    final_place: HirPlace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirIndirection {
    Place(HirPlace),
    Leash,
}

impl std::ops::Deref for HirIndirections {
    type Target = [HirIndirection];

    fn deref(&self) -> &Self::Target {
        &self.indirections
    }
}

impl HirIndirections {
    pub fn from_fly(indirections: &FlyIndirections) -> Self {
        HirIndirections {
            initial_place: HirPlace::from_fly(indirections.initial_place()),
            indirections: indirections
                .iter()
                .map(|&indirection| HirIndirection::from_fly(indirection))
                .collect(),
            final_place: HirPlace::from_fly(indirections.final_place()),
        }
    }
}

impl HirIndirection {
    fn from_fly(indiretion: FlyIndirection) -> Self {
        match indiretion {
            FlyIndirection::Place(place) => HirIndirection::Place(HirPlace::from_fly(place)),
            FlyIndirection::Leash => HirIndirection::Leash,
        }
    }
}
