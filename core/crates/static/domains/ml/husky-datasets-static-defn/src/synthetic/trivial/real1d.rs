use crate::*;
use entity_kind::RoutineKind;
use husky_liason_semantics::OutputLiason;
use husky_trace_protocol::SampleId;
use std::sync::Arc;
use vm::{__Linkage, __LinkageFp, transfer_linkage};
use xrng::XRng;

pub const REAL_1D_MODULE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "real1d",
    items: &[DATASET1_MODULE_DEFN, DATASET2_SCOPE_DATA],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::__static_dev_src!(),
};

pub const DATASET1_MODULE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "dataset1",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicTemplate::None,
        output_ty: "Dataset<f32, i32>",
        output_liason: OutputLiason::Transfer,
        linkage: transfer_linkage!(
            |_, _| unsafe{(__Register::new_box(dataset1(), &__DATASET_VTABLE))},
         some   dataset1
        )
        .into(),
    },
    dev_src: husky_dev_utils::__static_dev_src!(),
};

pub const DATASET2_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "dataset2",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicTemplate::None,
        output_ty: "Dataset<f32, i32>",
        output_liason: OutputLiason::Transfer,
        linkage: transfer_linkage!(|_, _| unsafe {(__Register::new_box(
            dataset2(), &__DATASET_VTABLE
        ))},
        some   dataset2)
        .into(),
    },
    dev_src: husky_dev_utils::__static_dev_src!(),
};

pub fn gen_sample1<'eval>(seed: u64, sample_id: SampleId) -> LabeledData<'eval> {
    let mut xrng = XRng::new(((seed + (sample_id.0 as u64)) >> 32) & ((sample_id.0 as u64) << 32));
    if xrng.with_probability(0.5) {
        LabeledData {
            input: 1.0f32.to_register(),
            label: 1.into(),
            sample_id: sample_id,
        }
    } else {
        LabeledData {
            input: (-1.0f32).to_register(),
            label: 1.into(),
            sample_id: sample_id,
        }
    }
}

pub fn gen_sample2<'eval>(seed: u64, sample_id: SampleId) -> LabeledData<'eval> {
    let mut xrng = XRng::new(((seed + (sample_id.0 as u64)) >> 32) & ((sample_id.0 as u64) << 32));
    if xrng.with_probability(0.5) {
        LabeledData {
            input: 1.0f32.to_register(),
            label: 1.into(),
            sample_id: sample_id,
        }
    } else {
        LabeledData {
            input: (-1.0f32).to_register(),
            label: 1.into(),
            sample_id: sample_id,
        }
    }
}

pub fn dataset1<'eval>() -> Dataset<'eval> {
    Dataset::new(SimpleSyntheticDataset::new(1223418012u64, gen_sample1))
}

pub fn dataset2<'eval>() -> Dataset<'eval> {
    Dataset::new(SimpleSyntheticDataset::new(1213148012u64, gen_sample2))
}
