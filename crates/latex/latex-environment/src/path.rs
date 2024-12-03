pub mod menu;

use base_coword::Coword;
use eterned::db::EternerDb;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LxEnvironmentPath {
    name: LxEnvironmentName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LxEnvironmentName(Coword);

impl LxEnvironmentPath {
    pub fn new(name: &str, db: &EternerDb) -> Self {
        Self {
            name: LxEnvironmentName::from_ref(name, db),
        }
    }
}

impl LxEnvironmentPath {
    pub fn name(&self) -> LxEnvironmentName {
        self.name
    }
}

impl LxEnvironmentName {
    pub fn new(coword: Coword) -> Self {
        Self(coword)
    }

    pub fn from_ref(name: &str, db: &EternerDb) -> Self {
        Self(Coword::from_ref(name, db))
    }
}

impl LxEnvironmentName {
    pub fn coword(self) -> Coword {
        self.0
    }
}
