use super::*;

// maybe impl Frozen for &'static T where T: ?Sized
impl Frozen for &'static str {
    type Thawed = Self;

    type Slush = ();

    fn revive(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
    }
}
