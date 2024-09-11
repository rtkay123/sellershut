use std::path::PathBuf;

pub struct Config {
    pub model_resource: PathBuf,
    pub config_resource: PathBuf,
    pub vocab_resource: PathBuf,
    pub merges_resource: PathBuf,
}
