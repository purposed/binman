use semver::Version;

use anyhow::{anyhow, Result};

fn trim_first_char(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

pub fn parse_version_fuzzy(version: &str) -> Result<Version> {
    match Version::parse(version) {
        Ok(v) => Ok(v),
        Err(e) => {
            if version.starts_with('v') {
                Ok(Version::parse(trim_first_char(version).ok_or_else(
                    || anyhow!("Could not parse version string"),
                )?)?)
            } else {
                Err(e.into())
            }
        }
    }
}
