/*!
 * Copyright (c) 2026 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: models.rs
 * Author: S.EE Development Team <dev@s.ee>
 * File Created: 2026-01-19 23:32:10
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2026-01-19 23:50:06
 */

use serde::{Deserialize, Serialize};

/// Data structure for file information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileData {
    pub delete: String,
    pub file_id: u64,
    pub filename: String,
    pub hash: String,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub page: Option<String>,
    pub path: String,
    pub size: u64,
    pub storename: String,
    pub upload_status: i32,
    pub url: String,
    #[serde(default)]
    pub width: Option<u32>,
}

/// Response structure for file upload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileUploadResponse {
    pub code: i32,
    pub message: String,
    pub data: FileData,
}

/// Response structure for file deletion
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileDeleteResponse {
    pub code: serde_json::Value,
    pub message: String,
    pub success: bool,
}

/// Data structure for domains
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileDomainsData {
    pub domains: Vec<String>,
}

/// Response structure for file domains
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileDomainsResponse {
    pub code: i32,
    pub message: String,
    pub data: FileDomainsData,
}
