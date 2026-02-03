/*!
 * Copyright (c) 2026 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: models.rs
 * Author: S.EE Development Team <dev@s.ee>
 * File Created: 2026-01-19 23:39:03
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2026-02-03 10:32:25
 */

use serde::{Deserialize, Serialize};

/// Request structure for creating text sharing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTextRequest {
    /// The text content to share
    pub content: String,

    /// Title for the text snippets
    pub title: String,

    /// Optional specific domain to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Optional custom slug
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_slug: Option<String>,

    /// Optional expiration timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<i64>,

    /// Optional password for protection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Optional tag IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<u32>>,

    /// Optional text type (e.g., syntax highlighting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
}

/// Data structure for created text
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTextData {
    pub custom_slug: Option<String>,
    pub short_url: String,
    pub slug: String,
}

/// Response for creating text sharing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTextResponse {
    pub code: i32,
    pub message: String,
    pub data: CreateTextData,
}

/// Request structure for updating text sharing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTextRequest {
    /// The new content
    pub content: String,

    /// The domain of the shared text
    pub domain: String,

    /// The slug of the shared text
    pub slug: String,

    /// The new title
    pub title: String,
}

/// Response for updating text sharing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTextResponse {
    pub code: i32,
    pub message: String,
    pub data: serde_json::Value,
}

/// Request structure for deleting text sharing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteTextRequest {
    /// The domain of the shared text
    pub domain: String,

    /// The slug of the shared text to delete
    pub slug: String,
}

/// Response for deleting text sharing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteTextResponse {
    pub code: i32,
    pub message: String,
    pub data: serde_json::Value,
}

/// Data structure for text domains
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextDomainsData {
    pub domains: Vec<String>,
}

/// Response for getting text domains
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextDomainsResponse {
    pub code: i32,
    pub message: String,
    pub data: TextDomainsData,
}
