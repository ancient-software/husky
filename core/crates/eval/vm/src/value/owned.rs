#[cfg(test)]
use check_utils::*;

use crate::*;

use super::*;

pub struct OwnedValue<'temp, 'eval: 'temp>(Box<dyn __AnyValueDyn<'eval> + 'temp>);

impl<'temp, 'eval: 'temp> From<Box<dyn __AnyValueDyn<'eval> + 'eval>> for OwnedValue<'temp, 'eval> {
    fn from(boxed_value: Box<dyn __AnyValueDyn<'eval> + 'eval>) -> Self {
        Self(boxed_value)
    }
}

impl<'temp, 'eval: 'temp> Clone for OwnedValue<'temp, 'eval> {
    fn clone(&self) -> Self {
        Self(self.0.clone_into_box_dyn())
    }
}

impl<'temp, 'eval: 'temp> PartialEq for OwnedValue<'temp, 'eval> {
    fn eq(&self, other: &OwnedValue<'temp, 'eval>) -> bool {
        self.0.equal_any(&*other.0)
    }
}

impl<'temp, 'eval: 'temp> Eq for OwnedValue<'temp, 'eval> {}

impl<'temp, 'eval: 'temp> OwnedValue<'temp, 'eval> {
    pub fn new<T: __AnyValueDyn<'eval> + 'eval>(value: T) -> OwnedValue<'temp, 'eval> {
        Self(Box::new(value))
    }

    pub fn from_any_ref(value: &(dyn __AnyValueDyn<'eval> + 'eval)) -> OwnedValue<'temp, 'eval> {
        Self(value.clone_into_box_dyn())
    }

    pub fn take<T: __AnyValue<'eval>>(self) -> EvalResult<T> {
        // check type
        if (*self.0).static_type_id_dyn() != T::static_type_id() {
            Err(vm_runtime_error!(format!("type_mismatch")))
        } else {
            let raw_pointer = Box::into_raw(self.0);
            Ok(unsafe { *Box::from_raw(raw_pointer as *mut T) })
        }
    }

    pub fn take_into_arc(self) -> Arc<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'eval,
    {
        let raw_pointer = Box::<(dyn __AnyValueDyn<'eval> + 'temp)>::into_raw(self.0);
        unsafe { (*raw_pointer).take_into_arc() }
    }

    pub fn any_ptr(&self) -> *const (dyn __AnyValueDyn<'eval> + 'temp) {
        &*(self.0)
    }

    pub fn any_ref(&self) -> &(dyn __AnyValueDyn<'eval> + 'temp) {
        &*self.0
    }

    pub fn any_mut_ptr(&mut self) -> *mut (dyn __AnyValueDyn<'eval> + 'temp) {
        &mut *self.0
    }

    pub fn downcast_ref<T: __AnyValue<'eval> + 'temp>(&self) -> &T {
        self.0.downcast_ref()
        // if T::static_type_id() != self.0.static_type_id() {
        //     panic!()
        // }
        // let ptr: *const dyn __AnyValueDyn = &*self.0;
        // let ptr: *const T = ptr as *const T;
        // unsafe { &*ptr }
    }

    pub fn get_json_value(&self) -> serde_json::value::Value {
        self.0.to_json_value_dyn()
    }
}

impl<'temp, 'eval: 'temp> std::fmt::Debug for OwnedValue<'temp, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn test_owned() {
    let a = OwnedValue::new(0 as i32);
    let b: i32 = a.take().unwrap();
    should_eq!(b, 0);
}

#[test]
fn test_owned_type_mistach() {
    let a = OwnedValue::new(0 as i32);
    let b = a.take::<f32>();
    should!(b.is_err());
}
