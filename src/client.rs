/*!
 * Copyright (c) 2025-2026 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: client.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-23 11:29:29
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2026-01-20 11:16:52
 */
use crate::config::Config;
use crate::error::{Error, ErrorResponse, Result};
use reqwest::StatusCode;
use reqwest::blocking::{Client as HttpClient, Response};
use std::sync::Arc;
use url::Url;

/// HTTP client for content sharing operations
#[derive(Debug, Clone)]
pub struct Client {
    http_client: HttpClient,
    config: Arc<Config>,
}

impl Client {
    /// Create a new content sharing client with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let http_client = HttpClient::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()?;

        Ok(Self {
            http_client,
            config: Arc::new(config),
        })
    }

    /// Create a new client with default configuration
    pub fn with_default_config() -> Result<Self> {
        Self::new(Config::default())
    }

    /// Handle the HTTP response and parse it into the desired type
    fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        let response_text = response.text()?;

        match status {
            StatusCode::OK | StatusCode::CREATED => {
                serde_json::from_str::<T>(&response_text).map_err(Error::JsonError)
            }
            StatusCode::NOT_FOUND => Err(Error::NotFound),
            StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimited),
            _ => {
                if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                    Err(Error::ServerError {
                        status: error_response.code as u16,
                        message: error_response.message,
                    })
                } else {
                    Err(Error::ServerError {
                        status: status.as_u16(),
                        message: response_text,
                    })
                }
            }
        }
    }

    /// Validate the URL format
    pub fn is_valid_url(&self, url_str: &str) -> bool {
        Url::parse(url_str)
            .map(|url| matches!(url.scheme(), "http" | "https"))
            .unwrap_or(false)
    }

    /// Build the API endpoint URL
    fn build_api_url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url, path)
    }

    /// Add authorization header if API key is configured
    fn add_auth_header(
        &self,
        req_builder: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        if let Some(api_key) = &self.config.api_key {
            req_builder.header("Authorization", api_key)
        } else {
            req_builder
        }
    }

    /// Process the request: add auth header, send, and handle response
    fn process_request<Res>(&self, req_builder: reqwest::blocking::RequestBuilder) -> Result<Res>
    where
        Res: serde::de::DeserializeOwned,
    {
        let req_builder = self.add_auth_header(req_builder);
        let response = req_builder.send()?;
        self.handle_response::<Res>(response)
    }

    /// Execute an API request with the given method and body
    pub fn execute_request<Req, Res>(
        &self,
        method: reqwest::Method,
        path: &str,
        request: Req,
    ) -> Result<Res>
    where
        Req: serde::Serialize,
        Res: serde::de::DeserializeOwned,
    {
        let url = self.build_api_url(path);
        let req_builder = self.http_client.request(method, &url).json(&request);
        self.process_request(req_builder)
    }

    /// Execute an API request without a request body
    pub fn execute_request_no_body<Res>(&self, method: reqwest::Method, path: &str) -> Result<Res>
    where
        Res: serde::de::DeserializeOwned,
    {
        let url = self.build_api_url(path);
        let req_builder = self.http_client.request(method, &url);
        self.process_request(req_builder)
    }

    /// Execute a multipart API request
    pub fn execute_multipart_request<Res>(
        &self,
        method: reqwest::Method,
        path: &str,
        form: reqwest::blocking::multipart::Form,
    ) -> Result<Res>
    where
        Res: serde::de::DeserializeOwned,
    {
        let url = self.build_api_url(path);
        let req_builder = self.http_client.request(method, &url).multipart(form);
        self.process_request(req_builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_client_creation() {
        let config = Config::default();
        let client = Client::new(config.clone()).unwrap();
        assert_eq!(client.config.base_url, config.base_url);
    }

    #[test]
    fn test_url_validation() {
        let client = Client::with_default_config().unwrap();

        assert!(client.is_valid_url("https://example.com"));
        assert!(client.is_valid_url("http://example.com"));
        assert!(!client.is_valid_url("ftp://example.com"));
        assert!(!client.is_valid_url("not-a-url"));
        assert!(!client.is_valid_url(""));
    }

    #[test]
    fn test_api_url_building() {
        let config = Config::new("https://api.example.com");
        let client = Client::new(config).unwrap();

        assert_eq!(
            client.build_api_url("/test"),
            "https://api.example.com/test"
        );
    }
}
