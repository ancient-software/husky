use crate::{*};

impl Debugtime {
    pub(crate) fn update_trace_statss(&mut self) -> DebugtimeUpdateM<()> {
        for root_trace_id in self.root_traces() {
            self.update_trace_statss_within_trace(root_trace_id)?;
        }
        DebugtimeUpdateM::Ok(())
    }

    fn update_trace_statss_within_trace(&mut self, trace_id: TraceId) -> DebugtimeUpdateM<()> {
        let trace_node_data = self.trace_node_data(trace_id);
        let expanded = trace_node_data.expanded;
        let trace_raw_data = &trace_node_data.trace_data;
        let trace_stats_key = TraceStatsKey {
            trace_id,
            partitions: self.state.presentation().partitions().clone(),
        };
        let associated_trace_ids = trace_raw_data.associated_trace_ids();
        if !self.state.trace_statss.contains(&trace_stats_key) {
            self.gen_trace_stats(trace_id, trace_stats_key)?
        }
        for associated_trace_id in associated_trace_ids {
            self.update_trace_statss_within_trace(associated_trace_id)?
        }
        if expanded {
            for subtrace_id in self.subtraces(trace_id) {
                self.update_trace_statss_within_trace(subtrace_id)?
            }
        }
        DebugtimeUpdateM::Ok(())
    }

    fn gen_trace_stats(&mut self, trace_id: TraceId, key: TraceStatsKey) -> DebugtimeUpdateM<()> {
        let (opt_stats, result) = self
            .trace(trace_id)
            .variant
            .opt_stats_result(self.runtime(), self.state.presentation().partitions())
            .split();
        self.state.trace_statss.insert_new(key, opt_stats);
        self.updating_t(result)
    }

    fn updating_t(&self, result: __VMResult<()>) -> DebugtimeUpdateM<()> {
        match result {
            Ok(()) => DebugtimeUpdateM::Ok(()),
            Err(e) => match e.variant() {
                __VMErrorVariant::Normal => todo!(),
                __VMErrorVariant::FromBatch { sample_id } => {
                    if self.state.presentation().is_generic()
                        || self.state.presentation().sample_id() != SampleId(*sample_id)
                    {
                        DebugtimeUpdateM::OtherworldlyErr(e)
                    } else {
                        DebugtimeUpdateM::Ok(())
                    }
                }
            },
        }
    }
}

trait ResultX<T, E>
where
    T: Default,
{
    fn split(self) -> (T, Result<(), E>);
}

impl<T, E> ResultX<T, E> for Result<T, E>
where
    T: Default,
{
    fn split(self) -> (T, Result<(), E>) {
        match self {
            Ok(t) => (t, Ok(())),
            Err(e) => (Default::default(), Err(e)),
        }
    }
}
