use std::ops::Deref;

use crate::TokenData;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeEntityKeyword {
    Extern,
    Struct,
    Enum,
    Record,
    Structure,
    Inductive,
}

impl TypeEntityKeyword {
    pub fn code(self) -> &'static str {
        match self {
            TypeEntityKeyword::Extern => "extern",
            TypeEntityKeyword::Struct => "struct",
            TypeEntityKeyword::Enum => "enum",
            TypeEntityKeyword::Record => "record",
            TypeEntityKeyword::Structure => "structure",
            TypeEntityKeyword::Inductive => "inductive",
        }
    }
}

impl Deref for TypeEntityKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl From<TypeEntityKeyword> for TokenData {
    fn from(val: TypeEntityKeyword) -> Self {
        TokenData::Keyword(val.into())
    }
}
