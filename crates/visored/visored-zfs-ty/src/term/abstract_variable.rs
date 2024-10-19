use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsAbstractVariable(ZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsAbstractVariableData {
    // Add appropriate fields here
}

impl ZfsAbstractVariable {
    pub fn data(self, db: &::salsa::Db) -> ZfsAbstractVariableData {
        match self.0.data(db) {
            ZfsTermData::AbstractVariable(data) => data,
            _ => unreachable!(),
        }
    }
}
