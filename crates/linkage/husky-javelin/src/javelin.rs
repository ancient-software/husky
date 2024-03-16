use crate::{
    amazon::package_amazon_javelins, instantiation::JavInstantiation, path::JavPath,
    template_argument::ty::JavelinType, valkyrie::package_valkyrie_javelins, *,
};

use husky_vfs::PackagePath;

#[salsa::interned(db = JavelinDb, jar = JavelinJar, constructor = pub(crate) new)]
pub struct Javelin {
    #[return_ref]
    pub data: JavelinData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum JavelinData {
    PathLeading {
        path: JavPath,
        instantiation: JavInstantiation,
    },
    VecConstructor {
        element_ty: JavelinType,
    },
    TypeDefault {
        ty: JavelinType,
    },
}

/// package javelins are package amazon javelins and valkyrie javelins
pub fn package_javelins<'db>(
    db: &'db ::salsa::Db,
    package_path: PackagePath,
) -> impl Iterator<Item = Javelin> + 'db {
    package_amazon_javelins(db, package_path)
        .iter()
        .map(|&amazon_javelin| *amazon_javelin)
        .chain(
            package_valkyrie_javelins(db, package_path)
                .iter()
                .map(|&valkyrie_javelin| *valkyrie_javelin),
        )
}
