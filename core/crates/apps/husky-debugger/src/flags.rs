use std::path::PathBuf;

xflags::xflags! {
    cmd husky-debugger-flags
    {
        optional --package-dir package_dir: PathBuf
        optional --report-vm
        optional -v, --verbose
        optional --sample-id sample_id: String
        optional --mode mode: String
        optional -c, --compile
    }
}
