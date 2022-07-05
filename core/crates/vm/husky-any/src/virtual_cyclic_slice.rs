use cyclic_slice::CyclicSlice;
use husky_entity_route::EntityRoutePtr;
use print_utils::{msg_once, p};
use serde::Serialize;
use word::{CustomIdentifier, IdentPairDict};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualCyclicSlice<'eval> {
    pub ty: EntityRoutePtr,
    pub data: CyclicSlice<'eval, MemberValue<'eval>>,
}

impl<'eval> std::ops::Deref for VirtualCyclicSlice<'eval> {
    type Target = CyclicSlice<'eval, MemberValue<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for VirtualCyclicSlice<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> HasStaticTypeInfo for VirtualCyclicSlice<'eval> {
    type __StaticSelf = VirtualCyclicSlice<'static>;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval, 'eval0: 'eval> AnyValue<'eval> for VirtualCyclicSlice<'eval0> {
    fn __to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn __short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        todo!()
    }

    fn __static_ty() -> EntityRoutePtr {
        panic!()
    }

    fn __ty(&self) -> EntityRoutePtr {
        self.ty
    }

    fn __print_short(&self) -> String {
        format!(
            "{{ start: {}, end: {}, data: {} }}",
            self.start,
            self.end,
            print_sequence(
                "{ ",
                self.iter(),
                &|value| format!("{}", value.any_ref().__print_short()),
                " }",
                20,
            )
        )
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Group(
            self.iter()
                .enumerate()
                .map(|(i, element)| visualize_element(i, element.any_ref().__short_dyn()))
                .collect::<__EvalResult<Vec<_>>>()?,
        )))
    }
}
