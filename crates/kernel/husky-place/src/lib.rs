pub mod jar;
pub mod place;

use self::jar::PlaceJar as Jar;
use husky_eth_term::term::svar::EthSvar;
use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaceIdx(ShiftedU32);

#[derive(Default)]
pub struct PlaceRegistry {
    next: ShiftedU32,
}

impl PlaceRegistry {
    pub fn issue_new(&mut self) -> PlaceIdx {
        let next = self.next;
        self.next += 1;
        PlaceIdx(next)
    }
}
