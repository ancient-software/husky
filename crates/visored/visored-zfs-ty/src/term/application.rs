use super::{ZfsTerm, ZfsTermData, ZfsTermId, ZfsTerms};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsApplication(ZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsApplicationData {
    pub function: ZfsTerm,
    pub arguments: ZfsTerms,
}

impl ZfsApplication {
    pub fn data(self, db: &::salsa::Db) -> ZfsApplicationData {
        match self.0.data(db) {
            ZfsTermData::Application(data) => data,
            _ => unreachable!(),
        }
    }
}
