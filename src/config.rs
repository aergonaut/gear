use crate::util;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct ProjectConfig {
    raw: toml::Value,
}

impl ProjectConfig {
    pub fn load() -> crate::errors::Result<ProjectConfig> {
        let cwd = env::current_dir()?;
        ProjectConfig::load_from(&cwd)
    }

    pub fn load_from(path: &Path) -> crate::errors::Result<ProjectConfig> {
        let mut contents = String::new();
        let config_path = util::ancestors(path)
            .find(|path| std::fs::metadata(&path.join("gear.toml")).is_ok())
            .map(|config_dir| config_dir.join("gear.toml"));

        if let Some(inner) = config_path {
            let mut config_file = File::open(inner)?;
            config_file.read_to_string(&mut contents)?;
        } else {
            log::debug!("No config file found");
        }

        let value = contents.parse::<toml::Value>()?;
        Ok(ProjectConfig { raw: value })
    }

    pub fn commands(&self) -> Option<&toml::value::Table> {
        self.raw
            .as_table()
            .and_then(|raw| raw.get("commands"))
            .and_then(|commands| commands.as_table())
    }
}
