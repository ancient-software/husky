use crate::*;
pub(crate) use __husky::registration::*;

type ConnectedComponent = crate::connected_component::ConnectedComponent;

// ConnectedComponent
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __connected_component_clone(data: *mut ()) -> *mut () {
    Box::<ConnectedComponent>::into_raw(Box::new((*(data as *mut ConnectedComponent)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __connected_component_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut ConnectedComponent))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __connected_component_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const ConnectedComponent) == *(other as *const () as *const ConnectedComponent)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __connected_component_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<ConnectedComponent>(&__CONNECTED_COMPONENT_VTABLE) = registers[1].downcast_move(&__CONNECTED_COMPONENT_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __CONNECTED_COMPONENT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __connected_component_clone,
    drop: __connected_component_drop,
    eq: __connected_component_eq,
    assign: __connected_component_assign,
    typename_str_hash_u64: 3938217539454562886,
    typename_str: "ConnectedComponent",
};
type RawContour<'eval> = crate::raw_contour::RawContour<'eval>;

// RawContour
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __raw_contour_clone(data: *mut ()) -> *mut () {
    Box::<RawContour>::into_raw(Box::new((*(data as *mut RawContour)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __raw_contour_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut RawContour))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __raw_contour_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const RawContour) == *(other as *const () as *const RawContour)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __raw_contour_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<RawContour>(&__RAW_CONTOUR_VTABLE) = registers[1].downcast_move(&__RAW_CONTOUR_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __RAW_CONTOUR_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __raw_contour_clone,
    drop: __raw_contour_drop,
    eq: __raw_contour_eq,
    assign: __raw_contour_assign,
    typename_str_hash_u64: 16427054487712554671,
    typename_str: "RawContour",
};
type Vec__RawContour<'eval> = Vec<crate::raw_contour::RawContour<'eval>>;

// Vec__RawContour
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_raw_contour_clone(data: *mut ()) -> *mut () {
    Box::<Vec__RawContour>::into_raw(Box::new((*(data as *mut Vec__RawContour)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_raw_contour_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Vec__RawContour))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_raw_contour_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Vec__RawContour) == *(other as *const () as *const Vec__RawContour)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_raw_contour_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vec__RawContour>(&__VEC_RAW_CONTOUR_VTABLE) = registers[1].downcast_move(&__VEC_RAW_CONTOUR_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VEC_RAW_CONTOUR_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __vec_raw_contour_clone,
    drop: __vec_raw_contour_drop,
    eq: __vec_raw_contour_eq,
    assign: __vec_raw_contour_assign,
    typename_str_hash_u64: 4886731136539949416,
    typename_str: "Vec__RawContour",
};
type Vec__ConnectedComponent = Vec<crate::connected_component::ConnectedComponent>;

// Vec__ConnectedComponent
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_connected_component_clone(data: *mut ()) -> *mut () {
    Box::<Vec__ConnectedComponent>::into_raw(Box::new((*(data as *mut Vec__ConnectedComponent)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_connected_component_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Vec__ConnectedComponent))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_connected_component_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Vec__ConnectedComponent) == *(other as *const () as *const Vec__ConnectedComponent)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_connected_component_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vec__ConnectedComponent>(&__VEC_CONNECTED_COMPONENT_VTABLE) = registers[1].downcast_move(&__VEC_CONNECTED_COMPONENT_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VEC_CONNECTED_COMPONENT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __vec_connected_component_clone,
    drop: __vec_connected_component_drop,
    eq: __vec_connected_component_eq,
    assign: __vec_connected_component_assign,
    typename_str_hash_u64: 16072456320675099692,
    typename_str: "Vec__ConnectedComponent",
};
type Point2d = crate::geom2d::Point2d;

// Point2d
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __point_2_d_clone(data: *mut ()) -> *mut () {
    Box::<Point2d>::into_raw(Box::new((*(data as *mut Point2d)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __point_2_d_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Point2d))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __point_2_d_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Point2d) == *(other as *const () as *const Point2d)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __point_2_d_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Point2d>(&__POINT_2_D_VTABLE) = registers[1].downcast_move(&__POINT_2_D_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __POINT_2_D_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __point_2_d_clone,
    drop: __point_2_d_drop,
    eq: __point_2_d_eq,
    assign: __point_2_d_assign,
    typename_str_hash_u64: 7235593814690561581,
    typename_str: "Point2d",
};
type Vec__Point2d = Vec<crate::geom2d::Point2d>;

// Vec__Point2d
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_point_2_d_clone(data: *mut ()) -> *mut () {
    Box::<Vec__Point2d>::into_raw(Box::new((*(data as *mut Vec__Point2d)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_point_2_d_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Vec__Point2d))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_point_2_d_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Vec__Point2d) == *(other as *const () as *const Vec__Point2d)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_point_2_d_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vec__Point2d>(&__VEC_POINT_2_D_VTABLE) = registers[1].downcast_move(&__VEC_POINT_2_D_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VEC_POINT_2_D_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __vec_point_2_d_clone,
    drop: __vec_point_2_d_drop,
    eq: __vec_point_2_d_eq,
    assign: __vec_point_2_d_assign,
    typename_str_hash_u64: 13732815865654048798,
    typename_str: "Vec__Point2d",
};
type LineSegmentSketch<'eval> = crate::line_segment_sketch::LineSegmentSketch<'eval>;

// LineSegmentSketch
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_sketch_clone(data: *mut ()) -> *mut () {
    Box::<LineSegmentSketch>::into_raw(Box::new((*(data as *mut LineSegmentSketch)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_sketch_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut LineSegmentSketch))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_sketch_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const LineSegmentSketch) == *(other as *const () as *const LineSegmentSketch)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_sketch_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<LineSegmentSketch>(&__LINE_SEGMENT_SKETCH_VTABLE) = registers[1].downcast_move(&__LINE_SEGMENT_SKETCH_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __LINE_SEGMENT_SKETCH_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __line_segment_sketch_clone,
    drop: __line_segment_sketch_drop,
    eq: __line_segment_sketch_eq,
    assign: __line_segment_sketch_assign,
    typename_str_hash_u64: 6800712277405564928,
    typename_str: "LineSegmentSketch",
};
type Direction = crate::raw_contour::Direction;

// Direction
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __direction_clone(data: *mut ()) -> *mut () {
    Box::<Direction>::into_raw(Box::new((*(data as *mut Direction)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __direction_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Direction))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __direction_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Direction) == *(other as *const () as *const Direction)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __direction_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Direction>(&__DIRECTION_VTABLE) = registers[1].downcast_move(&__DIRECTION_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __DIRECTION_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __direction_clone,
    drop: __direction_drop,
    eq: __direction_eq,
    assign: __direction_assign,
    typename_str_hash_u64: 552646181455499180,
    typename_str: "Direction",
};
type StreakCache = crate::raw_contour::StreakCache;

// StreakCache
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __streak_cache_clone(data: *mut ()) -> *mut () {
    Box::<StreakCache>::into_raw(Box::new((*(data as *mut StreakCache)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __streak_cache_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut StreakCache))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __streak_cache_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const StreakCache) == *(other as *const () as *const StreakCache)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __streak_cache_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<StreakCache>(&__STREAK_CACHE_VTABLE) = registers[1].downcast_move(&__STREAK_CACHE_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __STREAK_CACHE_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __streak_cache_clone,
    drop: __streak_cache_drop,
    eq: __streak_cache_eq,
    assign: __streak_cache_assign,
    typename_str_hash_u64: 1217416088575417618,
    typename_str: "StreakCache",
};
type Vector2d = crate::geom2d::Vector2d;

// Vector2d
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vector_2_d_clone(data: *mut ()) -> *mut () {
    Box::<Vector2d>::into_raw(Box::new((*(data as *mut Vector2d)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vector_2_d_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Vector2d))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vector_2_d_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Vector2d) == *(other as *const () as *const Vector2d)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vector_2_d_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vector2d>(&__VECTOR_2_D_VTABLE) = registers[1].downcast_move(&__VECTOR_2_D_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VECTOR_2_D_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __vector_2_d_clone,
    drop: __vector_2_d_drop,
    eq: __vector_2_d_eq,
    assign: __vector_2_d_assign,
    typename_str_hash_u64: 6199847519887484943,
    typename_str: "Vector2d",
};
type ConcaveComponent<'eval> =
    crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>;

// ConcaveComponent
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __concave_component_clone(data: *mut ()) -> *mut () {
    Box::<ConcaveComponent>::into_raw(Box::new((*(data as *mut ConcaveComponent)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __concave_component_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut ConcaveComponent))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __concave_component_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const ConcaveComponent) == *(other as *const () as *const ConcaveComponent)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __concave_component_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<ConcaveComponent>(&__CONCAVE_COMPONENT_VTABLE) = registers[1].downcast_move(&__CONCAVE_COMPONENT_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __CONCAVE_COMPONENT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __concave_component_clone,
    drop: __concave_component_drop,
    eq: __concave_component_eq,
    assign: __concave_component_assign,
    typename_str_hash_u64: 4043962232733124493,
    typename_str: "ConcaveComponent",
};
type LineSegmentStroke<'eval> = crate::line_segment_sketch::LineSegmentStroke<'eval>;

// LineSegmentStroke
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_stroke_clone(data: *mut ()) -> *mut () {
    Box::<LineSegmentStroke>::into_raw(Box::new((*(data as *mut LineSegmentStroke)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_stroke_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut LineSegmentStroke))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_stroke_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const LineSegmentStroke) == *(other as *const () as *const LineSegmentStroke)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_stroke_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<LineSegmentStroke>(&__LINE_SEGMENT_STROKE_VTABLE) = registers[1].downcast_move(&__LINE_SEGMENT_STROKE_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __LINE_SEGMENT_STROKE_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __line_segment_stroke_clone,
    drop: __line_segment_stroke_drop,
    eq: __line_segment_stroke_eq,
    assign: __line_segment_stroke_assign,
    typename_str_hash_u64: 6281179995675231489,
    typename_str: "LineSegmentStroke",
};
type CyclicSlice__LineSegmentStroke<'eval> =
    __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegmentStroke<'eval>>;

// CyclicSlice__LineSegmentStroke
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __cyclic_slice_line_segment_stroke_clone(data: *mut ()) -> *mut () {
    Box::<CyclicSlice__LineSegmentStroke>::into_raw(Box::new((*(data as *mut CyclicSlice__LineSegmentStroke)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __cyclic_slice_line_segment_stroke_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut CyclicSlice__LineSegmentStroke))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __cyclic_slice_line_segment_stroke_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const CyclicSlice__LineSegmentStroke) == *(other as *const () as *const CyclicSlice__LineSegmentStroke)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __cyclic_slice_line_segment_stroke_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<CyclicSlice__LineSegmentStroke>(&__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE) = registers[1].downcast_move(&__CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __CYCLIC_SLICE_LINE_SEGMENT_STROKE_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __cyclic_slice_line_segment_stroke_clone,
    drop: __cyclic_slice_line_segment_stroke_drop,
    eq: __cyclic_slice_line_segment_stroke_eq,
    assign: __cyclic_slice_line_segment_stroke_assign,
    typename_str_hash_u64: 1827233183231989048,
    typename_str: "CyclicSlice__LineSegmentStroke",
};
type Vec__ConcaveComponent<'eval> =
    Vec<crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>>;

// Vec__ConcaveComponent
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_concave_component_clone(data: *mut ()) -> *mut () {
    Box::<Vec__ConcaveComponent>::into_raw(Box::new((*(data as *mut Vec__ConcaveComponent)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_concave_component_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Vec__ConcaveComponent))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_concave_component_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Vec__ConcaveComponent) == *(other as *const () as *const Vec__ConcaveComponent)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_concave_component_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vec__ConcaveComponent>(&__VEC_CONCAVE_COMPONENT_VTABLE) = registers[1].downcast_move(&__VEC_CONCAVE_COMPONENT_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VEC_CONCAVE_COMPONENT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __vec_concave_component_clone,
    drop: __vec_concave_component_drop,
    eq: __vec_concave_component_eq,
    assign: __vec_concave_component_assign,
    typename_str_hash_u64: 17127300658855120924,
    typename_str: "Vec__ConcaveComponent",
};
type ConvexCompoent<'eval> = crate::line_segment_sketch::convex_component::ConvexCompoent<'eval>;

// ConvexCompoent
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __convex_compoent_clone(data: *mut ()) -> *mut () {
    Box::<ConvexCompoent>::into_raw(Box::new((*(data as *mut ConvexCompoent)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __convex_compoent_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut ConvexCompoent))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __convex_compoent_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const ConvexCompoent) == *(other as *const () as *const ConvexCompoent)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __convex_compoent_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<ConvexCompoent>(&__CONVEX_COMPOENT_VTABLE) = registers[1].downcast_move(&__CONVEX_COMPOENT_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __CONVEX_COMPOENT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __convex_compoent_clone,
    drop: __convex_compoent_drop,
    eq: __convex_compoent_eq,
    assign: __convex_compoent_assign,
    typename_str_hash_u64: 13420018735853642728,
    typename_str: "ConvexCompoent",
};
type LineSegment = crate::line_segment_sketch::line_segment::LineSegment;

// LineSegment
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_clone(data: *mut ()) -> *mut () {
    Box::<LineSegment>::into_raw(Box::new((*(data as *mut LineSegment)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut LineSegment))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const LineSegment) == *(other as *const () as *const LineSegment)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __line_segment_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<LineSegment>(&__LINE_SEGMENT_VTABLE) = registers[1].downcast_move(&__LINE_SEGMENT_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __LINE_SEGMENT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __line_segment_clone,
    drop: __line_segment_drop,
    eq: __line_segment_eq,
    assign: __line_segment_assign,
    typename_str_hash_u64: 2513494300754753971,
    typename_str: "LineSegment",
};
type CyclicSlice__Point2d<'eval> = __std::slice::CyclicSlice<'eval, crate::geom2d::Point2d>;

// CyclicSlice__Point2d
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __cyclic_slice_point_2_d_clone(data: *mut ()) -> *mut () {
    Box::<CyclicSlice__Point2d>::into_raw(Box::new((*(data as *mut CyclicSlice__Point2d)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __cyclic_slice_point_2_d_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut CyclicSlice__Point2d))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __cyclic_slice_point_2_d_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const CyclicSlice__Point2d) == *(other as *const () as *const CyclicSlice__Point2d)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __cyclic_slice_point_2_d_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<CyclicSlice__Point2d>(&__CYCLIC_SLICE_POINT_2_D_VTABLE) = registers[1].downcast_move(&__CYCLIC_SLICE_POINT_2_D_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __CYCLIC_SLICE_POINT_2_D_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __cyclic_slice_point_2_d_clone,
    drop: __cyclic_slice_point_2_d_drop,
    eq: __cyclic_slice_point_2_d_eq,
    assign: __cyclic_slice_point_2_d_assign,
    typename_str_hash_u64: 17025899022928918821,
    typename_str: "CyclicSlice__Point2d",
};
type Vec__LineSegmentStroke<'eval> = Vec<crate::line_segment_sketch::LineSegmentStroke<'eval>>;

// Vec__LineSegmentStroke
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_line_segment_stroke_clone(data: *mut ()) -> *mut () {
    Box::<Vec__LineSegmentStroke>::into_raw(Box::new((*(data as *mut Vec__LineSegmentStroke)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_line_segment_stroke_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Vec__LineSegmentStroke))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_line_segment_stroke_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Vec__LineSegmentStroke) == *(other as *const () as *const Vec__LineSegmentStroke)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_line_segment_stroke_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vec__LineSegmentStroke>(&__VEC_LINE_SEGMENT_STROKE_VTABLE) = registers[1].downcast_move(&__VEC_LINE_SEGMENT_STROKE_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VEC_LINE_SEGMENT_STROKE_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __vec_line_segment_stroke_clone,
    drop: __vec_line_segment_stroke_drop,
    eq: __vec_line_segment_stroke_eq,
    assign: __vec_line_segment_stroke_assign,
    typename_str_hash_u64: 6123776351078547476,
    typename_str: "Vec__LineSegmentStroke",
};
type FermiMatchResult<'eval> = crate::fermi::FermiMatchResult<'eval>;

// FermiMatchResult
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __fermi_match_result_clone(data: *mut ()) -> *mut () {
    Box::<FermiMatchResult>::into_raw(Box::new((*(data as *mut FermiMatchResult)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __fermi_match_result_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut FermiMatchResult))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __fermi_match_result_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const FermiMatchResult) == *(other as *const () as *const FermiMatchResult)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __fermi_match_result_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<FermiMatchResult>(&__FERMI_MATCH_RESULT_VTABLE) = registers[1].downcast_move(&__FERMI_MATCH_RESULT_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __FERMI_MATCH_RESULT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __fermi_match_result_clone,
    drop: __fermi_match_result_drop,
    eq: __fermi_match_result_eq,
    assign: __fermi_match_result_assign,
    typename_str_hash_u64: 15105436207856257768,
    typename_str: "FermiMatchResult",
};
type Vec__Option__Ref__ConcaveComponent<'eval> =
    Vec<Option<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>>>;

// Vec__Option__Ref__ConcaveComponent
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_option_ref_concave_component_clone(data: *mut ()) -> *mut () {
    Box::<Vec__Option__Ref__ConcaveComponent>::into_raw(Box::new((*(data as *mut Vec__Option__Ref__ConcaveComponent)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_option_ref_concave_component_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Vec__Option__Ref__ConcaveComponent))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_option_ref_concave_component_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Vec__Option__Ref__ConcaveComponent) == *(other as *const () as *const Vec__Option__Ref__ConcaveComponent)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_option_ref_concave_component_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vec__Option__Ref__ConcaveComponent>(&__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE) = registers[1].downcast_move(&__VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VEC_OPTION_REF_CONCAVE_COMPONENT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __vec_option_ref_concave_component_clone,
    drop: __vec_option_ref_concave_component_drop,
    eq: __vec_option_ref_concave_component_eq,
    assign: __vec_option_ref_concave_component_assign,
    typename_str_hash_u64: 8896686783611719876,
    typename_str: "Vec__Option__Ref__ConcaveComponent",
};
type Vec__Ref__ConcaveComponent<'eval> =
    Vec<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>>;

// Vec__Ref__ConcaveComponent
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_ref_concave_component_clone(data: *mut ()) -> *mut () {
    Box::<Vec__Ref__ConcaveComponent>::into_raw(Box::new((*(data as *mut Vec__Ref__ConcaveComponent)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_ref_concave_component_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Vec__Ref__ConcaveComponent))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_ref_concave_component_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Vec__Ref__ConcaveComponent) == *(other as *const () as *const Vec__Ref__ConcaveComponent)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_ref_concave_component_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vec__Ref__ConcaveComponent>(&__VEC_REF_CONCAVE_COMPONENT_VTABLE) = registers[1].downcast_move(&__VEC_REF_CONCAVE_COMPONENT_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VEC_REF_CONCAVE_COMPONENT_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __vec_ref_concave_component_clone,
    drop: __vec_ref_concave_component_drop,
    eq: __vec_ref_concave_component_eq,
    assign: __vec_ref_concave_component_assign,
    typename_str_hash_u64: 12868255181803251158,
    typename_str: "Vec__Ref__ConcaveComponent",
};
type ThickFp__Ref__ConcaveComponent_Option__f32<'eval> = ThickFp<
    fn(
        &'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>,
    ) -> Option<f32>,
>;

// ThickFp__Ref__ConcaveComponent_Option__f32
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __thick_fp_ref_concave_component_option_f_32_clone(data: *mut ()) -> *mut () {
    Box::<ThickFp__Ref__ConcaveComponent_Option__f32>::into_raw(Box::new((*(data as *mut ThickFp__Ref__ConcaveComponent_Option__f32)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __thick_fp_ref_concave_component_option_f_32_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut ThickFp__Ref__ConcaveComponent_Option__f32))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __thick_fp_ref_concave_component_option_f_32_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const ThickFp__Ref__ConcaveComponent_Option__f32) == *(other as *const () as *const ThickFp__Ref__ConcaveComponent_Option__f32)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __thick_fp_ref_concave_component_option_f_32_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<ThickFp__Ref__ConcaveComponent_Option__f32>(&__THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE) = registers[1].downcast_move(&__THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __thick_fp_ref_concave_component_option_f_32_clone,
    drop: __thick_fp_ref_concave_component_option_f_32_drop,
    eq: __thick_fp_ref_concave_component_option_f_32_eq,
    assign: __thick_fp_ref_concave_component_option_f_32_assign,
    typename_str_hash_u64: 2410464468364455782,
    typename_str: "ThickFp__Ref__ConcaveComponent_Option__f32",
};
type Vec__ThickFp__Ref__ConcaveComponent_Option__f32<'eval> = Vec<
    ThickFp<
        fn(
            &'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>,
        ) -> Option<f32>,
    >,
>;

// Vec__ThickFp__Ref__ConcaveComponent_Option__f32
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_thick_fp_ref_concave_component_option_f_32_clone(data: *mut ()) -> *mut () {
    Box::<Vec__ThickFp__Ref__ConcaveComponent_Option__f32>::into_raw(Box::new((*(data as *mut Vec__ThickFp__Ref__ConcaveComponent_Option__f32)).clone())) as *mut ()
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_thick_fp_ref_concave_component_option_f_32_drop(data: *mut ()) {
    drop(Box::from_raw(data as *mut Vec__ThickFp__Ref__ConcaveComponent_Option__f32))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_thick_fp_ref_concave_component_option_f_32_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const Vec__ThickFp__Ref__ConcaveComponent_Option__f32) == *(other as *const () as *const Vec__ThickFp__Ref__ConcaveComponent_Option__f32)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_thick_fp_ref_concave_component_option_f_32_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vec__ThickFp__Ref__ConcaveComponent_Option__f32>(&__VEC_THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE) = registers[1].downcast_move(&__VEC_THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VEC_THICK_FP_REF_CONCAVE_COMPONENT_OPTION_F_32_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __vec_thick_fp_ref_concave_component_option_f_32_clone,
    drop: __vec_thick_fp_ref_concave_component_option_f_32_drop,
    eq: __vec_thick_fp_ref_concave_component_option_f_32_eq,
    assign: __vec_thick_fp_ref_concave_component_option_f_32_assign,
    typename_str_hash_u64: 14858446301517109270,
    typename_str: "Vec__ThickFp__Ref__ConcaveComponent_Option__f32",
};
