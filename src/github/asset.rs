use rood::sys::{Architecture, Platform};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    name: String,
    pub browser_download_url: String,
}

impl Asset {
    pub fn name(&self) -> &str {
        match self.name.split("-").next() {
            Some(name) => name,
            None => "unknown_artifact",
        }
    }

    pub fn architecture(&self) -> Architecture {
        match self.name.split("-").last() {
            Some(v) => Architecture::from(v),
            None => Architecture::Unknown,
        }
    }

    pub fn platform(&self) -> Platform {
        match self.name.split("-").collect::<Vec<&str>>().get(1) {
            Some(v) => Platform::from(*v),
            None => Platform::Unknown,
        }
    }
}
