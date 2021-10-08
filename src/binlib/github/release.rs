use anyhow::Result;

use rood::sys::{Architecture, Platform};

use semver::Version;

use serde::Deserialize;

use crate::fuzzy_semver::parse_version_fuzzy;

use super::Asset;

#[derive(Clone, Debug, Deserialize)]
pub struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

impl Release {
    pub fn version(&self) -> Result<Version> {
        parse_version_fuzzy(&self.tag_name)
    }

    pub fn platform_assets(&self) -> Vec<&Asset> {
        let cur_platform = Platform::detect();
        let cur_arch = Architecture::detect();

        // Hack to support Apple Silicon + Rosetta
        let fallback_architectures =
            if cur_platform == Platform::Darwin && cur_arch == Architecture::Arm64 {
                vec![Architecture::Arm64, Architecture::Amd64]
            } else {
                vec![cur_arch]
            };

        println!(
            "Assets: {:?}",
            &self
                .assets
                .iter()
                .map(|a| (String::from(a.name()), a.platform(), a.architecture()))
                .collect::<Vec<(String, Platform, Architecture)>>()
        );
        for arch in fallback_architectures {
            let arch_assets = self
                .assets
                .iter()
                .filter(|asset| asset.platform() == cur_platform && asset.architecture() == arch)
                .collect::<Vec<_>>();

            if !arch_assets.is_empty() {
                return arch_assets;
            }
        }

        Vec::default()
    }
}
