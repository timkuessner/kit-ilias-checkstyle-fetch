use std::{sync::Arc, time::Duration};

use reqwest::{Client, cookie::Jar, redirect::Policy};
use scraper::Html;
use url::Url;

use crate::{error::Result, http::form::FormData};

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let cookie_jar = Arc::new(Jar::default());

        let client = Client::builder()
            .cookie_provider(cookie_jar)
            .redirect(Policy::limited(10))
            .timeout(Duration::from_secs(30))
            .user_agent("checkstyle-fetch")
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_html(&self, url: &Url) -> Result<(Url, Html)> {
        let response = self
            .client
            .get(url.clone())
            .send()
            .await?
            .error_for_status()?;

        let final_url = response.url().clone();
        let body = response.text().await?;

        Ok((final_url, Html::parse_document(&body)))
    }

    pub async fn post_form(&self, url: &Url, form: &FormData) -> Result<(Url, Html)> {
        let response = self
            .client
            .post(url.clone())
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(form.encode())
            .send()
            .await?
            .error_for_status()?;

        let final_url = response.url().clone();
        let body = response.text().await?;

        Ok((final_url, Html::parse_document(&body)))
    }
}
