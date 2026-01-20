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

        self.execute_request(reqwest::Method::POST, "/shorten", request)
    }

    /// Delete a shortened URL using the configured service
    fn delete(&self, request: DeleteRequest) -> Result<DeleteResponse> {
        self.execute_request(reqwest::Method::DELETE, "/shorten", request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DEFAULT_DOMAIN;
    use crate::test_helpers::helpers::{
        assert_not_empty, assert_status_ok, create_test_client, get_api_key_or_skip,
    };
    use crate::url::builder::UrlShortenerRequestBuilder;

    /// Helper function to create a shortened URL for testing
    fn create_test_short_url() -> Result<(Client, String)> {
        let client = create_test_client()?;

        let request = UrlShortenerRequestBuilder::new("https://git.guanwaii.com/login")?
            .with_domain(DEFAULT_DOMAIN)
            .build();

        let response = client.shorten(request)?;
        assert_status_ok(response.code);
        assert_not_empty(&response.data.slug);
        assert_not_empty(&response.data.short_url);
        assert!(client.is_valid_url(&response.data.short_url));

        Ok((client, response.data.slug))
    }

    #[test]
    fn test_shorten_url() -> Result<()> {
        // Skip test if API key is not set
        if get_api_key_or_skip().is_none() {
            return Ok(());
        }

        let client = create_test_client()?;

        let request = UrlShortenerRequestBuilder::new("https://git.guanwaii.com/login")?
            .with_domain(DEFAULT_DOMAIN)
            .build();

        let response = client.shorten(request)?;
        assert_status_ok(response.code);
        assert_not_empty(&response.data.slug);
        assert_not_empty(&response.data.short_url);
        assert!(client.is_valid_url(&response.data.short_url));

        // Cleanup: delete the created short URL
        let _ = client.delete(DeleteRequest {
            domain: DEFAULT_DOMAIN.to_string(),
            slug: response.data.slug,
        });

        Ok(())
    }

    #[test]
    fn test_delete_url() -> Result<()> {
        // Skip test if API key is not set
        if get_api_key_or_skip().is_none() {
            return Ok(());
        }

        // Create a short URL for deletion test
        let (client, slug) = create_test_short_url()?;

        // Test delete URL
        let delete_request = DeleteRequest {
            domain: DEFAULT_DOMAIN.to_string(),
            slug,
        };

        let delete_response = client.delete(delete_request)?;
        assert_status_ok(delete_response.code);

        Ok(())
    }
}
