use rood::sys::{Architecture, Platform};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    name: String,
    pub browser_download_url: String,
}

impl Asset {
    fn strip_extension(&self) -> &str {
        self.name.split('.').next().unwrap()
    }

    pub fn extension(&self) -> &str {
        if !self.name.contains('.') {
            return "";
        }
        match self.name.split('.').last() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn name(&self) -> &str {
        match self.strip_extension().split('-').next() {
            Some(name) => name,
            None => "unknown_artifact",
        }
    }

    pub fn architecture(&self) -> Architecture {
        match self.strip_extension().split('-').last() {
            Some(v) => Architecture::from(v),
            None => Architecture::Unknown,
        }
    }

    pub fn platform(&self) -> Platform {
        match self
            .strip_extension()
            .split('-')
            .collect::<Vec<&str>>()
            .get(1)
        {
            Some(v) => Platform::from(*v),
            None => Platform::Unknown,
        }
    }
}
