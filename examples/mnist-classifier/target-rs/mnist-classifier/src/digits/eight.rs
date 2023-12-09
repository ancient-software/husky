use super::*;

#[ad_hoc_task_dependency::val_item(ingredient_index = 41, return_ref)]
pub fn upper_mouth_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![big_mouth])
}

#[ad_hoc_task_dependency::val_item(ingredient_index = 42)]
pub fn is_eight() -> OneVsAll {
    require!(let none = is_one());
    require!(let none = is_six());
    require!(let none = is_zero());
    require!(let none = is_seven());
    let upper_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
    if let none = major_connected_component().eff_holes().matches[1 as usize] {
        if let none = major_connected_component().eff_holes().matches[0 as usize] {
            require!(false);
        }
        require!(false);
    }
    OneVsAll::Yes
}

pub fn big_mouth(cc: Leash<ConcaveComponent>) -> Option<f32> {
    if cc.relative_bounding_box().ymax() > 0.5f32 {
        require!(cc.strokes.first().unwrap().start.x.into_inner() > cc.strokes.first().unwrap().end.x.into_inner());
    }
    Some(cc.relative_bounding_box().ymax())
}