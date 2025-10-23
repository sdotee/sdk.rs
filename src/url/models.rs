/*!
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co., Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: models.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-23 11:23:53
 *
 * Modified By: mingcheng <mingcheng@apache.org>
 * Last Modified: 2025-10-23 21:52:31
 */

use std::str;

use crate::config::DEFAULT_DOMAIN;
use serde::{Deserialize, Serialize};

/// Request structure for URL shortening
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortenRequest {
    pub target_url: String,
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}

impl Default for ShortenRequest {
    fn default() -> Self {
        Self {
            target_url: String::new(),
            domain: DEFAULT_DOMAIN.to_string(),
            title: None,
            custom_slug: None,
            expiration_redirect_url: None,
            expire_at: None,
            password: None,
            tag_ids: None,
        }
    }
}

/// Response structure for URL shortening
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortenResponse {
    pub code: u16,
    pub message: String,
    pub data: ShortenData,
}

/// Data structure containing shortened URL information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortenData {
    pub custom_slug: String,
    pub short_url: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteRequest {
    pub domain: String,
    pub slug: String,
}

// Response type for URL deletion
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteResponse {
    pub code: u16,
    pub message: String,
    pub data: Option<String>,
}
