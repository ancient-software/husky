use crate::EntityDefn;
use crate::*;
use husky_atom::AtomContext;
use husky_dev_utils::DevSource;
use husky_static_defn::{EntityStaticDefn, MethodStaticDefnKind, StaticTraitImplDefn};
use map_collect::MapCollect;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDefn {}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDefn {
    pub trai: Ty,
    pub member_impls: Vec<Arc<EntityDefn>>,
    pub dev_src: DevSource,
}

impl TraitImplDefn {
    pub fn from_static(
        db: &dyn EntityDefnQueryGroup,
        symbol_context: &mut dyn AtomContext,
        static_trait_impl: &StaticTraitImplDefn,
    ) -> Arc<Self> {
        let trai = symbol_context
            .parse_entity_route(static_trait_impl.trai)
            .unwrap();
        Arc::new(Self {
            trai,
            member_impls: static_trait_impl.member_impls.map(|static_trait_impl| {
                EntityDefn::trait_member_impl_from_static(
                    db,
                    symbol_context,
                    trai,
                    static_trait_impl,
                )
            }),
            dev_src: static_trait_impl.dev_src.into(),
        })
    }

    pub fn member_impl(&self, ident: CustomIdentifier) -> &Arc<EntityDefn> {
        self.member_impls
            .iter()
            .find(|member_impl| member_impl.ident.custom() == ident)
            .unwrap()
    }
}

impl EntityDefn {
    pub fn trait_member_impl_from_static(
        db: &dyn EntityDefnQueryGroup,
        context: &mut dyn AtomContext,
        trai: Ty,
        static_trait_impl: &EntityStaticDefn,
    ) -> Arc<Self> {
        let variant =
            EntityDefnVariant::trait_member_impl_from_static(context, trai, static_trait_impl);
        let ident = context
            .entity_syntax_db()
            .it_word(static_trait_impl.name)
            .ident();
        let ty = context.opt_this_ty().unwrap();
        assert!(!matches!(ty.variant, EntityRouteVariant::ThisType { .. }));
        Self::new(
            db,
            ident,
            variant,
            context.entity_syntax_db().ty_as_trai_subroute(
                ty,
                trai,
                ident.custom(),
                Default::default(),
            ),
            context
                .entity_syntax_db()
                .intern_file(static_trait_impl.dev_src.file.into()),
            static_trait_impl.dev_src.into(),
        )
    }
}

impl EntityDefnVariant {
    pub fn trait_member_impl_from_static(
        context: &mut dyn AtomContext,
        trai: Ty,
        static_defn: &EntityStaticDefn,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => {
                EntityDefnVariant::TraitAssociatedTypeImpl {
                    trai,
                    ty: context.parse_entity_route(ty).unwrap(),
                }
            }
            EntityStaticDefnVariant::Method {
                this_modifier: this_liason,
                parameters,
                output_ty,
                output_liason,
                spatial_parameters,
                opt_linkage,
                method_static_defn_kind: MethodStaticDefnKind::TraitMethodImpl,
            } => Self::Method {
                parameters: Arc::new(
                    parameters.map(|parameter| context.parameter_from_static(parameter)),
                ),
                output_ty: RangedEntityRoute {
                    route: context.parse_entity_route(output_ty).unwrap(),
                    range: Default::default(),
                },
                this_modifier: this_liason,
                output_modifier: output_liason,
                spatial_parameters: spatial_parameters.map(|spatial_parameter| {
                    SpatialParameter::from_static(context.entity_syntax_db(), spatial_parameter)
                }),
                opt_source: opt_linkage.map(|linkage| CallFormSource::Static(linkage)),
                method_defn_kind: MethodDefnKind::TraitMethodImpl { trai },
            },
            _ => panic!(),
        }
    }
}
