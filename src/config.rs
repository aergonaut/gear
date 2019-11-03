//! Configuration documentation.
//!
//! Gear uses two configuration files:
//!
//! 1. A project config file: `gear.toml`.
//! 2. A global config file stored in a canonical location.
//!
//! All config files use the [TOML][] format.
//!
//! [TOML]: https://github.com/toml-lang/toml
//!
//! See the documentation for each file for more information on the format and available options.

use crate::util;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// `gear` can be configured on a per-project basis using a project config file.
///
///`gear` will search for a file named `gear.toml` starting in the current directory and walking
/// back to root. The file uses the [TOML][] format.
///
/// [TOML]: https://github.com/toml-lang/toml
///
/// # `commands` table
///
/// The `commands` table is the main table in the project config. Each `gear` subcommand stores its
/// configuration options in the `commands` table. For example, configuration for the `gear pr`
/// subcommand is stored in `commands.pr`.
#[derive(Debug)]
pub struct ProjectConfig {
    raw: toml::Value,
}

impl ProjectConfig {
    pub(crate) fn load() -> crate::errors::Result<ProjectConfig> {
        let cwd = env::current_dir()?;
        ProjectConfig::load_from(&cwd)
    }

    pub(crate) fn load_from(path: &Path) -> crate::errors::Result<ProjectConfig> {
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

    pub(crate) fn commands(&self) -> Option<&toml::value::Table> {
        self.raw
            .as_table()
            .and_then(|raw| raw.get("commands"))
            .and_then(|commands| commands.as_table())
    }
}

/// Global config file containing global options.
///
/// The global config file is stored in a canonical location, depending on your OS:
///
/// - on Linux: `$XDG_CONFIG_HOME/gear/config.toml` or `$HOME/.config/gear/config.toml`
/// - on Windows: `{FOLDERID_RoamingAppData}/gear/config/config.toml`
/// - on macOS: `$HOME/Library/Preferences/gear/config.toml`
///
/// Currently there are no global options, so the global config file is not used. When options are
/// added, they will be documented on this page.
#[derive(Debug)]
pub struct GlobalConfig {
    raw: toml::Value,
}
