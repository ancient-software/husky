use husky_linket_impl::{eval_context::DevEvalContext, linket_impl::IsLinketImpl};

pub trait IsDevsoulInterface {
    type LinketImpl: IsLinketImpl;

    unsafe fn set_dev_eval_context(ctx: DevEvalContext<Self::LinketImpl>);
    unsafe fn unset_dev_eval_context();
    fn dev_eval_context() -> DevEvalContext<Self::LinketImpl>;
}
