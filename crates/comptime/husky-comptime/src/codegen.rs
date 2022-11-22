use std::path::Path;

use crate::*;
use husky_rust_code_gen::cargo_toml_content;

impl HuskyComptime {
    pub fn cargo_toml_content(&self, target_entrance: SourcePath, husky_dir: &Path) -> String {
        cargo_toml_content(self, target_entrance, husky_dir)
    }
}
