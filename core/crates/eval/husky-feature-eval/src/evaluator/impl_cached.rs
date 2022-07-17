use crate::*;
use check_utils::should_eq;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::{CallFormSource, EntityDefnVariant};
use husky_feature_gen::*;
use husky_lazy_semantics::LazyStmt;
use husky_trace_protocol::VisualData;
use print_utils::{epin, msg_once, p};
use std::{iter::zip, panic::catch_unwind, sync::Arc};
use vm::__Linkage;
use vm::*;
use word::IdentPairDict;

use super::FeatureEvaluator;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(super) fn eval_cached(
        &mut self,
        eval_key: EvalKey<'eval>,
        f: impl FnOnce(&mut Self) -> __EvalValueResult<'eval>,
    ) -> __EvalValueResult<'eval> {
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = f(self);
            self.sheet.cache(eval_key, result)
        }
    }
}
