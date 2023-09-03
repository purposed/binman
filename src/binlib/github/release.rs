use rood::sys::{Architecture, Platform};

use semver::{Prerelease, Version};

use serde::Deserialize;

use crate::fuzzy_semver::parse_version_fuzzy;

use super::Asset;

#[derive(Clone, Debug, Deserialize)]
pub struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

impl Release {
    pub fn version(&self) -> Version {
        parse_version_fuzzy(&self.tag_name).unwrap_or_else(|_| {
            let mut v = Version::new(0, 0, 0);
            v.pre = Prerelease::new(&self.tag_name).unwrap();
            v
        })
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
