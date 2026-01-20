/*!
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co., Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: test_helpers.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-24 07:27:39
 *
 * Modified By: mingcheng <mingcheng@apache.org>
 * Last Modified: 2025-10-24 07:30:54
 */

#[cfg(test)]
pub mod helpers {
    use crate::{client::Client, config::Config, error::Result};
    use std::env;

    /// Get API key from environment variable, skip test if not set
    pub fn get_api_key_or_skip() -> Option<String> {
        match env::var("SEE_API_KEY") {
            Ok(key) if !key.is_empty() => Some(key),
            _ => {
                log::warn!("Skipping test: SEE_API_KEY not set");
                None
            }
        }
    }

    /// Create a test client with default configuration and API key from environment
    pub fn create_test_client() -> Result<Client> {
        let api_key = get_api_key_or_skip()
            .ok_or_else(|| crate::error::Error::InvalidUrl("No API key".to_string()))?;

        let config = Config::default().with_api_key(&api_key);
        Client::new(config)
    }

    /// Assert that a response code matches the expected status
    pub fn assert_status_ok(code: u16) {
        assert_eq!(code, reqwest::StatusCode::OK.as_u16());
    }

    /// Assert that a string is not empty
    pub fn assert_not_empty(value: &str) {
        assert!(!value.is_empty(), "Expected non-empty string");
    }
}
