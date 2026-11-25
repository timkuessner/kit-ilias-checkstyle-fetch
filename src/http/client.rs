use std::{sync::Arc, time::Duration};

use reqwest::{Client, cookie::Jar, redirect::Policy};

use crate::error::Result;

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
}
