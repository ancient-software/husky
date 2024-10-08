use crate::*;
use husky_yaml_utils::ordered::OrderedYaml;
use yaml_rust2::Yaml;

#[derive(Debug)]
pub struct Soul {
    src: Vec<SourceFile>,
    config: OrderedYaml,
}

#[derive(Debug)]
pub struct SourceFile {
    origin: String,
    destination: String,
}

impl Soul {
    pub(super) fn new(yaml: &Yaml) -> Self {
        Self {
            src: yaml["src"]
                .as_hash()
                .map(|h| {
                    h.iter()
                        .filter_map(|(k, v)| {
                            Some(SourceFile {
                                origin: k.as_str()?.to_string(),
                                destination: v.as_str()?.to_string(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default(),
            config: OrderedYaml::new(&yaml["config"]),
        }
    }
}
