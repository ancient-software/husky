#[cfg(test)]
use super::*;

/// # destructor

#[macro_export]
macro_rules! struct_destructor_linkage_impl {
    ($self_ty: ty, $ty_path: path, $($fields: ident),* $(,)?) => {{
        fn struct_destructor_wrapper(owner: Value) -> Vec<Value> {
            match owner {
                Value::Owned(owner) => {
                    let $ty_path { $($fields),* } = owner.downcast_into_owned::<$self_ty>() else {
                        unreachable!()
                    };
                    vec![$($fields.into_value()),*]
                }
                _ => unreachable!(),
            }
        }
        __LinkageImpl::StructDestructor {
            struct_destructor_wrapper,
        }
    }};
}

// # field

#[macro_export]
macro_rules! struct_field_linkage_impl {
    ($self_ty: ty, $field: ident) => {{
        fn struct_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    __ValueLeashTest(owner.downcast_into_owned::<$self_ty>().$field).into_value()
                }
                Value::Leash(owner) => __ValueLeashTest(
                    (&((owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                        .$field) as &'static _),
                )
                .into_value(),
                Value::Ref(owner) => todo!("struct_field_wrapper Ref"),
                Value::Mut(owner) => todo!("struct_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        __LinkageImpl::StructField {
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

    let _: StandardLinkageImpl<()> = struct_field_linkage_impl!(A, x);
}
