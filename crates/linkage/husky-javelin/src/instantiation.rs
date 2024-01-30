use crate::template_argument::{ty::JavelinType, JavelinTemplateArgument};
use husky_entity_path::ItemPath;
use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolResolution},
    HirTemplateSymbol, HirTemplateSymbolClass,
};
use vec_like::SmallVecPairMap;

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JavelinInstantiation {
    pub symbol_resolutions: SmallVecPairMap<HirTemplateSymbol, JavelinTermSymbolResolution, 4>,
    pub separator: Option<u8>,
}

impl std::ops::Deref for JavelinInstantiation {
    type Target = [(HirTemplateSymbol, JavelinTermSymbolResolution)];

    fn deref(&self) -> &Self::Target {
        &self.symbol_resolutions
    }
}

impl JavelinInstantiation {
    pub(crate) fn from_hir(
        hir_instantiation: &HirInstantiation,
        javelin_instantiation: Option<&JavelinInstantiation>,
        db: &::salsa::Db,
    ) -> JavelinInstantiation {
        JavelinInstantiation {
            symbol_resolutions: hir_instantiation
                .symbol_map()
                .iter()
                .filter_map(|&(symbol, resolution)| {
                    match symbol {
                        HirTemplateSymbol::Const(symbol)
                            if symbol.index(db).class() == HirTemplateSymbolClass::Runtime =>
                        {
                            return None
                        }
                        _ => (),
                    }
                    Some((
                        symbol,
                        JavelinTermSymbolResolution::from_hir(
                            resolution,
                            javelin_instantiation,
                            db,
                        ),
                    ))
                })
                .collect(),
            separator: hir_instantiation.separator(),
        }
    }

    pub(crate) fn new_amazon(item_path: ItemPath) -> Self {
        Self {
            symbol_resolutions: Default::default(),
            separator: match item_path {
                ItemPath::Submodule(_, _) => todo!(),
                ItemPath::MajorItem(_) => None,
                ItemPath::AssociatedItem(_) => Some(0),
                ItemPath::TypeVariant(_, _) => None,
                ItemPath::ImplBlock(_) => todo!(),
                ItemPath::Attr(_, _) => todo!(),
            },
        }
    }

    pub fn is_univalent(&self) -> bool {
        self.symbol_resolutions
            .iter()
            .all(|(_, res)| res.is_univalent())
    }

    pub fn resolve(&self, symbol: impl Into<HirTemplateSymbol>) -> JavelinTermSymbolResolution {
        self.symbol_resolutions[symbol.into()].1
    }

    #[track_caller]
    pub fn resolve_ty(&self, symbol: impl Into<HirTemplateSymbol>) -> JavelinType {
        match self.symbol_resolutions[symbol.into()].1 {
            JavelinTermSymbolResolution::Explicit(JavelinTemplateArgument::Type(ty)) => ty,
            _ => unreachable!("expect type"),
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JavelinTermSymbolResolution {
    Explicit(JavelinTemplateArgument),
    SelfLifetime,
    SelfPlace,
}

impl JavelinTermSymbolResolution {
    fn from_hir(
        resolution: HirTermSymbolResolution,
        javelin_instantiation: Option<&JavelinInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match resolution {
            HirTermSymbolResolution::Explicit(template_argument) => {
                JavelinTermSymbolResolution::Explicit(JavelinTemplateArgument::from_hir(
                    template_argument,
                    javelin_instantiation,
                    db,
                ))
            }
            HirTermSymbolResolution::SelfLifetime => JavelinTermSymbolResolution::SelfLifetime,
            HirTermSymbolResolution::SelfPlace(_) => JavelinTermSymbolResolution::SelfPlace,
        }
    }

    fn is_univalent(&self) -> bool {
        match self {
            JavelinTermSymbolResolution::Explicit(arg) => match arg {
                JavelinTemplateArgument::Vacant => true,
                JavelinTemplateArgument::Type(_) => false,
                JavelinTemplateArgument::Constant(_) => false,
                JavelinTemplateArgument::Lifetime => true,
                JavelinTemplateArgument::Place => true,
            },
            JavelinTermSymbolResolution::SelfLifetime => true,
            JavelinTermSymbolResolution::SelfPlace => true,
        }
    }
}
