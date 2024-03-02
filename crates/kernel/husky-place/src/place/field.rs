use super::*;
use crate::jar::PlaceJar;
use husky_coword::Ident;

#[salsa::interned(jar = PlaceJar)]
pub struct FieldPlace {
    pub parent: Place,
    pub field_name: FieldName,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FieldName {
    Tuple(u8),
    Prop(Ident),
}
