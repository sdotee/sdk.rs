/*!
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co., Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: error.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-23 11:21:39
 *
 * Modified By: mingcheng <mingcheng@apache.org>
 * Last Modified: 2025-10-23 22:37:59
 */

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error response from the API
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i16,
    pub message: String,
    pub data: String,
}

/// Error types for content sharing operations
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Invalid URL format provided
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),

    /// URL parsing error
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// Server returned an error
    #[error("Server error: {status} - {message}")]
    ServerError { status: u16, message: String },

    /// JSON parsing failed
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// URL not found
    #[error("URL not found")]
    NotFound,

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimited,

    /// IO Error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Generic error
    #[error("Error: {0}")]
    GenericError(String),
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::GenericError(err.to_string())
    }
}

/// Result type alias for content sharing operations
pub type Result<T> = std::result::Result<T, Error>;
