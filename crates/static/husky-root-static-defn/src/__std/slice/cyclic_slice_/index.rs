use husky_print_utils::msg_once;

use super::*;

pub(super) fn virtual_cyclic_slice_index_move<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    todo!()
}

pub(super) unsafe fn virtual_cyclic_slice_index_copy<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    assert_eq!(
        values[0].vtable as *const _,
        &__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE as *const _
    );
    let this_value: &DeprecatedVirtualCyclicSlice =
        values[0].downcast_temp_ref(&__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE);
    let i: usize = values[1].downcast_i32() as usize;
    this_value[i].bind_copy()
}

pub(super) unsafe fn virtual_cyclic_slice_index_eval_ref<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    assert_eq!(
        values[0].vtable as *const _,
        &__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE as *const _
    );
    let this_value: &'eval DeprecatedVirtualCyclicSlice =
        values[0].downcast_eval_ref(&__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE);
    let i: usize = values[1].downcast_i32() as usize;
    this_value[i].eval_bind_eval_ref()
}

pub(super) unsafe fn virtual_cyclic_slice_index_temp_ref<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    let this_value: &DeprecatedVirtualCyclicSlice =
        values[0].downcast_temp_ref(&__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE);
    let i: usize = values[1].downcast_i32() as usize;
    this_value[i].bind_temp_ref()
}

pub(super) unsafe fn virtual_cyclic_slice_index_temp_mut<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    panic!()
    // let i: usize = values[1].downcast_i32() as usize;
    // let this_value: &mut DeprecatedVirtualCyclicSlice = values[0].downcast_temp_mut();
    // this_value[i].bind_temp_mut()
}
