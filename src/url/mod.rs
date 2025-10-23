/*!
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co., Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: mod.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-23 17:27:21
 *
 * Modified By: mingcheng <mingcheng@apache.org>
 * Last Modified: 2025-10-23 22:52:49
 */

use crate::client::Client;
use crate::error::{Error, Result};
use crate::url::models::{DeleteRequest, DeleteResponse, ShortenRequest, ShortenResponse};

pub mod builder;
pub mod models;

pub trait ShortenService {
    fn shorten(&self, request: ShortenRequest) -> Result<ShortenResponse>;
    fn delete(&self, request: DeleteRequest) -> Result<DeleteResponse>;
}

impl ShortenService for Client {
    /// Shorten a URL using the configured service
    fn shorten(&self, request: ShortenRequest) -> Result<ShortenResponse> {
        if !self.is_valid_url(&request.target_url) {
            return Err(Error::InvalidUrl(request.target_url));
        }

        self.execute_request(reqwest::Method::POST, "/api/v1/shorten", request)
    }

    /// Delete a shortened URL using the configured service
    fn delete(&self, request: DeleteRequest) -> Result<DeleteResponse> {
        self.execute_request(reqwest::Method::DELETE, "/api/v1/shorten", request)
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::DEFAULT_DOMAIN, url::builder::UrlShortenerRequestBuilder};

    use super::*;
    use crate::config::Config;
    use log::warn;
    use reqwest::StatusCode;
    use std::env;

    #[test]
    fn test_shorten_and_delete_url() -> Result<()> {
        let api_key = env::var("URL_SHORTENER_API_KEY").unwrap_or_else(|_| "".to_string());

        if api_key.is_empty() {
            warn!("Skipping test_shorten_url: URL_SHORTENER_API_KEY not set");
            return Ok(());
        }

        let config = Config::default().with_api_key(&api_key);
        let client = Client::new(config)?;

        let request = UrlShortenerRequestBuilder::new("https://git.guanwaii.com/login")?
            .with_domain(DEFAULT_DOMAIN);

        let response = client.shorten(request.build()).unwrap();

        assert_eq!(response.code, StatusCode::OK.as_u16());
        assert_ne!(response.data.slug, "");
        assert_ne!(response.data.short_url, "");
        assert!(client.is_valid_url(&response.data.short_url));

        let response = client.delete(DeleteRequest {
            domain: DEFAULT_DOMAIN.to_string(),
            slug: response.data.slug,
        })?;

        assert_eq!(response.code, StatusCode::OK.as_u16());
        Ok(())
    }
}
