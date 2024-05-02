use super::*;
use vec_like::VecPairMap;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct AttrItemPath(ItemPathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AttrItemPathData {
    pub parent_item_path: ItemPath,
    // ad hoc
    // todo: change it with OriginalAttrPath
    pub ident: Ident,
    disambiguator: u8,
}

impl AttrItemPath {
    pub fn data(self, db: &::salsa::Db) -> AttrItemPathData {
        match self.0.data(db) {
            ItemPathData::Attr(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn parent(self, db: &::salsa::Db) -> ItemPath {
        self.data(db).parent_item_path
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident
    }
}

impl AttrItemPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> AttrItemPath {
        AttrItemPath(id)
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.parent_item_path.module_path(db)
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.module_path(db).toolchain(db)
    }
}

#[derive(Debug)]
pub struct AttrRegistry {
    parent_item_path: Option<ItemPath>,
    next_attr_disambiguators: VecPairMap<Ident, u8>,
}

impl AttrRegistry {
    pub fn new(parent_item_path: Option<ItemPath>) -> Self {
        Self {
            parent_item_path,
            next_attr_disambiguators: Default::default(),
        }
    }
    pub fn issue(&mut self, ident: Ident, db: &::salsa::Db) -> Result<AttrItemPath, (Ident, u8)> {
        let next_disambiguator = self
            .next_attr_disambiguators
            .get_value_mut_or_insert_default(ident);
        let disambiguator = *next_disambiguator;
        *next_disambiguator += 1;
        match self.parent_item_path {
            Some(parent_item_path) => Ok(AttrItemPath(ItemPathId::new(
                db,
                ItemPathData::Attr(AttrItemPathData {
                    parent_item_path,
                    ident,
                    disambiguator,
                }),
            ))),
            None => todo!(),
        }
    }
}
