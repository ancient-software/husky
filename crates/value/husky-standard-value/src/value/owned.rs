use super::*;

#[derive(Debug)]
pub struct OwnedValue(Box<dyn StaticDyn>);

impl std::ops::Deref for OwnedValue {
    type Target = dyn StaticDyn;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl OwnedValue {
    pub(super) fn index_owned_dyn(self, index: usize) -> ExceptedValue {
        self.0.index_owned_dyn(index)
    }

    pub(super) fn upcast_from_owned<T>(t: T) -> Self
    where
        T: Static,
    {
        Self(Box::<T>::new(t))
    }

    pub fn downcast_into_owned<T>(self) -> T
    where
        T: 'static,
    {
        *((self.0 as Box<dyn std::any::Any>).downcast().unwrap())
    }

    pub(super) fn downcast_as_ref<T>(&self) -> &T
    where
        T: WeakStatic,
    {
        unsafe {
            std::mem::transmute(
                ((&*self.0 as &dyn StaticDyn) as &dyn std::any::Any)
                    .downcast_ref::<T::Static>()
                    .unwrap(),
            )
        }
    }

    pub(super) fn into_inner(self) -> Box<dyn StaticDyn> {
        self.0
    }

    pub(super) fn as_ref(&self) -> &dyn StaticDyn {
        &*self.0
    }
}
