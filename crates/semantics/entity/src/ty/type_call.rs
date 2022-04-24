use atom::SymbolContext;
use defn_head::InputPlaceholder;
use entity_route::RangedEntityRoute;
use map_collect::MapCollect;
use static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use std::sync::Arc;
use vm::Linkage;

#[derive(Debug, PartialEq, Eq)]
pub struct TyCallDefn {
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output_ty: RangedEntityRoute,
    pub source: TyCallSource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TyCallSource {
    GenericStruct,
    GenericRecord,
    Static(Linkage),
}

impl TyCallDefn {
    pub fn from_static(context: &SymbolContext, static_defn: &EntityStaticDefn) -> Arc<TyCallDefn> {
        Arc::new(match static_defn.variant {
            EntityStaticDefnVariant::Routine {
                ref input_placeholders,
                output_ty,
                linkage,
                ..
            } => TyCallDefn {
                input_placeholders: Arc::new(input_placeholders.map(|input_placeholder| {
                    context.input_placeholder_from_static(input_placeholder)
                })),
                output_ty: RangedEntityRoute {
                    route: context.entity_route_from_str(output_ty).unwrap(),
                    range: Default::default(),
                },
                source: TyCallSource::Static(linkage),
            },
            _ => panic!(),
        })
    }
}
