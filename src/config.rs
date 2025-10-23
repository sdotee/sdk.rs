/*!
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co., Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: config.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-23 11:28:59
 *
 * Modified By: mingcheng <mingcheng@apache.org>
 * Last Modified: 2025-10-23 13:11:42
 */

use std::time::Duration;

/// Default base URL for the API
pub const DEFAULT_BASE_URL: &str = "https://s.ee";

/// Default user agent string
pub const DEFAULT_USER_AGENT: &str = concat!("see-rust-sdk/", env!("CARGO_PKG_VERSION"));

/// Default timeout in seconds
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default maximum retry attempts
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Default domain for shortened URLs
pub const DEFAULT_DOMAIN: &str = "s.ee";

/// Client configuration for URL shortener
#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout: Duration,
    pub user_agent: String,
    pub max_retries: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }
}

impl Config {
    /// Create a new client configuration with a custom base URL
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            ..Default::default()
        }
    }

    /// Set the API key for authentication
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set a custom user agent
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Set the maximum number of retry attempts
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.base_url, DEFAULT_BASE_URL);
        assert_eq!(config.timeout, Duration::from_secs(DEFAULT_TIMEOUT_SECS));
        assert_eq!(config.user_agent, DEFAULT_USER_AGENT);
        assert_eq!(config.max_retries, DEFAULT_MAX_RETRIES);
        assert!(config.api_key.is_none());
    }

    #[test]
    fn test_custom_config() {
        let config = Config::new("https://custom.api")
            .with_api_key("my_api_key")
            .with_timeout(Duration::from_secs(60))
            .with_user_agent("custom-agent/1.0");

        assert_eq!(config.base_url, "https://custom.api");
        assert_eq!(config.api_key.unwrap(), "my_api_key");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.user_agent, "custom-agent/1.0");
    }
}
