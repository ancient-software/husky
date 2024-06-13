pub mod assoc_ritchie;
pub mod method_ritchie;

use self::assoc_ritchie::TypeAssocRitchieFlySignature;
use super::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq)]
pub enum TypeItemFlySignature {
    AssocRitchie(TypeAssocRitchieFlySignature),
}

impl TypeItemFlySignature {
    pub fn ty(&self) -> FlyTerm {
        match self {
            TypeItemFlySignature::AssocRitchie(slf) => slf.ty(),
        }
    }
}
