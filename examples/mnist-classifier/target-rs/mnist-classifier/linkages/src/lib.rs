use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use mnist_classifier::*;

#[rustfmt::skip]
linkage_impls![
    fn_linkage_impl!(mnist_classifier::main),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution::__constructor),
    struct_field_linkage_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution, row_start),
    struct_field_linkage_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution, row_end),
    struct_field_linkage_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution, upper_mass),
    struct_field_linkage_impl!(mnist_classifier::connected_component::ConnectedComponentDistribution, lower_mass),
    fn_linkage_impl!(mnist_classifier::connected_component::EffHoles::__constructor),
    struct_field_linkage_impl!(mnist_classifier::connected_component::EffHoles, matches),
    fn_linkage_impl!(mnist_classifier::connected_component::hole_tmpl),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent::__constructor),
    struct_field_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent, mask),
    fn_linkage_impl!(mnist_classifier::connected_component::horizontal_extend),
    fn_linkage_impl!(mnist_classifier::connected_component::find_connected_components),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent::raw_contours),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent::eff_holes),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent::max_hole_ilen),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent::max_row_span),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent::row_span_sum),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent::distribution),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent::upper_mass),
    fn_linkage_impl!(mnist_classifier::connected_component::ConnectedComponent::lower_mass),
    fn_linkage_impl!(<mnist_classifier::connected_component::ConnectedComponent>::top_k_row_span_sum),
    fn_linkage_impl!(<mnist_classifier::connected_component::ConnectedComponent>::top_k_row_right_mass_sum),
    fn_linkage_impl!(mnist_classifier::raw_contour::RawContour::__constructor),
    struct_field_linkage_impl!(mnist_classifier::raw_contour::RawContour, cc),
    struct_field_linkage_impl!(mnist_classifier::raw_contour::RawContour, points),
    enum_u8_presenter_linkage_impl!(mnist_classifier::raw_contour::Direction),
    fn_linkage_impl!(mnist_classifier::raw_contour::get_pixel_pair),
    fn_linkage_impl!(mnist_classifier::raw_contour::get_pixel_to_the_left),
    fn_linkage_impl!(mnist_classifier::raw_contour::get_pixel_to_the_right),
    fn_linkage_impl!(mnist_classifier::raw_contour::get_inward_direction),
    fn_linkage_impl!(mnist_classifier::raw_contour::get_angle_change),
    fn_linkage_impl!(mnist_classifier::raw_contour::get_outward_direction),
    fn_linkage_impl!(mnist_classifier::raw_contour::StreakCache::__constructor),
    struct_field_linkage_impl!(mnist_classifier::raw_contour::StreakCache, prev1),
    struct_field_linkage_impl!(mnist_classifier::raw_contour::StreakCache, prev2),
    fn_linkage_impl!(mnist_classifier::raw_contour::get_concave_middle_point),
    fn_linkage_impl!(mnist_classifier::raw_contour::find_raw_contours),
    fn_linkage_impl!(mnist_classifier::raw_contour::RawContour::line_segment_sketch),
    fn_linkage_impl!(mnist_classifier::raw_contour::RawContour::bounding_box),
    fn_linkage_impl!(mnist_classifier::raw_contour::RawContour::relative_bounding_box),
    fn_linkage_impl!(mnist_classifier::raw_contour::RawContour::contour_len),
    fn_linkage_impl!(<mnist_classifier::raw_contour::RawContour>::displacement),
    fn_linkage_impl!(mnist_classifier::geom2d::Point2d::__constructor),
    struct_field_linkage_impl!(mnist_classifier::geom2d::Point2d, x),
    struct_field_linkage_impl!(mnist_classifier::geom2d::Point2d, y),
    fn_linkage_impl!(mnist_classifier::geom2d::RelativePoint2d::__constructor),
    struct_field_linkage_impl!(mnist_classifier::geom2d::RelativePoint2d, x),
    struct_field_linkage_impl!(mnist_classifier::geom2d::RelativePoint2d, y),
    fn_linkage_impl!(mnist_classifier::geom2d::Vector2d::__constructor),
    struct_field_linkage_impl!(mnist_classifier::geom2d::Vector2d, x),
    struct_field_linkage_impl!(mnist_classifier::geom2d::Vector2d, y),
    fn_linkage_impl!(mnist_classifier::geom2d::ClosedRange::__constructor),
    struct_field_linkage_impl!(mnist_classifier::geom2d::ClosedRange, min),
    struct_field_linkage_impl!(mnist_classifier::geom2d::ClosedRange, max),
    fn_linkage_impl!(mnist_classifier::geom2d::BoundingBox::__constructor),
    struct_field_linkage_impl!(mnist_classifier::geom2d::BoundingBox, xrange),
    struct_field_linkage_impl!(mnist_classifier::geom2d::BoundingBox, yrange),
    fn_linkage_impl!(mnist_classifier::geom2d::RelativeBoundingBox::__constructor),
    struct_field_linkage_impl!(mnist_classifier::geom2d::RelativeBoundingBox, xrange),
    struct_field_linkage_impl!(mnist_classifier::geom2d::RelativeBoundingBox, yrange),
    fn_linkage_impl!(<mnist_classifier::geom2d::Point2d>::from_i_shift28),
    fn_linkage_impl!(<mnist_classifier::geom2d::Point2d>::vector),
    fn_linkage_impl!(<mnist_classifier::geom2d::Point2d>::to),
    fn_linkage_impl!(<mnist_classifier::geom2d::Point2d>::norm),
    fn_linkage_impl!(<mnist_classifier::geom2d::Point2d>::dist),
    fn_linkage_impl!(<mnist_classifier::geom2d::Vector2d>::point),
    fn_linkage_impl!(<mnist_classifier::geom2d::Vector2d>::to),
    fn_linkage_impl!(<mnist_classifier::geom2d::Vector2d>::norm),
    fn_linkage_impl!(<mnist_classifier::geom2d::Vector2d>::dot),
    fn_linkage_impl!(<mnist_classifier::geom2d::Vector2d>::cross),
    fn_linkage_impl!(<mnist_classifier::geom2d::Vector2d>::angle),
    fn_linkage_impl!(<mnist_classifier::geom2d::Vector2d>::rotation_direction_to),
    fn_linkage_impl!(<mnist_classifier::geom2d::Vector2d>::angle_to),
    fn_linkage_impl!(<mnist_classifier::geom2d::ClosedRange>::relative_range),
    fn_linkage_impl!(<mnist_classifier::geom2d::ClosedRange>::relative_point),
    fn_linkage_impl!(<mnist_classifier::geom2d::BoundingBox>::relative_bounding_box),
    fn_linkage_impl!(<mnist_classifier::geom2d::BoundingBox>::relative_point),
    fn_linkage_impl!(<mnist_classifier::geom2d::BoundingBox>::xmin),
    fn_linkage_impl!(<mnist_classifier::geom2d::BoundingBox>::xmax),
    fn_linkage_impl!(<mnist_classifier::geom2d::BoundingBox>::ymin),
    fn_linkage_impl!(<mnist_classifier::geom2d::BoundingBox>::ymax),
    fn_linkage_impl!(<mnist_classifier::geom2d::RelativeBoundingBox>::xmin),
    fn_linkage_impl!(<mnist_classifier::geom2d::RelativeBoundingBox>::xmax),
    fn_linkage_impl!(<mnist_classifier::geom2d::RelativeBoundingBox>::ymin),
    fn_linkage_impl!(<mnist_classifier::geom2d::RelativeBoundingBox>::ymax),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::LineSegmentStroke::__constructor),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::LineSegmentStroke, points),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::LineSegmentStroke, start),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::LineSegmentStroke, end),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch::__constructor),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch, contour),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch, strokes),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::go_right),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::go_left),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::extend_end),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::extend_start),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::find_line_segments),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::LineSegmentStroke>::new),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::LineSegmentStroke>::displacement),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch::concave_components),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::LineSegmentSketch::bounding_box),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::LineSegmentSketch>::new),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::__constructor),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, line_segment_sketch),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, strokes),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::find_concave_components),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::norm),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::rel_norm),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::hausdorff_norm),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::angle_change),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::bounding_box),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::relative_bounding_box),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::line_segment),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::start),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::end),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::displacement),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::start_tangent),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::end_tangent),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::convex_component::ConvexComponent::__constructor),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::convex_component::ConvexComponent, line_segment_sketch),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::convex_component::ConvexComponent, line_segments),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::convexity::is_convex),
    fn_linkage_impl!(mnist_classifier::line_segment_sketch::line_segment::LineSegment::__constructor),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::line_segment::LineSegment, start),
    struct_field_linkage_impl!(mnist_classifier::line_segment_sketch::line_segment::LineSegment, end),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::line_segment::LineSegment>::displacement),
    fn_linkage_impl!(<mnist_classifier::line_segment_sketch::line_segment::LineSegment>::dist_to_point),
    fn_linkage_impl!(mnist_classifier::fermi::FermiMatchResult::__constructor),
    struct_field_linkage_impl!(mnist_classifier::fermi::FermiMatchResult, matches),
    struct_field_linkage_impl!(mnist_classifier::fermi::FermiMatchResult, others),
    fn_linkage_impl!(mnist_classifier::fermi::fermi_match),
    fn_linkage_impl!(mnist_classifier::fermi::FermiMatchResult::norm),
    fn_linkage_impl!(mnist_classifier::fermi::FermiMatchResult::rel_norm),
    fn_linkage_impl!(mnist_classifier::fermi::FermiMatchResult::angle_change_norm),
    fn_linkage_impl!(mnist_classifier::digits::zero::open_one_match),
    fn_linkage_impl!(mnist_classifier::digits::zero::almost_closed),
    fn_linkage_impl!(mnist_classifier::digits::zero::is_zero),
    fn_linkage_impl!(mnist_classifier::digits::one::one_fermi_match),
    fn_linkage_impl!(mnist_classifier::digits::one::is_one),
    fn_linkage_impl!(mnist_classifier::digits::one::upmost),
    fn_linkage_impl!(mnist_classifier::digits::one::downmost),
    fn_linkage_impl!(mnist_classifier::digits::one::hat),
    fn_linkage_impl!(mnist_classifier::digits::six::six_match),
    fn_linkage_impl!(mnist_classifier::digits::six::six_match_refined1),
    fn_linkage_impl!(mnist_classifier::digits::six::is_six),
    fn_linkage_impl!(mnist_classifier::digits::six::upmost),
    fn_linkage_impl!(mnist_classifier::digits::six::bottom1),
    fn_linkage_impl!(mnist_classifier::digits::three::three_fermi_match),
    fn_linkage_impl!(mnist_classifier::digits::three::is_three),
    fn_linkage_impl!(mnist_classifier::digits::three::uparc),
    fn_linkage_impl!(mnist_classifier::digits::three::downarc),
    fn_linkage_impl!(mnist_classifier::digits::three::back),
    fn_linkage_impl!(mnist_classifier::digits::four::left_components),
    fn_linkage_impl!(mnist_classifier::digits::four::left_coordinate_max),
    fn_linkage_impl!(mnist_classifier::digits::four::components_max_downwards),
    fn_linkage_impl!(mnist_classifier::digits::four::components_max_heights),
    fn_linkage_impl!(mnist_classifier::digits::four::is_four),
    fn_linkage_impl!(mnist_classifier::digits::four::displacement_downwards),
    fn_linkage_impl!(mnist_classifier::digits::four::cc_box_heights),
    fn_linkage_impl!(mnist_classifier::digits::five::is_five),
    fn_linkage_impl!(mnist_classifier::digits::seven::simple_seven_match),
    fn_linkage_impl!(mnist_classifier::digits::seven::simple_leftdown_pattern),
    fn_linkage_impl!(mnist_classifier::digits::seven::special_seven_match),
    fn_linkage_impl!(mnist_classifier::digits::seven::leftupcc_pattern),
    fn_linkage_impl!(mnist_classifier::digits::seven::leftdowncc_pattern),
    fn_linkage_impl!(mnist_classifier::digits::seven::is_seven),
    fn_linkage_impl!(mnist_classifier::digits::eight::upper_mouth_match),
    fn_linkage_impl!(mnist_classifier::digits::eight::is_eight),
    fn_linkage_impl!(mnist_classifier::digits::eight::big_mouth),
    fn_linkage_impl!(mnist_classifier::digits::nine::nine_match),
    fn_linkage_impl!(mnist_classifier::digits::nine::nine_match_refine),
    fn_linkage_impl!(mnist_classifier::digits::nine::is_nine),
    fn_linkage_impl!(mnist_classifier::digits::nine::downmost),
    fn_linkage_impl!(mnist_classifier::digits::nine::big_cc),
    fn_linkage_impl!(mnist_classifier::digits::two::two_match),
    fn_linkage_impl!(mnist_classifier::digits::two::left_cc_pattern),
    fn_linkage_impl!(mnist_classifier::digits::two::right_cc_pattern),
    fn_linkage_impl!(mnist_classifier::digits::two::down_cc_pattern),
    fn_linkage_impl!(mnist_classifier::digits::two::is_two),
    fn_linkage_impl!(mnist_classifier::major::connected_components),
    fn_linkage_impl!(mnist_classifier::major::major_connected_component),
    fn_linkage_impl!(mnist_classifier::major::ignored_connected_components_row_span_sum_sum),
    fn_linkage_impl!(mnist_classifier::major::major_raw_contours),
    fn_linkage_impl!(mnist_classifier::major::major_raw_contour),
    fn_linkage_impl!(mnist_classifier::major::major_line_segment_sketch),
    fn_linkage_impl!(mnist_classifier::major::major_concave_components),
    fn_linkage_impl!(<malamute::Class<mnist::MnistLabel> as Unveil<malamute::OneVsAll>>::unveil),
    unveil_fn_linkage_impl!(<malamute::Class<mnist::MnistLabel> as Unveil<malamute::OneVsAll>>::unveil),
    fn_linkage_impl!(|v: Vec<mnist_classifier::connected_component::ConnectedComponent>|v),
    fn_linkage_impl!(<mnist::BinaryImage28 as Clone>::clone),
    fn_linkage_impl!(<Vec<mnist_classifier::connected_component::ConnectedComponent>>::push),
    fn_linkage_impl!(<Vec<mnist_classifier::raw_contour::RawContour>>::collect_leashes),
    fn_linkage_impl!(|v: Vec<Option<Leash<mnist_classifier::raw_contour::RawContour>>>|v),
    fn_linkage_impl!(<Vec<Leash<mnist_classifier::raw_contour::RawContour>>>::pop_with_largest_opt_f32),
    fn_linkage_impl!(<Vec<Option<Leash<mnist_classifier::raw_contour::RawContour>>>>::push),
    fn_linkage_impl!(<Vec<mnist_classifier::raw_contour::RawContour>>::ilen),
    fn_linkage_impl!(<Vec<mnist_classifier::geom2d::Point2d>>::ilen),
    fn_linkage_impl!(|v: Vec<mnist_classifier::raw_contour::RawContour>|v),
    fn_linkage_impl!(|v: Vec<mnist_classifier::geom2d::Point2d>|v),
    fn_linkage_impl!(<[mnist_classifier::geom2d::Point2d]>::last),
    fn_linkage_impl!(<[mnist_classifier::geom2d::Point2d]>::last_mut),
    fn_linkage_impl!(<Vec<mnist_classifier::geom2d::Point2d>>::push),
    fn_linkage_impl!(<Vec<mnist_classifier::geom2d::Point2d>>::pop),
    fn_linkage_impl!(<Vec<mnist_classifier::raw_contour::RawContour>>::push),
    fn_linkage_impl!(<CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::first),
    fn_linkage_impl!(<mnist_classifier::geom2d::Point2d as Clone>::clone),
    fn_linkage_impl!(<CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::last),
    fn_linkage_impl!(|v: Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>|v),
    fn_linkage_impl!(<Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::ilen),
    fn_linkage_impl!(<[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::last),
    fn_linkage_impl!(<[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::last_mut),
    fn_linkage_impl!(<CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::start),
    fn_linkage_impl!(<Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::pop),
    fn_linkage_impl!(<CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::end),
    fn_linkage_impl!(<Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::push),
    fn_linkage_impl!(<[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::first),
    fn_linkage_impl!(<[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::first_mut),
    fn_linkage_impl!(<Vec<mnist_classifier::geom2d::Point2d>>::cyclic_slice_leashed),
    fn_linkage_impl!(|v: Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>|v),
    fn_linkage_impl!(<Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::cyclic_slice_leashed),
    fn_linkage_impl!(<Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>::push),
    fn_linkage_impl!(<CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::first),
    fn_linkage_impl!(<CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::start),
    fn_linkage_impl!(<CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::end),
    fn_linkage_impl!(<CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::last),
    fn_linkage_impl!(<Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>::collect_leashes),
    fn_linkage_impl!(|v: Vec<Option<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>|v),
    fn_linkage_impl!(<Vec<fn (Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32>>>::ilen),
    fn_linkage_impl!(<Vec<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>::pop_with_largest_opt_f32),
    fn_linkage_impl!(<Vec<Option<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>>::push),
    fn_linkage_impl!(<Vec<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>::ilen),
    fn_linkage_impl!(|v: Vec<fn (Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32>>|v),
    fn_linkage_impl!(<Vec<mnist_classifier::connected_component::ConnectedComponent>>::ilen),
    gn_linkage_impl!(malamute::narrow_down::<mnist::MnistLabel>),
    ty_default_linkage_impl!(malamute::OneVsAll),
    fn_linkage_impl!(<Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>::ilen),
    fn_linkage_impl!(Option::Some::<f32>),
    fn_linkage_impl!(malamute::Class::Known::<mnist::MnistLabel>),
    fn_linkage_impl!(husky_core::ops::ControlFlow::Break::<malamute::Class<mnist::MnistLabel>, ()>),
    fn_linkage_impl!(husky_core::ops::ControlFlow::Continue::<malamute::Class<mnist::MnistLabel>, ()>),
];