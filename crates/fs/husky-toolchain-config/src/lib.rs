/// this is the same as husky_vfs::toolchain_config
///
/// todo: refactor husky-vfs into two crates, let this crate depends on one and be dependent of the other
mod jar;

use husky_toml_utils::read_toml;
use husky_vfs::{
    path::virtual_path::VirtualPath,
    toolchain::{Toolchain, ToolchainData},
};
use serde::*;
use std::path::Path;
use std::path::PathBuf;

#[salsa::jar]
pub struct ToolchainConfigJar();

pub struct ToolchainConfig {
    toml: ToolchainToml,
    toolchain: Toolchain,
}

impl ToolchainConfig {
    pub fn toolchain(&self) -> Toolchain {
        self.toolchain
    }
}

#[derive(Deserialize)]
pub struct ToolchainToml {
    #[serde(rename = "toolchain")]
    pub toolchain_section: ToolchainSection,
}

#[derive(Serialize, Deserialize)]
pub struct ToolchainSection {
    #[serde(rename = "library-path")]
    library_path: PathBuf,
}

/// ad hoc
pub fn toolchain_config(dir: &Path, db: &::salsa::Db) -> ToolchainConfig {
    let mut dir: &Path = &dir.canonicalize().unwrap();
    let husky_toolchain_toml_path = loop {
        let husky_toolchain_toml_path = dir.join("husky-toolchain.toml");
        if husky_toolchain_toml_path.exists() {
            break husky_toolchain_toml_path;
        }
        dir = dir.parent().unwrap()
    };
    let toml: ToolchainToml = read_toml(&husky_toolchain_toml_path).unwrap();
    ToolchainConfig {
        toolchain: Toolchain::new(
            db,
            ToolchainData::Local {
                library_path: VirtualPath::try_new(
                    db,
                    dir.join(toml.toolchain_section.library_path.clone()),
                )
                .unwrap(),
            },
        ),
        toml,
    }
}
