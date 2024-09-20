#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};

#[rustfmt::skip]
linket_impls![
    val_linket_impl!(mnist_classifier::main, mnist_classifier::__main__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution, mnist_classifier::connected_component::ConnectedComponentDistribution, row_start, row_end, upper_mass, lower_mass),
    struct_field_linket_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution, copyable row_start),
    struct_field_linket_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution, copyable row_end),
    struct_field_linket_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution, copyable upper_mass),
    struct_field_linket_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution, copyable lower_mass),
    fn_linket_impl!(mnist_classifier::connected_component::EffHoles::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::connected_component::EffHoles, mnist_classifier::connected_component::EffHoles, matches),
    struct_field_linket_impl!(mnist_classifier::connected_component::EffHoles, other matches),
    fn_linket_impl!(mnist_classifier::connected_component::hole_tmpl),
    fn_linket_impl!(mnist_classifier::connected_component::ConnectedComponent::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::connected_component::ConnectedComponent, mnist_classifier::connected_component::ConnectedComponent, mask),
    struct_field_linket_impl!(mnist_classifier::connected_component::ConnectedComponent, other mask),
    fn_linket_impl!(mnist_classifier::connected_component::horizontal_extend),
    fn_linket_impl!(mnist_classifier::connected_component::find_connected_components),
    memo_linket_impl!(mnist_classifier::connected_component::ConnectedComponent::raw_contours, mnist_classifier::connected_component::__ConnectedComponent__raw_contours__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::connected_component::ConnectedComponent::eff_holes, mnist_classifier::connected_component::__ConnectedComponent__eff_holes__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::connected_component::ConnectedComponent::max_hole_ilen, mnist_classifier::connected_component::__ConnectedComponent__max_hole_ilen__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::connected_component::ConnectedComponent::max_row_span, mnist_classifier::connected_component::__ConnectedComponent__max_row_span__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::connected_component::ConnectedComponent::row_span_sum, mnist_classifier::connected_component::__ConnectedComponent__row_span_sum__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::connected_component::ConnectedComponent::distribution, mnist_classifier::connected_component::__ConnectedComponent__distribution__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::connected_component::ConnectedComponent::upper_mass, mnist_classifier::connected_component::__ConnectedComponent__upper_mass__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::connected_component::ConnectedComponent::lower_mass, mnist_classifier::connected_component::__ConnectedComponent__lower_mass__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(<mnist_classifier::connected_component::ConnectedComponent>::top_k_row_span_sum),
    fn_linket_impl!(<mnist_classifier::connected_component::ConnectedComponent>::top_k_row_right_mass_sum),
    fn_linket_impl!(mnist_classifier::raw_contour::RawContour::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::raw_contour::RawContour, mnist_classifier::raw_contour::RawContour, cc, points),
    struct_field_linket_impl!(mnist_classifier::raw_contour::RawContour, copyable cc),
    struct_field_linket_impl!(mnist_classifier::raw_contour::RawContour, other points),
    enum_index_presenter_linket_impl!(mnist_classifier::raw_contour::Direction),
    enum_variant_constructor_linket_impl!(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction::Up),
    enum_variant_discriminator_linket_impl!(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction::Up),
    enum_variant_constructor_linket_impl!(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction::Left),
    enum_variant_discriminator_linket_impl!(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction::Left),
    enum_variant_constructor_linket_impl!(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction::Down),
    enum_variant_discriminator_linket_impl!(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction::Down),
    enum_variant_constructor_linket_impl!(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction::Right),
    enum_variant_discriminator_linket_impl!(mnist_classifier::raw_contour::Direction, mnist_classifier::raw_contour::Direction::Right),
    fn_linket_impl!(mnist_classifier::raw_contour::get_pixel_pair),
    fn_linket_impl!(mnist_classifier::raw_contour::get_pixel_to_the_left),
    fn_linket_impl!(mnist_classifier::raw_contour::get_pixel_to_the_right),
    fn_linket_impl!(mnist_classifier::raw_contour::get_inward_direction),
    fn_linket_impl!(mnist_classifier::raw_contour::get_angle_change),
    fn_linket_impl!(mnist_classifier::raw_contour::get_outward_direction),
    fn_linket_impl!(mnist_classifier::raw_contour::StreakCache::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::raw_contour::StreakCache, mnist_classifier::raw_contour::StreakCache, prev1, prev2),
    struct_field_linket_impl!(mnist_classifier::raw_contour::StreakCache, copyable prev1),
    struct_field_linket_impl!(mnist_classifier::raw_contour::StreakCache, copyable prev2),
    fn_linket_impl!(mnist_classifier::raw_contour::get_concave_middle_point),
    fn_linket_impl!(mnist_classifier::raw_contour::find_raw_contours),
    memo_linket_impl!(mnist_classifier::raw_contour::RawContour::line_segment_sketch, mnist_classifier::raw_contour::__RawContour__line_segment_sketch__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::raw_contour::RawContour::bounding_box, mnist_classifier::raw_contour::__RawContour__bounding_box__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::raw_contour::RawContour::relative_bounding_box, mnist_classifier::raw_contour::__RawContour__relative_bounding_box__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::raw_contour::RawContour::contour_len, mnist_classifier::raw_contour::__RawContour__contour_len__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(<mnist_classifier::raw_contour::RawContour>::displacement),
    fn_linket_impl!(mnist_classifier::geom2d::Point2d::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::geom2d::Point2d, mnist_classifier::geom2d::Point2d, x, y),
    struct_field_linket_impl!(mnist_classifier::geom2d::Point2d, copyable x),
    struct_field_linket_impl!(mnist_classifier::geom2d::Point2d, copyable y),
    fn_linket_impl!(mnist_classifier::geom2d::RelativePoint2d::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::geom2d::RelativePoint2d, mnist_classifier::geom2d::RelativePoint2d, x, y),
    struct_field_linket_impl!(mnist_classifier::geom2d::RelativePoint2d, copyable x),
    struct_field_linket_impl!(mnist_classifier::geom2d::RelativePoint2d, copyable y),
    fn_linket_impl!(mnist_classifier::geom2d::Vector2d::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::geom2d::Vector2d, mnist_classifier::geom2d::Vector2d, x, y),
    struct_field_linket_impl!(mnist_classifier::geom2d::Vector2d, copyable x),
    struct_field_linket_impl!(mnist_classifier::geom2d::Vector2d, copyable y),
    fn_linket_impl!(mnist_classifier::geom2d::ClosedRange::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::geom2d::ClosedRange, mnist_classifier::geom2d::ClosedRange, min, max),
    struct_field_linket_impl!(mnist_classifier::geom2d::ClosedRange, copyable min),
    struct_field_linket_impl!(mnist_classifier::geom2d::ClosedRange, copyable max),
    fn_linket_impl!(mnist_classifier::geom2d::BoundingBox::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::geom2d::BoundingBox, mnist_classifier::geom2d::BoundingBox, xrange, yrange),
    struct_field_linket_impl!(mnist_classifier::geom2d::BoundingBox, other xrange),
    struct_field_linket_impl!(mnist_classifier::geom2d::BoundingBox, other yrange),
    fn_linket_impl!(mnist_classifier::geom2d::RelativeBoundingBox::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::geom2d::RelativeBoundingBox, mnist_classifier::geom2d::RelativeBoundingBox, xrange, yrange),
    struct_field_linket_impl!(mnist_classifier::geom2d::RelativeBoundingBox, other xrange),
    struct_field_linket_impl!(mnist_classifier::geom2d::RelativeBoundingBox, other yrange),
    fn_linket_impl!(<mnist_classifier::geom2d::Point2d>::from_i_shift28),
    fn_linket_impl!(<mnist_classifier::geom2d::Point2d>::vector),
    fn_linket_impl!(<mnist_classifier::geom2d::Point2d>::to),
    fn_linket_impl!(<mnist_classifier::geom2d::Point2d>::norm),
    fn_linket_impl!(<mnist_classifier::geom2d::Point2d>::dist),
    fn_linket_impl!(<mnist_classifier::geom2d::Vector2d>::point),
    fn_linket_impl!(<mnist_classifier::geom2d::Vector2d>::to),
    fn_linket_impl!(<mnist_classifier::geom2d::Vector2d>::norm),
    fn_linket_impl!(<mnist_classifier::geom2d::Vector2d>::dot),
    fn_linket_impl!(<mnist_classifier::geom2d::Vector2d>::cross),
    fn_linket_impl!(<mnist_classifier::geom2d::Vector2d>::angle),
    fn_linket_impl!(<mnist_classifier::geom2d::Vector2d>::rotation_direction_to),
    fn_linket_impl!(<mnist_classifier::geom2d::Vector2d>::angle_to),
    fn_linket_impl!(<mnist_classifier::geom2d::ClosedRange>::relative_range),
    fn_linket_impl!(<mnist_classifier::geom2d::ClosedRange>::relative_point),
    fn_linket_impl!(<mnist_classifier::geom2d::BoundingBox>::relative_bounding_box),
    fn_linket_impl!(<mnist_classifier::geom2d::BoundingBox>::relative_point),
    fn_linket_impl!(<mnist_classifier::geom2d::BoundingBox>::xmin),
    fn_linket_impl!(<mnist_classifier::geom2d::BoundingBox>::xmax),
    fn_linket_impl!(<mnist_classifier::geom2d::BoundingBox>::ymin),
    fn_linket_impl!(<mnist_classifier::geom2d::BoundingBox>::ymax),
    fn_linket_impl!(<mnist_classifier::geom2d::RelativeBoundingBox>::xmin),
    fn_linket_impl!(<mnist_classifier::geom2d::RelativeBoundingBox>::xmax),
    fn_linket_impl!(<mnist_classifier::geom2d::RelativeBoundingBox>::ymin),
    fn_linket_impl!(<mnist_classifier::geom2d::RelativeBoundingBox>::ymax),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentStroke::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentStroke, mnist_classifier::line_segment_sketch::LineSegmentStroke, points, start, end),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentStroke, copyable points),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentStroke, other start),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentStroke, other end),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch, mnist_classifier::line_segment_sketch::LineSegmentSketch, contour, strokes),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch, copyable contour),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch, other strokes),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::go_right),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::go_left),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::extend_end),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::extend_start),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::find_line_segments),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::LineSegmentStroke>::new),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::LineSegmentStroke>::displacement),
    memo_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch::concave_components, mnist_classifier::line_segment_sketch::__LineSegmentSketch__concave_components__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch::bounding_box, mnist_classifier::line_segment_sketch::__LineSegmentSketch__bounding_box__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::LineSegmentSketch>::new),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, line_segment_sketch, strokes),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, copyable line_segment_sketch),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, copyable strokes),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::find_concave_components),
    memo_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::norm, mnist_classifier::line_segment_sketch::concave_component::__ConcaveComponent__norm__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::rel_norm, mnist_classifier::line_segment_sketch::concave_component::__ConcaveComponent__rel_norm__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::hausdorff_norm, mnist_classifier::line_segment_sketch::concave_component::__ConcaveComponent__hausdorff_norm__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::angle_change, mnist_classifier::line_segment_sketch::concave_component::__ConcaveComponent__angle_change__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::bounding_box, mnist_classifier::line_segment_sketch::concave_component::__ConcaveComponent__bounding_box__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::relative_bounding_box, mnist_classifier::line_segment_sketch::concave_component::__ConcaveComponent__relative_bounding_box__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::line_segment),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::start),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::end),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::displacement),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::start_tangent),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::end_tangent),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::convex_component::ConvexComponent::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::line_segment_sketch::convex_component::ConvexComponent, mnist_classifier::line_segment_sketch::convex_component::ConvexComponent, line_segment_sketch, line_segments),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::convex_component::ConvexComponent, copyable line_segment_sketch),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::convex_component::ConvexComponent, copyable line_segments),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::convexity::is_convex),
    fn_linket_impl!(mnist_classifier::line_segment_sketch::line_segment::LineSegment::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::line_segment_sketch::line_segment::LineSegment, mnist_classifier::line_segment_sketch::line_segment::LineSegment, start, end),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::line_segment::LineSegment, other start),
    struct_field_linket_impl!(mnist_classifier::line_segment_sketch::line_segment::LineSegment, other end),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::line_segment::LineSegment>::displacement),
    fn_linket_impl!(<mnist_classifier::line_segment_sketch::line_segment::LineSegment>::dist_to_point),
    fn_linket_impl!(mnist_classifier::fermi::FermiMatchResult::__constructor),
    struct_destructor_linket_impl!(mnist_classifier::fermi::FermiMatchResult, mnist_classifier::fermi::FermiMatchResult, matches, others),
    struct_field_linket_impl!(mnist_classifier::fermi::FermiMatchResult, other matches),
    struct_field_linket_impl!(mnist_classifier::fermi::FermiMatchResult, other others),
    fn_linket_impl!(mnist_classifier::fermi::fermi_match),
    memo_linket_impl!(mnist_classifier::fermi::FermiMatchResult::norm, mnist_classifier::fermi::__FermiMatchResult__norm__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::fermi::FermiMatchResult::rel_norm, mnist_classifier::fermi::__FermiMatchResult__rel_norm__ITEM_PATH_ID_INTERFACE),
    memo_linket_impl!(mnist_classifier::fermi::FermiMatchResult::angle_change_norm, mnist_classifier::fermi::__FermiMatchResult__angle_change_norm__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::zero::open_one_match, mnist_classifier::digits::zero::__open_one_match__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::zero::almost_closed),
    val_linket_impl!(mnist_classifier::digits::zero::is_zero, mnist_classifier::digits::zero::__is_zero__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::one::one_fermi_match, mnist_classifier::digits::one::__one_fermi_match__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::one::is_one, mnist_classifier::digits::one::__is_one__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::one::upmost),
    fn_linket_impl!(mnist_classifier::digits::one::downmost),
    fn_linket_impl!(mnist_classifier::digits::one::hat),
    val_linket_impl!(mnist_classifier::digits::six::six_match, mnist_classifier::digits::six::__six_match__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::six::six_match_refined1, mnist_classifier::digits::six::__six_match_refined1__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::six::is_six, mnist_classifier::digits::six::__is_six__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::six::upmost),
    fn_linket_impl!(mnist_classifier::digits::six::bottom1),
    val_linket_impl!(mnist_classifier::digits::three::three_fermi_match, mnist_classifier::digits::three::__three_fermi_match__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::three::is_three, mnist_classifier::digits::three::__is_three__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::three::uparc),
    fn_linket_impl!(mnist_classifier::digits::three::downarc),
    fn_linket_impl!(mnist_classifier::digits::three::back),
    val_linket_impl!(mnist_classifier::digits::four::left_components, mnist_classifier::digits::four::__left_components__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::four::left_coordinate_max),
    val_linket_impl!(mnist_classifier::digits::four::components_max_downwards, mnist_classifier::digits::four::__components_max_downwards__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::four::components_max_heights, mnist_classifier::digits::four::__components_max_heights__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::four::is_four, mnist_classifier::digits::four::__is_four__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::four::displacement_downwards),
    fn_linket_impl!(mnist_classifier::digits::four::cc_box_heights),
    val_linket_impl!(mnist_classifier::digits::five::is_five, mnist_classifier::digits::five::__is_five__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::seven::simple_seven_match, mnist_classifier::digits::seven::__simple_seven_match__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::seven::simple_leftdown_pattern),
    val_linket_impl!(mnist_classifier::digits::seven::special_seven_match, mnist_classifier::digits::seven::__special_seven_match__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::seven::leftupcc_pattern),
    fn_linket_impl!(mnist_classifier::digits::seven::leftdowncc_pattern),
    val_linket_impl!(mnist_classifier::digits::seven::is_seven, mnist_classifier::digits::seven::__is_seven__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::eight::upper_mouth_match, mnist_classifier::digits::eight::__upper_mouth_match__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::eight::is_eight, mnist_classifier::digits::eight::__is_eight__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::eight::big_mouth),
    val_linket_impl!(mnist_classifier::digits::nine::nine_match, mnist_classifier::digits::nine::__nine_match__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::nine::nine_match_refine, mnist_classifier::digits::nine::__nine_match_refine__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::digits::nine::is_nine, mnist_classifier::digits::nine::__is_nine__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::nine::downmost),
    fn_linket_impl!(mnist_classifier::digits::nine::big_cc),
    val_linket_impl!(mnist_classifier::digits::two::two_match, mnist_classifier::digits::two::__two_match__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(mnist_classifier::digits::two::left_cc_pattern),
    fn_linket_impl!(mnist_classifier::digits::two::right_cc_pattern),
    fn_linket_impl!(mnist_classifier::digits::two::down_cc_pattern),
    val_linket_impl!(mnist_classifier::digits::two::is_two, mnist_classifier::digits::two::__is_two__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::major::connected_components, mnist_classifier::major::__connected_components__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::major::major_connected_component, mnist_classifier::major::__major_connected_component__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::major::ignored_connected_components_row_span_sum_sum, mnist_classifier::major::__ignored_connected_components_row_span_sum_sum__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::major::major_raw_contours, mnist_classifier::major::__major_raw_contours__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::major::major_raw_contour, mnist_classifier::major::__major_raw_contour__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::major::major_line_segment_sketch, mnist_classifier::major::__major_line_segment_sketch__ITEM_PATH_ID_INTERFACE),
    val_linket_impl!(mnist_classifier::major::major_concave_components, mnist_classifier::major::__major_concave_components__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(<malamute::Class<mnist::MnistLabel> as Unveil<malamute::OneVsAll>>::unveil),
    unveil_fn_linket_impl!(<malamute::Class<mnist::MnistLabel> as Unveil<malamute::OneVsAll>>::unveil),
    fn_linket_impl!(|v: Vec<mnist_classifier::connected_component::ConnectedComponent>|v),
    fn_linket_impl!(<mnist::BinaryImage28 as Clone>::clone),
    fn_linket_impl!(<Vec<mnist_classifier::connected_component::ConnectedComponent>>::push),
    fn_linket_impl!(<Vec<mnist_classifier::raw_contour::RawContour>>::collect_leashes),
    fn_linket_impl!(|v: Vec<Option<Leash<mnist_classifier::raw_contour::RawContour>>>|v),
    fn_linket_impl!(<Vec<Leash<mnist_classifier::raw_contour::RawContour>>>::pop_with_largest_opt_f32),
    fn_linket_impl!(<Vec<Option<Leash<mnist_classifier::raw_contour::RawContour>>>>::push),
    fn_linket_impl!(<Vec<mnist_classifier::raw_contour::RawContour>>::ilen),
    fn_linket_impl!(<Vec<mnist_classifier::geom2d::Point2d>>::ilen),
    fn_linket_impl!(|v: Vec<mnist_classifier::raw_contour::RawContour>|v),
    fn_linket_impl!(|v: Vec<mnist_classifier::geom2d::Point2d>|v),
    fn_linket_impl!(<[mnist_classifier::geom2d::Point2d]>::last),
    fn_linket_impl!(<[mnist_classifier::geom2d::Point2d]>::last_mut),
    fn_linket_impl!(<Vec<mnist_classifier::geom2d::Point2d>>::push),
    fn_linket_impl!(<Vec<mnist_classifier::geom2d::Point2d>>::pop),
    fn_linket_impl!(<Vec<mnist_classifier::raw_contour::RawContour>>::push),
    fn_linket_impl!(<CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::first),
    fn_linket_impl!(<mnist_classifier::geom2d::Point2d as Clone>::clone),
    fn_linket_impl!(<CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::last),
    fn_linket_impl!(|v: Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>|v),
    fn_linket_impl!(<Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::ilen),
    fn_linket_impl!(<[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::last),
    fn_linket_impl!(<[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::last_mut),
    fn_linket_impl!(<CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::start),
    fn_linket_impl!(<Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::pop),
    fn_linket_impl!(<CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::end),
    fn_linket_impl!(<Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::push),
    fn_linket_impl!(<[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::first),
    fn_linket_impl!(<[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::first_mut),
    fn_linket_impl!(<Vec<mnist_classifier::geom2d::Point2d>>::cyclic_slice_leashed),
    fn_linket_impl!(|v: Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>|v),
    fn_linket_impl!(<Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::cyclic_slice_leashed),
    fn_linket_impl!(<Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>::push),
    fn_linket_impl!(<CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::first),
    fn_linket_impl!(<CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::start),
    fn_linket_impl!(<CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::end),
    fn_linket_impl!(<CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::last),
    fn_linket_impl!(<Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>::collect_leashes),
    fn_linket_impl!(|v: Vec<Option<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>|v),
    fn_linket_impl!(<Vec<fn (Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32>>>::ilen),
    fn_linket_impl!(<Vec<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>::pop_with_largest_opt_f32),
    fn_linket_impl!(<Vec<Option<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>>::push),
    fn_linket_impl!(<Vec<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>::ilen),
    fn_linket_impl!(|v: Vec<fn (Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32>>|v),
    fn_linket_impl!(<Vec<mnist_classifier::connected_component::ConnectedComponent>>::ilen),
    gn_linket_impl!(malamute::narrow_down::<mnist::task::MnistTask, mnist::MnistLabel>),
    ty_default_linket_impl!(malamute::OneVsAll),
    fn_linket_impl!(<Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>::ilen),
    enum_variant_constructor_linket_impl!(Option<f32>, Option::Some, (v0)),
    enum_variant_discriminator_linket_impl!(Option<f32>, Option::Some, ()),
    enum_variant_destructor_linket_impl!(Option<f32>, Option::Some, (v0)),
    enum_variant_field_linket_impl!(Option<f32>, Option::Some, (copyable v0)),
    enum_variant_constructor_linket_impl!(Option<f32>, Option::None),
    enum_variant_discriminator_linket_impl!(Option<f32>, Option::None),
    enum_variant_constructor_linket_impl!(malamute::Class<mnist::MnistLabel>, malamute::Class::Known, (v0)),
    enum_variant_discriminator_linket_impl!(malamute::Class<mnist::MnistLabel>, malamute::Class::Known, ()),
    enum_variant_destructor_linket_impl!(malamute::Class<mnist::MnistLabel>, malamute::Class::Known, (v0)),
    enum_variant_field_linket_impl!(malamute::Class<mnist::MnistLabel>, malamute::Class::Known, (other v0)),
    enum_variant_constructor_linket_impl!(malamute::Class<mnist::MnistLabel>, malamute::Class::Unknown),
    enum_variant_discriminator_linket_impl!(malamute::Class<mnist::MnistLabel>, malamute::Class::Unknown),
    enum_variant_constructor_linket_impl!(husky_core::ops::ControlFlow<malamute::Class<mnist::MnistLabel>, ()>, husky_core::ops::ControlFlow::Continue, (v0)),
    enum_variant_discriminator_linket_impl!(husky_core::ops::ControlFlow<malamute::Class<mnist::MnistLabel>, ()>, husky_core::ops::ControlFlow::Continue, ()),
    enum_variant_destructor_linket_impl!(husky_core::ops::ControlFlow<malamute::Class<mnist::MnistLabel>, ()>, husky_core::ops::ControlFlow::Continue, (v0)),
    enum_variant_field_linket_impl!(husky_core::ops::ControlFlow<malamute::Class<mnist::MnistLabel>, ()>, husky_core::ops::ControlFlow::Continue, (copyable v0)),
    enum_variant_constructor_linket_impl!(husky_core::ops::ControlFlow<malamute::Class<mnist::MnistLabel>, ()>, husky_core::ops::ControlFlow::Break, (v0)),
    enum_variant_discriminator_linket_impl!(husky_core::ops::ControlFlow<malamute::Class<mnist::MnistLabel>, ()>, husky_core::ops::ControlFlow::Break, ()),
    enum_variant_destructor_linket_impl!(husky_core::ops::ControlFlow<malamute::Class<mnist::MnistLabel>, ()>, husky_core::ops::ControlFlow::Break, (v0)),
    enum_variant_field_linket_impl!(husky_core::ops::ControlFlow<malamute::Class<mnist::MnistLabel>, ()>, husky_core::ops::ControlFlow::Break, (other v0)),
    fn_linket_impl!(<i8>::abs),
    fn_linket_impl!(<i8>::max),
    fn_linket_impl!(<i8 as Add<i8>>::add),
    fn_linket_impl!(<i16>::abs),
    fn_linket_impl!(<i16>::max),
    fn_linket_impl!(<i16 as Add<i16>>::add),
    fn_linket_impl!(<i32>::abs),
    fn_linket_impl!(<i32>::max),
    fn_linket_impl!(<i32>::min),
    fn_linket_impl!(<i32 as Add<i32>>::add),
    fn_linket_impl!(<i64>::abs),
    fn_linket_impl!(<i64 as Add<i64>>::add),
    fn_linket_impl!(<i128>::abs),
    fn_linket_impl!(<i128 as Add<i128>>::add),
    fn_linket_impl!(<isize>::abs),
    fn_linket_impl!(<isize as Add<isize>>::add),
    fn_linket_impl!(<u8 as Add<u8>>::add),
    fn_linket_impl!(<u16 as Add<u16>>::add),
    fn_linket_impl!(<u32 as Add<u32>>::add),
    fn_linket_impl!(<u64 as Add<u64>>::add),
    fn_linket_impl!(<u128 as Add<u128>>::add),
    fn_linket_impl!(<usize as Add<usize>>::add),
    fn_linket_impl!(<f32>::abs),
    fn_linket_impl!(<f32>::sqrt),
    fn_linket_impl!(<f32>::max),
    fn_linket_impl!(<f32>::min),
    fn_linket_impl!(<f32>::sgnx),
    fn_linket_impl!(<f32>::cos),
    fn_linket_impl!(<f32>::sin),
    fn_linket_impl!(<f32>::acos),
    fn_linket_impl!(<f32 as Add<f32>>::add),
    fn_linket_impl!(<f64>::abs),
    fn_linket_impl!(<f64>::acos),
    fn_linket_impl!(<f64 as Add<f64>>::add),
    fn_linket_impl!(<u32>::last_bits),
    fn_linket_impl!(<u32>::ctz),
    fn_linket_impl!(<u32>::co),
    fn_linket_impl!(<u32>::span),
    fn_linket_impl!(<u32>::right_mass),
    static_var_linket_impl!(husky_core::task::TASK, husky_core::task::__TASK__ITEM_PATH_ID_INTERFACE),
    enum_index_presenter_linket_impl!(malamute::OneVsAll),
    enum_variant_constructor_linket_impl!(malamute::OneVsAll, malamute::OneVsAll::Yes),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAll, malamute::OneVsAll::Yes),
    enum_variant_constructor_linket_impl!(malamute::OneVsAll, malamute::OneVsAll::No),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAll, malamute::OneVsAll::No),
    enum_index_presenter_linket_impl!(malamute::OneVsAllResult),
    enum_variant_constructor_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentYes),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentYes),
    enum_variant_constructor_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentNo),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentNo),
    enum_variant_constructor_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::Unconfident),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::Unconfident),
    fn_linket_impl!(<malamute::OneVsAll as Default>::default),
    fn_linket_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    unveil_fn_linket_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    enum_variant_constructor_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, (v0)),
    enum_variant_discriminator_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, ()),
    enum_variant_destructor_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, (v0)),
    enum_variant_field_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, (copyable v0)),
    enum_variant_constructor_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, (v0)),
    enum_variant_discriminator_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, ()),
    enum_variant_destructor_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, (v0)),
    enum_variant_field_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, (other v0)),
    enum_index_presenter_linket_impl!(mnist::MnistLabel),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Zero),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Zero),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::One),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::One),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Two),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Two),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Three),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Three),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Four),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Four),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Five),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Five),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Six),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Six),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Seven),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Seven),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Eight),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Eight),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Nine),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Nine),
    static_var_linket_impl!(mnist::INPUT, mnist::__INPUT__ITEM_PATH_ID_INTERFACE),
    fn_linket_impl!(<mnist::BinaryImage28>::new_zeros),
    fn_linket_impl!(<mnist::BinaryGrid28>::new_zeros),
    fn_linket_impl!(<mnist::task::MnistTask>::new),
    static_var_linket_impl!(mnist::TASK, mnist::__TASK__ITEM_PATH_ID_INTERFACE),
];