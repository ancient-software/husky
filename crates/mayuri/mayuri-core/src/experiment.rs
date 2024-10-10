pub mod path;

use self::path::ExperimentPath;
use crate::{
    src::{MayuriSrc, MayuriSrcFile},
    *,
};
use config::nemu::NemuConfig;
use husky_sha_utils::Sha512Output;
use husky_yaml_utils::ordered::OrderedYaml;
use makefile::MayuriMakefileExtracted;
use vec_like::ordered_vec_map::OrderedVecPairMap;
use yaml_rust2::Yaml;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Experiment {
    path: ExperimentPath,
    /// maps destination paths to files
    src_files: ExperimentSrcFiles,
    config: OrderedYaml,
    makefile: MayuriMakefileExtracted,
}

pub type ExperimentSrcFiles = OrderedVecPairMap<String, MayuriSrcFile>;

impl Experiment {
    pub(super) fn new(
        yaml: &Yaml,
        src: &MayuriSrc,
        makefile: MayuriMakefileExtracted,
        nemu_config: &NemuConfig,
    ) -> Self {
        Self {
            path: ExperimentPath::new(yaml, nemu_config),
            src_files: nemu_config
                .src_paths()
                .iter()
                .map(|src_path| (src_path.path().to_string(), src[src_path.path()].clone()))
                .chain(
                    yaml["src"]
                        .as_hash()
                        .expect("expected hash")
                        .iter()
                        .map(|(k, v)| {
                            (
                                k.as_str()
                                    .expect("invalid yaml, expected string")
                                    .to_string(),
                                src[v.as_str().expect("invalid, expected string")].clone(),
                            )
                        }),
                )
                .collect(),
            makefile,
            config: OrderedYaml::new(&yaml["config"]),
        }
    }
}

impl Experiment {
    fn run_local(&self) {
        todo!()
    }

    /// Copy code from origin to destination
    fn prepare_local(&self) -> std::io::Result<()> {
        use std::fs::{self, File};
        use std::io::Write;
        use std::path::Path;

        for (dest, src_file) in &self.src_files {
            let dest_path = Path::new(dest);

            // Create parent directories if they don't exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Read the content from the origin file
            let content = &src_file.content;

            // Write the content to the destination file
            let mut file = File::create(dest_path)?;
            file.write_all(content.as_bytes())?;
        }

        Ok(())
    }
}
