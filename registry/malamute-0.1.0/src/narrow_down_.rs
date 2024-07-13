mod flag;

use self::flag::*;
use crate::*;
use ad_hoc_devsoul_dependency::ki_control_flow::KiControlFlow;
use smallvec::SmallVec;

#[allow(warnings, non_snake_case)]
pub struct narrow_down<Label>(std::marker::PhantomData<Label>);

#[derive(Debug, PartialEq, Eq, Clone, __Serialize)]
// #[value_conversion]
pub struct NarrowDownInternal<Label> {
    label0: Label,
    flag_ranges: SmallVec<[Option<FlagRange>; 4]>,
}
impl<Label> __WeakStatic for NarrowDownInternal<Label>
where
    Label: __WeakStatic<Static = Label>
        + __Static<Frozen = Label>
        + __Frozen<Static = Label>
        + __Serialize,
{
    type Static = NarrowDownInternal<Label>;
    unsafe fn into_static(self) -> Self::Static {
        self
    }
}
impl<Label> __Static for NarrowDownInternal<Label>
where
    Label: __Static<Frozen = Label> + __Frozen<Static = Label> + __Serialize,
{
    type Frozen = NarrowDownInternal<Label>;
    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }
    fn serialize_to_value(&self) -> __JsonValue {
        __to_json_value(self).unwrap()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        // ad hoc
        __Visual::Void
    }
}
impl<Label> __Frozen for NarrowDownInternal<Label>
where
    Label: __Frozen<Static = Label> + __Static<Frozen = Label> + __Serialize,
{
    type Static = NarrowDownInternal<<Label as __Frozen>::Static>;
    type Stand = ();
    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        todo!()
    }
}
impl<Label> __FromValue for NarrowDownInternal<Label>
where
    Label: __WeakStatic<Static = Label> + __Static,
{
    fn from_value_aux(value: __Value, _value_stands: Option<&mut __ValueStands>) -> Self {
        value.into_owned()
    }
}
impl<Label> __IntoValue for NarrowDownInternal<Label>
where
    Label: __WeakStatic<Static = Label>
        + __Static<Frozen = Label>
        + __Frozen<Static = Label>
        + __Serialize,
{
    fn into_value(self) -> __Value {
        __Value::from_owned(self)
    }
}

impl<Label> __IsGnItem for narrow_down<Label>
where
    Label: IsLabel,
{
    type LinkageImpl = __LinkageImpl;

    fn generic_pedestal(generic_pedestal: __Pedestal) -> __Pedestal {
        __Pedestal::Generic
    }

    type ValueAtGenericPedestal = NarrowDownInternal<Label>;

    fn train(
        ki_domain_repr: __ValDomainReprInterface,
        val_argument_reprs: &[__KiArgumentReprInterface],
    ) -> __KiControlFlow<Self::ValueAtGenericPedestal> {
        debug_assert_eq!(val_argument_reprs.len(), 3);
        let __KiArgumentReprInterface::Variadic(ref features) = val_argument_reprs[0] else {
            unreachable!()
        };
        let __KiArgumentReprInterface::Keyed(skip) = val_argument_reprs[1] else {
            unreachable!()
        };
        let skip: i32 = match skip {
            Some(skip) => __eval_ki_repr_interface(skip, None)?,
            None => 5,
        };
        let __KiArgumentReprInterface::RuntimeConstants(ref runtime_constants) =
            val_argument_reprs[2]
        else {
            unreachable!()
        };
        debug_assert_eq!(runtime_constants.len(), 1);
        let label: Label = __eval_val_runtime_constant(runtime_constants[0]);
        let fvf = FlagVectorField::from_features(ki_domain_repr, features, label)?;
        // let fvf = FlagVectorField::from_registers(&opds[0], &opds[2..], &labels)?;
        // let ntrim = opds[1].value().downcast_i32();
        __KiControlFlow::Continue(NarrowDownInternal {
            label0: fvf.label0(),
            flag_ranges: fvf.flag_ranges(skip, 0.1),
        })
    }

    type EvalOutput = OneVsAllResult;

    fn eval(
        val_argument_reprs: &[__KiArgumentReprInterface],
        value_at_generic_pedestal: &Self::ValueAtGenericPedestal,
    ) -> OneVsAllResult {
        let __KiArgumentReprInterface::Variadic(ref features) = val_argument_reprs[0] else {
            unreachable!()
        };
        debug_assert_eq!(features.len(), value_at_generic_pedestal.flag_ranges.len());
        for (&feature, flag_range) in
            std::iter::zip(features, &value_at_generic_pedestal.flag_ranges)
        {
            let Some(flag_range) = flag_range else {
                continue;
            };
            let v: f32 = match __eval_ki_repr_interface(feature, None) {
                KiControlFlow::Continue(v) => v,
                KiControlFlow::LoopContinue => todo!(),
                KiControlFlow::LoopExit(_) => todo!(),
                KiControlFlow::Return(_) => todo!(),
                KiControlFlow::Undefined => todo!(),
                KiControlFlow::Throw(_) => todo!(),
            };
            let v = NotNan::new(v).unwrap();
            let apply_result = flag_range.apply(v);
            if !apply_result.within_false_range() && apply_result.within_true_range() {
                return OneVsAllResult::ConfidentYes;
            } else if apply_result.within_false_range() && !apply_result.within_true_range() {
                // corresponds to `return Some(None)` in Rust
                return OneVsAllResult::ConfidentNo;
            }
        }
        OneVsAllResult::Unconfident
    }
}
