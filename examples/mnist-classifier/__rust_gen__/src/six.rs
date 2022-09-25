use crate::*;

pub(crate) fn six_match<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval crate::fermi::FermiMatchResult<'eval> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::six::six_match");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_box::<crate::fermi::FermiMatchResult<'eval>>(crate::fermi::fermi_match(&crate::major::major_concave_components(__ctx), &vec![ThickFp::__new_base(upmost as fn(&'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>)], __ctx), &__registration__::__FERMI_MATCH_RESULT_VTABLE)),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
}
pub(crate) fn six_match_refined1<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval crate::fermi::FermiMatchResult<'eval> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::six::six_match_refined1");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_box::<crate::fermi::FermiMatchResult<'eval>>(crate::fermi::fermi_match(&crate::major::major_concave_components(__ctx), &vec![ThickFp::__new_base(upmost as fn(&'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>), ThickFp::__new_ctx(bottom1 as fn(&'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>)->Option<f32>)], __ctx), &__registration__::__FERMI_MATCH_RESULT_VTABLE)),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
}
pub(crate) fn upmost<'eval>(
    cc: &'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
) -> Option<f32> {
    let dp = cc.displacement();
    normal_require!(dp.y > 0f32);
    return Some(dp.y);
}
pub(crate) fn bottom1<'eval>(
    cc: &'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
    __ctx: &dyn __EvalContext<'eval>,
) -> Option<f32> {
    let dp = cc.displacement();
    if dp.y < -3f32 {
        normal_require!((dp.x / dp.y).abs() > 1.4f32);
    }
    normal_require!(cc.relative_bounding_box(__ctx).ymax() < 0.6f32);
    let relative_end = cc
        .line_segment_sketch
        .bounding_box(__ctx)
        .relative_point(&cc.end());
    normal_require!(relative_end.x > 0.5f32);
    return Some(-cc.end().y);
}
