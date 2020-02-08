use serde::Deserialize;

use crate::error::{BinmanError, BinmanResult, Cause};

static GITHUB_RELEASES_PATTERN: &str = "https://api.github.com/repos/{owner}/{name}/releases";
static GITHUB_LATEST_RELEASE: &str = "https://api.github.com/repos/{owner}/{name}/releases/latest";

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub owner: String,
}

impl Repository {
    pub fn new(name: &str, owner: &str) -> Repository {
        Repository {
            name: String::from(name),
            owner: String::from(owner),
        }
    }

    fn format_repo_url(&self, url: &str) -> String {
        url.replace("{owner}", &self.owner)
            .replace("{name}", &self.name)
    }

    pub fn from_url(url: &str) -> BinmanResult<Repository> {
        let splitted: Vec<&str> = url.split("/").collect();
        if splitted.len() < 2 {
            return Err(BinmanError::new(
                Cause::InvalidData,
                &format!("URL \"{}\" is invalid", url),
            ));
        }

        Ok(Repository::new(
            splitted.last().unwrap(),
            splitted[splitted.len() - 2],
        ))
    }

    pub fn releases_url(&self) -> String {
        self.format_repo_url(GITHUB_RELEASES_PATTERN)
    }

    pub fn latest_release_url(&self) -> String {
        self.format_repo_url(GITHUB_LATEST_RELEASE)
    }
}
