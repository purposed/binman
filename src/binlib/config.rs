use std::fs;
use std::io::BufWriter;

use anyhow::{anyhow, Result};

use dirs;

use shellexpand::tilde;

use serde::{Deserialize, Serialize};

use serde_json;

fn default_code_host() -> String {
    String::from("github.com/purposed")
}

fn default_install_location() -> String {
    String::from("~/bin")
}

fn default_state_file_path() -> String {
    String::from(
        dirs::config_dir()
            .unwrap()
            .join("purposed")
            .join("binman")
            .join("state.json")
            .to_str()
            .unwrap(),
    )
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_code_host")]
    pub default_code_host: String,

    #[serde(default = "default_install_location")]
    pub install_location: String,

    #[serde(default = "default_state_file_path")]
    pub state_file_path: String,
}

impl Config {
    fn config_file_path() -> Result<String> {
        if let Some(config_dir) = dirs::config_dir() {
            let binman_config_dir = &config_dir.join("purposed").join("binman");
            fs::create_dir_all(binman_config_dir)?;
            return Ok(String::from(
                binman_config_dir.join("config.json").to_str().unwrap(),
            ));
        }
        Err(anyhow!("No configuration directory found"))
    }

    fn load_config_raw() -> Result<String> {
        let config_file_path = Config::config_file_path()?;
        let raw = fs::read_to_string(config_file_path).unwrap_or_else(|_| String::from("{}"));
        Ok(raw)
    }

    pub fn new() -> Result<Config> {
        let raw_data = Config::load_config_raw()?;
        let mut cfg: Config = serde_json::from_str(&raw_data)?;
        cfg.save()?; // Useful to apply defaults.
        Ok(cfg)
    }

    fn ensure_abs(&mut self) {
        self.install_location = tilde(&self.install_location).to_string();
        self.default_code_host = tilde(&self.default_code_host).to_string();
    }

    pub fn save(&mut self) -> Result<()> {
        self.ensure_abs();
        let config_path = Config::config_file_path()?;
        let file_handle = fs::File::create(config_path)?;
        serde_json::to_writer(BufWriter::new(file_handle), &self)?;
        Ok(())
    }
}
