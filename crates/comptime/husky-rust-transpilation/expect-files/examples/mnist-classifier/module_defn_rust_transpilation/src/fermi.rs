use crate::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct FermiMatchResult {
    pub matches: Vec<Option<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>>,
    pub others: Vec<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>,
}

impl FermiMatchResult {
    pub fn __constructor(matches: Vec<Option<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>>, others: Vec<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>) -> Self {
        Self{
            matches,
            others,
        }
    }
}

#[rustfmt::skip]
pub fn fermi_match(concave_components: Leash<Vec<crate::line_segment_sketch::concave_component::ConcaveComponent>>, templates: &Vec<fn(Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32>>) -> crate::fermi::FermiMatchResult {
    let mut others = <Vec<crate::line_segment_sketch::concave_component::ConcaveComponent>>::collect_leashes(concave_components);
    let mut matches: Vec<Option<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>> = vec![];
    for i in 0..templates.ilen() {
        let template = templates[i as usize];
        matches.push(others.pop_with_largest_opt_f32(template))
    }
    return crate::fermi::FermiMatchResult::__constructor(matches, others);
}

#[rustfmt::skip]
impl crate::fermi::FermiMatchResult {
    #[ad_hoc_devsoul_dependency::memo(ingredient_index = 21)]
    pub fn norm(&'static self) -> f32 {
        let mut norm: f32 = 0.0f32;
        for i in 0..__self.deleash().others.ilen() {
            norm = norm.max(<crate::line_segment_sketch::concave_component::ConcaveComponent>::norm(__self.deleash().others[i as usize]))
        }
        return norm;
    }

    #[ad_hoc_devsoul_dependency::memo(ingredient_index = 22)]
    pub fn rel_norm(&'static self) -> f32 {
        let mut norm: f32 = 0.0f32;
        for i in 0..__self.deleash().others.ilen() {
            norm = norm.max(<crate::line_segment_sketch::concave_component::ConcaveComponent>::rel_norm(__self.deleash().others[i as usize]))
        }
        return norm;
    }

    #[ad_hoc_devsoul_dependency::memo(ingredient_index = 23)]
    pub fn angle_change_norm(&'static self) -> f32 {
        let mut norm: f32 = 0.0f32;
        for i in 0..__self.deleash().others.ilen() {
            norm = norm.max(<crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(__self.deleash().others[i as usize]).abs())
        }
        return norm;
    }
}