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
            dev_unreturneds,
            dev_nones,
            dev_trues,
            dev_falses,
            dev_partition_noness,
        } => view! {
            scope,
            div (
                class="TraceStatsView",
                style=format!("padding-left: {}ch", 3 + props.indent),
            ) {
                div (class = "ArrivalStats") {
                    "A "
                    (*dev_arrivals)
                }
                div (class = "UnreturnedStats") {
                    "R "
                    (*dev_arrivals - *dev_unreturneds)
                }
                div (class = "TrueStats") {
                    "TP "
                    (*dev_trues)
                }
                div (class = "FalseStats") {
                    "FP "
                    (*dev_falses)
                }
                (View::new_fragment(dev_partition_noness.iter().enumerate().map(
                    |(idx, (partition, dev_partition_nones))| {
                        let partition_name = partition.name();
                        view!{
                            scope,
                            div (class = "DevPartitionNoneStats") {
                                "N["
                                (partition_name)
                                "] "
                                (*dev_partition_nones)
                            }
                        }
                    }
                ).collect()))
            }
        },
    }
}
