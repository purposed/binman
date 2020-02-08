use semver::Version;

use crate::error::{BinmanError, BinmanResult, Cause};

fn trim_first_char(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

pub fn parse_version_fuzzy(version: &str) -> BinmanResult<Version> {
    match Version::parse(version) {
        Ok(v) => Ok(v),
        Err(e) => {
            if version.starts_with("v") {
                Ok(Version::parse(trim_first_char(version).ok_or(
                    BinmanError::new(Cause::InvalidVersion, "Could not trim version string"),
                )?)?)
            } else {
                Err(BinmanError::from(e))
            }
        }
    }
}
