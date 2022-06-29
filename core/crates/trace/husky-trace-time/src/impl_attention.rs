use std::time::Instant;

use crate::*;
use text::Text;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub fn set_attention(&mut self, attention: Attention) {
        self.attention = attention;
        if let Some(sample_id0) = self.attention.opt_sample_id() {
            let main_file = self.eval_time().compile_time().main_file();
            let main_feature_repr = self.eval_time().main_feature_repr(main_file);
            match self
                .eval_time_singleton
                .eval_feature_repr(&main_feature_repr, sample_id0)
            {
                Ok(_) => (),
                Err(e) => match e {
                    EvalError::FromBatch { sample_id, .. } => {
                        self.set_attention_raw(Attention::Specific { sample_id })
                    }
                    EvalError::Normal { .. } => (),
                },
            }
            for root_trace_id in self.root_trace_ids.clone() {
                let _ = self.keyed_trace_stalk(root_trace_id);
            }
        }
    }

    fn set_attention_raw(&mut self, attention: Attention) {
        self.attention = attention;
    }
}
