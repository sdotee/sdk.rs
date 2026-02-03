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
    /// Deletion key/URL
    pub delete: String,

    /// Unique file ID
    pub file_id: u64,

    /// Original filename
    pub filename: String,

    /// File hash/key
    pub hash: String,

    /// Image height (if applicable)
    #[serde(default)]
    pub height: Option<u32>,

    /// Page/extension info
    #[serde(default)]
    pub page: Option<String>,

    /// File path
    pub path: String,

    /// File size in bytes
    pub size: u64,

    /// Storage name
    pub storename: String,

    /// Upload status code
    pub upload_status: i32,

    /// Full URL to the file
    pub url: String,

    /// Image width (if applicable)
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
    pub code: String,
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
