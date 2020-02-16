use rood::sys::Platform;

use semver::Version;
use serde::Deserialize;

use super::Asset;
use crate::binman::fuzzy_semver::parse_version_fuzzy;
use crate::error::BinmanResult;

#[derive(Clone, Debug, Deserialize)]
pub struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

impl Release {
    pub fn version(&self) -> BinmanResult<Version> {
        parse_version_fuzzy(&self.tag_name)
    }

    pub fn platform_assets(&self) -> Vec<&Asset> {
        // TODO: Actually check architecture along with platform.
        let cur_platform = Platform::detect();
        self.assets
            .iter()
            .filter(|asset| asset.platform() == cur_platform)
            .collect()
    }
}
