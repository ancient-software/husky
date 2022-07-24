use std::borrow::Cow;

use husky_entity_route::EntityRoutePtr;
use husky_print_utils::{msg_once, p};
use serde::Serialize;
use word::{CustomIdentifier, IdentPairDict};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualVec<'eval> {
    ty: EntityRoutePtr,
    data: Vec<__Register<'eval>>,
}

impl<'eval> VirtualVec<'eval> {
    pub fn new(ty: EntityRoutePtr, data: Vec<__Register<'eval>>) -> Self {
        Self { ty, data }
    }
}

impl<'eval> std::ops::Deref for VirtualVec<'eval> {
    type Target = Vec<__Register<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for VirtualVec<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> __StaticInfo for VirtualVec<'eval> {
    type __StaticSelf = VirtualVec<'static>;

    fn __static_type_name__() -> Cow<'static, str> {
        "[]Any".into()
    }
}

impl<'eval> __Registrable for VirtualVec<'eval> {
    unsafe fn __to_register__<'eval0>(self) -> __Register<'eval0> {
        __Register::new_box(self)
    }
}
