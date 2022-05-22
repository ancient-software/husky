use dev_utils::static_dev_src;
use vm::{InputLiason, OutputLiason};

use crate::*;

pub static STD_OPS_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "ops",
    subscopes: &[
        ("Index", &INDEX_TRAIT_DEFN),
        ("IndexMut", &INDEX_TRAIT_DEFN),
    ],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

pub static INDEX_TRAIT_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Index",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Trait {
        base_route: "std::ops::Index",
        generic_placeholders: &[StaticGenericPlaceholder {
            name: "Idx",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        members: &[
            EntityStaticDefn {
                name: "Output",
                subscopes: &[],
                variant: EntityStaticDefnVariant::TraitAssociatedType {
                    trai: "std::ops::Index",
                    traits: &[],
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "index",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputLiason::MemberAccess,
                    input_parameters: &[],
                    output_ty: "This::Output",
                    output_liason: OutputLiason::MemberAccess,
                    generic_parameters: &[],
                    kind: MethodStaticDefnVariant::TraitMethod {
                        opt_default_source: None,
                    },
                },
                dev_src: static_dev_src!(),
            },
        ],
    },
    dev_src: dev_utils::static_dev_src!(),
};
