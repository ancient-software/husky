pub mod jar;
pub mod jar0;

use sealed::sealed;
use std::pin::Pin;

pub trait IsMemo {
    type Jar: IsMemoJar;
}

#[sealed]
pub trait IsMemoJar: IsMemoJarDyn + Default + 'static {}

#[sealed]
pub trait IsMemoJarDyn: std::any::Any + Send + Sync + 'static {}

pub struct MemoJarDyn(Pin<Box<dyn IsMemoJarDyn>>);

impl MemoJarDyn {
    pub fn new<M: IsMemo>() -> Self {
        Self(Box::pin(M::Jar::default()))
    }

    pub fn downcast<J: IsMemoJarDyn>(&self) -> &J {
        (&*self.0 as &dyn std::any::Any)
            .downcast_ref::<J>()
            .unwrap()
    }
}
