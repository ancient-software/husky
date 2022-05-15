use upcast::Upcast;

use crate::*;

#[salsa::query_group(InferEntityRouteQueryGroupStorage)]
pub trait InferEntityRouteQueryGroup: DeclQueryGroup + Upcast<dyn DeclQueryGroup> {
    // fn scope_ty(&self, scope: EntityRoutePtr) -> InferResult<EntityRoutePtr>;
    // fn enum_literal_value(&self, scope: EntityRoutePtr) -> EnumLiteralValue;
    fn entity_route_sheet(&self, file: FilePtr) -> EntitySyntaxResultArc<EntityRouteSheet>;

    fn is_implicit_convertible(&self, src_ty: EntityRoutePtr, dst_ty: EntityRoutePtr) -> bool;
}
