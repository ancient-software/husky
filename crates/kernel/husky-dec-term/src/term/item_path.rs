use husky_entity_path::path::{
    major_item::{form::MajorFormPath, trai::TraitPath, ty::TypePath},
    ty_variant::TypeVariantPath,
};
use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum DecItemPath {
    Form(MajorFormPath),
    Trait(TraitPath),
    Type(TypePath),
    TypeVariant(TypeVariantPath),
}

impl From<ItemPathTerm> for DecItemPath {
    fn from(path: ItemPathTerm) -> Self {
        match path {
            ItemPathTerm::MajorForm(path) => DecItemPath::Form(path),
            ItemPathTerm::Trait(path) => DecItemPath::Trait(path),
            ItemPathTerm::TypeOntology(path) | ItemPathTerm::TypeInstance(path) => {
                DecItemPath::Type(path)
            }
            ItemPathTerm::TypeVariant(path) => DecItemPath::TypeVariant(path),
        }
    }
}

impl From<ItemPathTerm> for DecTerm {
    fn from(path: ItemPathTerm) -> Self {
        DecTerm::ItemPath(path.into())
    }
}

impl DecItemPath {
    pub fn ty_path(self) -> Option<TypePath> {
        match self {
            DecItemPath::Type(path) => Some(path),
            DecItemPath::Form(_) | DecItemPath::Trait(_) | DecItemPath::TypeVariant(_) => None,
        }
    }
}
impl From<MajorFormPath> for DecTerm {
    fn from(value: MajorFormPath) -> Self {
        DecTerm::ItemPath(value.into())
    }
}

impl From<TraitPath> for DecTerm {
    fn from(value: TraitPath) -> Self {
        DecTerm::ItemPath(value.into())
    }
}

impl From<TypePath> for DecTerm {
    fn from(value: TypePath) -> Self {
        DecTerm::ItemPath(value.into())
    }
}

impl DecItemPath {
    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        _ctx: &DecSymbolicVariableNameMap,
    ) -> std::fmt::Result {
        match self {
            DecItemPath::Form(path) => path.display_fmt_with_db(f, db),
            DecItemPath::Trait(path) => path.display_fmt_with_db(f, db),
            DecItemPath::Type(path) => path.display_fmt_with_db(f, db),
            DecItemPath::TypeVariant(path) => path.display_fmt_with_db(f, db),
        }
    }
}
