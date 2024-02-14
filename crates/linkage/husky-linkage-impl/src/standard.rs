pub mod ugly;

pub use husky_standard_value::{
    frozen::ValueStands, value_conversion, FromValue, IntoValue, Value, ValueLeashTest,
};

use super::*;
use husky_decl_macro_utils::for_all_ritchie_tys;
use husky_task_interface::{val_repr::ValDomainReprInterface, DevEvalContext};
use husky_value_protocol::presentation::EnumU8ValuePresenter;

// ad hoc
pub type Error = ();

pub type StandardLinkageImplValControlFlow<C = Value, B = Value> =
    husky_task_interface::val_control_flow::ValControlFlow<C, B, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl<Pedestal>
where
    Pedestal: std::fmt::Debug + Copy + 'static,
{
    RitchieFn {
        /// it's the wrapper's responsibility to properly set ctx
        fn_wrapper: fn(
            DevEvalContext<LinkageImpl<Pedestal>>,
            &[ValArgumentReprInterface],
        ) -> StandardLinkageImplValControlFlow,
        fn_pointer: fn(),
    },
    StructField {
        struct_field_wrapper: fn(Value) -> Value,
    },
    RitchieUnveilFn {
        /// it's the wrapper's responsibility to properly set ctx
        fn_wrapper: fn(
            DevEvalContext<LinkageImpl<Pedestal>>,
            &[ValArgumentReprInterface],
        ) -> StandardLinkageImplValControlFlow,
        fn_pointer: fn(),
    },
    RitchieGn {
        generic_pedestal: fn(Pedestal) -> Pedestal,
        /// it's the wrapper's responsibility to properly set ctx to that with generic pedestal
        gn_generic_wrapper: fn(
            DevEvalContext<LinkageImpl<Pedestal>>,
            ValDomainReprInterface,
            &[ValArgumentReprInterface],
        ) -> StandardLinkageImplValControlFlow,
        /// no need to set ctx
        gn_specific_wrapper:
            fn(&[ValArgumentReprInterface], Value) -> StandardLinkageImplValControlFlow,
    },
    /// used to get the json value of an enum u8-represented given only the index
    EnumU8ValuePresenter { presenter: EnumU8ValuePresenter },
}

