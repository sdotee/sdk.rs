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
    pub expire_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<u32>>,
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

/// Request structure for updating a short URL
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateShortURLRequest {
    pub domain: String,
    pub slug: String,
    pub target_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
    pub custom_slug: Option<String>,
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
    pub data: Option<serde_json::Value>,
}

/// Request structure for link visit statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetLinkVisitStatRequest {
    pub domain: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Data structure for link visit statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkVisitStatData {
    pub visit_count: i64,
}

/// Response structure for link visit statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetLinkVisitStatResponse {
    pub code: u16,
    pub message: String,
    pub data: LinkVisitStatData,
}

/// Data structure for available domains
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainsData {
    pub domains: Vec<String>,
}

/// Response structure for available domains
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAvailableDomainsResponse {
    pub code: u16,
    pub message: String,
    pub data: DomainsData,
}
