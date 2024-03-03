#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use mnist::*;

#[rustfmt::skip]
linkage_impls![
    enum_u8_presenter_linkage_impl!(mnist::MnistLabel),
    fn_linkage_impl!(mnist::input),
    fn_linkage_impl!(<mnist::BinaryImage28>::new_zeros),
    fn_linkage_impl!(<mnist::BinaryGrid28>::new_zeros),
];