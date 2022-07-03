use husky_atom::*;
use husky_entity_route::EntityRoutePtr;
use singleton::singleton;

use crate::*;

singleton! {HuskyEvalTime}

// pub(crate) static mut HUSKY_EVAL_TIME_SINGLETON: Option<*const HuskyEvalTime> = None;

pub fn eval_time() -> &'static HuskyEvalTime {
    access_singleton()
    // unsafe { &*HUSKY_EVAL_TIME_SINGLETON.unwrap() }
}

pub fn parse_entity_route(text: &str) -> EntityRoutePtr {
    compile_time().parse_entity_route(text)
}

pub fn compile_time() -> &'static HuskyCompileTime {
    eval_time().compile_time()
}

// pub struct HuskyEvalTimeSingletonKeeper(Box<HuskyEvalTime>);

// impl std::ops::Deref for HuskyEvalTimeSingletonKeeper {
//     type Target = HuskyEvalTime;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl Drop for HuskyEvalTimeSingletonKeeper {
//     fn drop(&mut self) {
//         unsafe { HUSKY_EVAL_TIME_SINGLETON = None }
//     }
// }

// impl From<HuskyEvalTime> for HuskyEvalTimeSingletonKeeper {
//     fn from(eval_time: HuskyEvalTime) -> Self {
//         let boxed_eval_time = Box::new(eval_time);
//         unsafe { HUSKY_EVAL_TIME_SINGLETON = Some(&*boxed_eval_time) };
//         Self(boxed_eval_time)
//     }
// }
