use husky_debugger::*;
use __husky_root::__main_utils::*;
use test_struct_example2::__init__::LINKAGES;
use husky_compile_time::*;

#[tokio::main]
async fn main() {
    let code_snapshot_dir =
        "crates/test-struct-example2/snapshot/test-struct-example2".into();
    HuskyDebugger::new(
        HuskyDebuggerConfig {
            package_dir: code_snapshot_dir,
            opt_sample_id: Some(SampleId(23)),
            verbose: false,
            warn_missing_linkage: true,
        },
        LINKAGES,
    )
    .serve("localhost:51617")
    .await
    .expect("")
}
