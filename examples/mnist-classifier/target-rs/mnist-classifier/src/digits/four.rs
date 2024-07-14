use super::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 33, return_ref)]
pub fn left_components() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![left_coordinate_max, left_coordinate_max])
}

#[rustfmt::skip]
pub fn left_coordinate_max(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    Some(cc.relative_bounding_box().xmax())
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 34, return_ref)]
pub fn components_max_downwards() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![displacement_downwards])
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 35, return_ref)]
pub fn components_max_heights() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![cc_box_heights])
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 36)]
pub fn is_four() -> malamute::OneVsAll {
    require!(let Some(_) = left_components().matches[0 as usize]);
    require!(let Some(_) = left_components().matches[1 as usize]);
    let eff_holes = &major_connected_component().eff_holes();
    require!(let Option::None = eff_holes.matches[1 as usize]);
    let down_match = components_max_downwards().matches[0 as usize];
    require!(let Some(_) = down_match);
    let down_match_dp_y = down_match.unwrap().displacement().y;
    let higher_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
    require!(higher_excess > 7.0f32);
    if let Option::None = eff_holes.matches[0 as usize] {
        require!(major_concave_components().ilen() >= 2);
        let four_match_refine_result = components_max_heights().matches[0 as usize];
        require!(let Some(_) = four_match_refine_result);
        require!(components_max_heights().norm() < 1.0f32);
        let higher_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
        let upper_arc = components_max_heights().matches[0 as usize];
        require!(let Some(_) = upper_arc);
        require!(upper_arc.unwrap().displacement().y > 0.0f32);
        require!(upper_arc.unwrap().angle_change() < -110.0f32);
        require!(components_max_heights().norm() < 9.0f32);
        let a = major_connected_component().top_k_row_right_mass_sum(3);
        require!(a < 22.0f32);
        require!(a > 9.0f32);
        return OneVsAll::Yes;
    }
    OneVsAll::Yes
}

#[rustfmt::skip]
pub fn displacement_downwards(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y < 0.0f32);
    Some(dp.y)
}

#[rustfmt::skip]
pub fn cc_box_heights(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y > 0.0f32);
    require!(cc.relative_bounding_box().ymin() > 0.4f32);
    Some(cc.relative_bounding_box().ymin())
}