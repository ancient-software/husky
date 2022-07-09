use std::borrow::Cow;

use super::*;

impl<'temp, T> __HasStaticTypeInfo for &'temp [T]
where
    T: __HasStaticTypeInfo,
{
    type __StaticSelf = &'static [T::__StaticSelf];
    fn __static_type_name() -> Cow<'static, str> {
        todo!()
    }
}

impl<'temp, 'eval, 'a: 'eval, T: __AnyValue<'a> + 'temp> __AnyValue<'eval> for &'temp [T] {
    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn __to_json_value(&self) -> serde_json::value::Value {
        todo!()
        // serde_json::value::to_value(self).unwrap()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        todo!()
    }

    fn __ty(&self) -> EntityRoutePtr {
        todo!()
    }
    fn __print_short(&self) -> String {
        todo!()
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Group(
            self.iter()
                .enumerate()
                .map(|(i, element)| visualize_element(i, element.__short()))
                .collect::<__EvalResult<Vec<_>>>()?,
        )))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        todo!()
    }
}

fn gen_iter<'temp, 'eval: 'temp, T>(
    slice: &'temp [T],
) -> Box<dyn Iterator<Item = __TempValue<'temp, 'eval>> + 'temp>
where
    T: __AnyValueDyn<'eval> + 'eval,
{
    Box::new(slice.iter().map(|t| __TempValue::TempRefEval(t)))
}
