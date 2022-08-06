use std::path::Path;

use husky_word::snake_to_dash;

use crate::*;

pub fn cargo_toml_content(
    db: &dyn RustCodeGenQueryGroup,
    target_entrance: FilePtr,
    husky_dir: &str,
) -> String {
    let package = db.package(target_entrance).unwrap();
    let package_ident = package.ident;
    let dashed_package_ident = snake_to_dash(&package_ident);
    msg_once!("ad hoc");
    format!(
        r#"[package]
name = "{dashed_package_ident}"
version = "0.0.0"
description = "generated by husky compiler"
license = "MIT OR Apache-2.0"
edition = "2021"
rust-version = "1.56"

[dependencies]
__husky = {{ path = "{husky_dir}/core/crates/static/__husky" }}

[lib]
crate-type = ["cdylib"]

[profile.dev]
opt-level = 3
lto = true

[profile.release]
opt-level = 3
lto = true
"#
    )
}
