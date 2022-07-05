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

impl<'eval> HasStaticTypeInfo for VirtualCyclicSlice<'eval> {
    type StaticSelf = VirtualCyclicSlice<'static>;

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval, 'eval0: 'eval> AnyValue<'eval> for VirtualCyclicSlice<'eval0> {
    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        todo!()
    }

    fn static_ty() -> EntityRoutePtr {
        panic!()
    }

    fn ty(&self) -> EntityRoutePtr {
        self.ty
    }

    fn print_short(&self) -> String {
        format!(
            "{{ start: {}, end: {}, data: {} }}",
            self.start,
            self.end,
            print_sequence(
                "{ ",
                self.iter(),
                &|value| format!("{}", value.any_ref().print_short()),
                " }",
                20,
            )
        )
    }

    fn opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        todo!()
    }
}
