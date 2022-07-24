use super::*;
use husky_entity_route::make_route;
use husky_print_utils::msg_once;
use thin_vec::thin_vec;
use word::RootIdentifier;

impl<'a, T> __StaticInfo for Vec<T>
where
    T: __StaticInfo,
{
    type __StaticSelf = Vec<T::__StaticSelf>;

    fn __static_type_name() -> Cow<'static, str> {
        format!("[]{}", T::__static_type_name()).into()
    }
}