impl<Pedestal> IsLinkageImpl for LinkageImpl<Pedestal>
where
    Pedestal: std::fmt::Debug + Copy + 'static,
{
    type Pedestal = Pedestal;
    type Value = Value;
    type Error = Error;

    fn eval(
        self,
        val_repr: ValReprInterface,
        ctx: DevEvalContext<Self>,
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> StandardLinkageImplValControlFlow {
        match self {
            LinkageImpl::RitchieFn { fn_wrapper, .. } => fn_wrapper(ctx, val_argument_reprs),
            LinkageImpl::RitchieUnveilFn { fn_wrapper, .. } => fn_wrapper(ctx, val_argument_reprs),
            LinkageImpl::RitchieGn {
                generic_pedestal,
                gn_generic_wrapper,
                gn_specific_wrapper,
            } => {
                let value_at_generic_pedestal = ctx
                    .eval_val_repr_interface_at_generic_pedestal_with(
                        val_repr,
                        generic_pedestal,
                        gn_generic_wrapper,
                        val_argument_reprs,
                    )?;
                gn_specific_wrapper(val_argument_reprs, value_at_generic_pedestal)
            }
            LinkageImpl::StructField {
                struct_field_wrapper,
            } => {
                debug_assert_eq!(val_argument_reprs.len(), 1);
                let ValArgumentReprInterface::Simple(owner) = val_argument_reprs[0] else {
                    unreachable!()
                };
                let owner = ctx.eval_val_repr_interface(owner)?;
                StandardLinkageImplValControlFlow::Continue(struct_field_wrapper(owner))
            }
            LinkageImpl::EnumU8ValuePresenter { .. } => {
                unreachable!("this linkage is not meant to be evaluated like this")
            }
        }
    }

    fn enum_u8_value_presenter(self) -> EnumU8ValuePresenter {
        match self {
            LinkageImpl::EnumU8ValuePresenter { presenter } => presenter,
            _ => unreachable!(),
        }
    }
}

pub struct FnLinkageImplSource<Pedestal, T>(pub std::marker::PhantomData<Pedestal>, pub T);

for_all_ritchie_tys! {impl_is_fn_linkage_impl_source}

pub struct UnveilFnLinkageImplSource<Pedestal, T>(pub std::marker::PhantomData<Pedestal>, pub T);

for_all_ritchie_tys! {impl_is_unveil_fn_linkage_impl_source}

pub trait IsGnItem {
    type LinkageImpl: IsLinkageImpl;

    fn generic_pedestal(
        specific_pedestal: <Self::LinkageImpl as IsLinkageImpl>::Pedestal,
    ) -> <Self::LinkageImpl as IsLinkageImpl>::Pedestal;

    type ValueAtGenericPedestal;

    /// compute `generic_pedestal` here for efficiency
    fn train(
        val_domain_repr: ValDomainReprInterface,
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> LinkageImplValControlFlow<Self::LinkageImpl, Self::ValueAtGenericPedestal>;

    type EvalOutput;

    fn eval(
        val_argument_reprs: &[ValArgumentReprInterface],
        value_at_generic_pedestal: &Self::ValueAtGenericPedestal,
    ) -> Self::EvalOutput;
}

#[macro_export]
macro_rules! gn_linkage_impl {
    ($gn_item: ty) => {{
        /// generic_pedestal is a pedestal that is not closed (minimal)
        ///
        /// it's the counterpart of generic point in algebraic geometry
        fn gn_generic_wrapper(
            ctx: __DevEvalContext,
            val_domain_repr: __ValDomainReprInterface,
            val_argument_reprs: &[__ValArgumentReprInterface],
        ) -> __ValControlFlow {
            __with_dev_eval_context(ctx, || {
                __ValControlFlow::Continue(
                    __ValueLeashTest(<$gn_item as __IsGnItem>::train(
                        val_domain_repr,
                        val_argument_reprs,
                    )?)
                    .into_value(),
                )
            })
        }
        fn gn_specific_wrapper(
            val_argument_reprs: &[__ValArgumentReprInterface],
            value_at_generic_pedestal: __Value,
        ) -> __ValControlFlow {
            let value_stands = &mut Default::default();
            let value_at_generic_pedestal: &<$gn_item as __IsGnItem>::ValueAtGenericPedestal =
                <&<$gn_item as __IsGnItem>::ValueAtGenericPedestal as FromValue>::from_value_temp(
                    value_at_generic_pedestal,
                    value_stands,
                );
            // todo: catch unwind
            __ValControlFlow::Continue(
                __ValueLeashTest(<$gn_item as __IsGnItem>::eval(
                    val_argument_reprs,
                    value_at_generic_pedestal,
                ))
                .into_value(),
            )
        }
        __LinkageImpl::RitchieGn {
            // ad hoc
            generic_pedestal: <$gn_item as __IsGnItem>::generic_pedestal,
            gn_generic_wrapper,
            gn_specific_wrapper,
        }
    }};
}

#[macro_export]
macro_rules! struct_field_linkage_impl {
    ($owner_ty: ty, $field: ident) => {{
        fn struct_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    __ValueLeashTest(owner.downcast_into_owned::<$owner_ty>().$field).into_value()
                }
                Value::Leash(owner) => __ValueLeashTest(
                    (&((owner as &'static dyn std::any::Any)
                        .downcast_ref::<$owner_ty>()
                        .unwrap()
                        .$field) as &'static _),
                )
                .into_value(),
                Value::Ref(owner) => todo!("struct_field_wrapper Ref"),
                Value::Mut(owner) => todo!("struct_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        LinkageImpl::StructField {
            struct_field_wrapper,
        }
    }};
}

#[test]
fn struct_field_linkage_impl_works() {
    use crate::standard::ugly::__ValueLeashTest;
    struct A {
        x: i32,
    }

    let _: LinkageImpl<()> = struct_field_linkage_impl!(A, x);
}

#[macro_export]
macro_rules! enum_u8_presenter_linkage_impl {
    ($ty: ty) => {
        __LinkageImpl::EnumU8ValuePresenter {
            presenter: |index: u8, _, _| {
                let slf: $ty = unsafe { std::mem::transmute(index) };
                __ValuePresentation::AdHoc(format!("{slf:?}"))
            },
        }
    };
}
