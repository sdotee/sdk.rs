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
 * Last Modified: 2026-01-19 23:50:26
 */

use serde::{Deserialize, Serialize};

/// Request structure for creating text sharing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTextRequest {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Data structure for created text
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTextData {
    pub custom_slug: String,
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
    pub content: String,
    pub domain: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
    pub domain: String,
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
