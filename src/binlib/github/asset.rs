use regex::Regex;

use rood::sys::{Architecture, Platform};

use serde::Deserialize;

fn parse_architecture(name: &str) -> Architecture {
    let archs = vec![Architecture::Amd64, Architecture::Arm, Architecture::Arm64];

    for arc in archs.iter() {
        for v in arc.value().iter() {
            println!("trying {} for {}", v, name);
            let ptn = Regex::new(&format!(r"-{}", v)).unwrap();
            if ptn.is_match(name) {
                println!("found");
                return arc.clone();
            }
        }
    }

    Architecture::Unknown
}

fn parse_platform(name: &str) -> Platform {
    let plats = vec![Platform::Linux, Platform::Darwin, Platform::Windows];

    for plat in plats.iter() {
        for plat_value in plat.value().iter() {
            let ptn = Regex::new(&format!(r"-{}[-\.]", plat_value)).unwrap();
            if ptn.is_match(name) {
                return plat.clone();
            }
        }
    }
    Platform::Unknown
}

#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    name: String,
    pub browser_download_url: String,
}

impl Asset {
    pub fn full_name(&self) -> &str {
        &self.name
    }

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
        parse_architecture(&self.name)
    }

    pub fn platform(&self) -> Platform {
        parse_platform(&self.name)
    }
}
