use husky_data_viewer::DataViewerDb;
use husky_vm::InterpreterQueryGroup;

use crate::*;

impl<Task: IsTask> Runtime for DevRuntime<Task> {
    fn db(&self) -> &(dyn husky_val_repr::db::ValReprDb + std::panic::RefUnwindSafe) {
        self.comptime.db()
    }

    fn session(&self) -> &Session {
        todo!()
        // match self.variant {
        //     HuskyRuntimeVariant::None => todo!(),
        //     HuskyRuntimeVariant::Learning { ref session } => session,
        // }
    }

    fn evaluator_config(&self) -> &EvaluatorConfig {
        todo!()
        // &self.config.evaluator
    }

    fn opt_static_husky_feature_eval(&self) -> Option<&dyn Runtime> {
        Some(self)
    }
}
