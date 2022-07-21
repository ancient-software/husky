use __husky_root::__main_utils::*;
use husky_compile_time::*;
use husky_debugger::*;
use test_input_major::__init__::LINKAGES;

#[tokio::main]
async fn main() {
    let code_snapshot_dir = "crates/test-input-major/snapshot/test-input-major".into();
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
