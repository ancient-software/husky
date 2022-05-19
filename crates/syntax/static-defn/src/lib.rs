mod call;
mod root;
mod ty;

pub use call::*;
use dev_utils::StaticDevSource;
pub use root::*;
pub use ty::*;

use entity_kind::{EntityKind, MemberKind, RoutineKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::{InputContract, Linkage, OutputLiason};

#[derive(Debug, PartialEq, Eq)]
pub struct EntityStaticDefn {
    pub name: &'static str,
    pub subscopes: &'static [(&'static str, &'static EntityStaticDefn)],
    pub variant: EntityStaticDefnVariant,
    pub dev_src: StaticDevSource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EntityStaticDefnVariant {
    Routine {
        generic_placeholders: &'static [StaticGenericPlaceholder],
        input_placeholders: Vec<StaticInputParameter>,
        output_ty: &'static str,
        output_contract: OutputLiason,
        linkage: Linkage,
        routine_kind: RoutineKind,
    },
    Type {
        base_route: &'static str,
        generic_placeholders: &'static [StaticGenericPlaceholder],
        trait_impls: &'static [StaticTraitImplDefn],
        type_members: &'static [&'static EntityStaticDefn],
        variants: &'static [EntityStaticDefn],
        kind: TyKind,
        visualizer: StaticVisualizer,
        opt_type_call: Option<&'static EntityStaticDefn>,
    },
    Trait {
        base_route: &'static str,
        generic_placeholders: &'static [StaticGenericPlaceholder],
        members: &'static [EntityStaticDefn],
    },
    Module,
    TypeField {
        field_variant: StaticFieldVariant,
    },
    Method {
        this_contract: InputContract,
        input_parameters: &'static [StaticInputParameter],
        output_ty: &'static str,
        output_contract: OutputLiason,
        generic_parameters: &'static [StaticGenericPlaceholder],
        kind: MethodStaticDefnVariant,
    },
    TraitAssociatedType {
        trai: &'static str,
        traits: &'static [&'static str],
    },
    TraitAssociatedTypeImpl {
        ty: &'static str,
    },
    TraitAssociatedConstSize,
}

impl EntityStaticDefnVariant {
    pub fn entity_kind(&self) -> EntityKind {
        match self {
            EntityStaticDefnVariant::Routine { .. } => EntityKind::Routine,
            EntityStaticDefnVariant::Type { kind, .. } => EntityKind::Type(*kind),
            EntityStaticDefnVariant::Module => EntityKind::Module,
            EntityStaticDefnVariant::Trait { .. } => EntityKind::Trait,
            EntityStaticDefnVariant::Method { .. } => EntityKind::Member(MemberKind::Method),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => EntityKind::Type(TyKind::Other),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TypeField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
        }
    }
}
