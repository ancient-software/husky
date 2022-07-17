#![allow(warnings)]
pub mod __init__;
use __husky_root::*;


// ad hoc
fn __input<'a, 'eval:'a>(__ctx: &'a __EvalContext<'eval>) -> &'a domains::ml::datasets::cv::mnist::BinaryImage28 {
    unsafe { __evaluator(__ctx) }
        .eval_input
        .any_ref()
        .__downcast_ref()
}

pub(crate) mod connected_component;
pub(crate) mod raw_contour;
pub(crate) mod geom2d;
pub(crate) mod line_segment_sketch;
