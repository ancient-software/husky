use super::*;
use husky_trace_protocol::TraceStats;
use web_sys::Event;

#[derive(Prop)]
pub struct TraceStatsProps<'a> {
    stats: &'a TraceStats,
    indent: Indent,
}

pub fn TraceStatsView<'a, G: Html>(scope: Scope<'a>, props: TraceStatsProps<'a>) -> View<G> {
    match props.stats {
        TraceStats::Classification {
            dev_samples,
            dev_arrivals,
            dev_nulls,
            dev_trues,
            dev_falses,
        } => view! {
            scope,
            div (
                class="TraceStatsView",
                style=format!("padding-left: {}ch", 3 + props.indent),
            ) {
                "dev: "
                "samples = "
                (*dev_samples)
                ", arrivals = "
                (*dev_arrivals)
                ", nulls = "
                (*dev_nulls)
                ", trues = "
                (*dev_trues)
                ", falses = "
                (*dev_falses)
                ", val: todo!()"
            }
        },
    }
}
