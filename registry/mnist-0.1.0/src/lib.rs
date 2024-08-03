pub mod dataset;
pub mod input_id;
pub mod task;

use self::input_id::*;
use dataset::MNIST_DATASET;
use husky_core::*;
use husky_devsoul_interface::ugly::*;
use husky_linket_impl::standard::ugly::*;
use husky_standard_devsoul_interface::{label::IsLabel, ugly::*};

#[husky_standard_value::value_conversion]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MnistLabel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl IsLabel for MnistLabel {
    fn label() -> Self {
        MNIST_DATASET.label(input_id())
    }
}

impl From<u8> for MnistLabel {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[husky_standard_value::value_conversion]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryImage28([u32; 30]);

impl BinaryImage28 {
    pub fn new_zeros() -> Self {
        Self::default()
    }

    pub fn pixel(&self, i: usize, j: usize) -> bool {
        let row = self.0[i + 1];
        (row & (1 << (29 - j))) != 0
    }
}

impl husky_core::visual::Visualize for BinaryImage28 {
    fn visualize(&self, sct: &mut __VisualSynchrotron) -> __Visual {
        __Visual::new_binary_image(
            4,
            28,
            28,
            self.0[2..]
                .iter()
                .map(|u| {
                    unsafe { std::mem::transmute::<_, [u8; 4]>(u << 2) }
                        .into_iter()
                        .rev()
                })
                .flatten()
                .collect(),
            sct,
        )
    }
}

impl std::ops::Index<usize> for BinaryImage28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryImage28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[husky_standard_value::value_conversion]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryGrid28([u32; 31]);

impl BinaryGrid28 {
    pub fn new_zeros() -> Self {
        Self::default()
    }
}

impl std::ops::Index<usize> for BinaryGrid28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryGrid28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl BinaryGrid28 {}

#[allow(non_upper_case_globals)]
pub static mut __INPUT__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[allow(non_snake_case)]
pub fn INPUT() -> Leash<BinaryImage28> {
    MNIST_DATASET.input_leashed(input_id())
}

pub struct INPUT {}

impl __IsStaticVar<__StaticVarId> for INPUT {
    fn item_path_id_interface() -> __ItemPathIdInterface {
        unsafe { __INPUT__ITEM_PATH_ID_INTERFACE.expect("__INPUT__ITEM_PATH_ID_INTERFACE") }
    }

    fn get_id() -> __StaticVarId {
        input_id().index().into()
    }

    fn set_id(id: __StaticVarId) {
        set_input_id(id.into())
    }

    fn ids() -> impl Iterator<Item = __StaticVarId> {
        input_ids().map(Into::into)
    }
}

impl INPUT {
    pub fn item_path_id_interface() -> __ItemPathIdInterface {
        unsafe { __INPUT__ITEM_PATH_ID_INTERFACE.expect("__INPUT__ITEM_PATH_ID_INTERFACE") }
    }

    pub fn set_up_for_testing(index: usize) {
        // todo: check range!
        set_input_id(MnistInputId::from_index(index))
    }
}

#[allow(non_upper_case_globals)]
pub static mut __TASK__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

// ad hoc
#[allow(non_snake_case)]
pub fn TASK() {}

pub struct TASK {}

impl TASK {
    pub fn set_up_for_testing(index: usize) {
        todo!()
    }

    pub fn get_id() -> __StaticVarId {
        todo!()
    }

    pub fn set_id(id: __StaticVarId) {
        todo!()
    }

    pub fn ids() -> impl Iterator<Item = __StaticVarId> {
        [].into_iter()
    }
}
