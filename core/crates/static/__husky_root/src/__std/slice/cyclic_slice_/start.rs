use super::*;

pub static STD_SLICE_CYCLIC_SLICE_START_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "start",
    items: &[],
    variant: EntityStaticDefnVariant::TyField {
        field_kind: FieldKind::StructOriginal,
        liason: MemberLiason::Mutable,
        ty: "i32",
        __Linkage: field_linkage!(VirtualCyclicSlice<'eval>, start),
    },
    dev_src: dev_utils::__static_dev_src!(),
};
