use reqwest;
use reqwest::header;

use http::status::StatusCode;

use serde_json;

use super::{Release, Repository};
use crate::error::{BinmanError, BinmanResult, Cause};

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> BinmanResult<Client> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("binman"),
        );

        Ok(Client {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
        })
    }

    pub fn get_repository(&self, url: &str) -> BinmanResult<Repository> {
        Repository::from_url(url)
    }

    fn validate_response(&self, resp: &reqwest::Response) -> BinmanResult<()> {
        let not_found = StatusCode::from_u16(404).unwrap();
        let status_code: StatusCode = resp.status();
        if status_code == not_found {
            return Err(BinmanError::new(
                Cause::NotFound,
                "Repository or owner does not exist",
            ));
        }
        Ok(())
    }

    pub async fn latest_release(&self, repo: &Repository) -> BinmanResult<Release> {
        let release_url = repo.latest_release_url();

        let resp = self.client.get(&release_url).send().await?;
        let release_json = resp.text().await?;
        let release: Release = serde_json::from_str(&release_json)?;
        Ok(release)
    }

    pub async fn get_releases(&self, repo: &Repository) -> BinmanResult<Vec<Release>> {
        let releases_url = repo.releases_url();
        let resp = self.client.get(&releases_url).send().await?;
        self.validate_response(&resp)?;
        let releases_json = resp.text().await?;
        let releases: Vec<Release> = serde_json::from_str(&releases_json)?;
        Ok(releases)
    }
}
