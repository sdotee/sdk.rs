/*!
 * Copyright (c) 2025-2026 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: lib.rs
 * Author: S.EE Development Team <dev@s.ee>
 * File Created: 2025-10-23 11:07:28
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2026-01-19 23:44:40
 */

//! A Rust SDK for content sharing services (URL, Text, File).
//!
//! This crate provides a simple and elegant interface for content sharing
//! using the s.ee service.
//!
//! # Example
//!
//! ```no_run
//! use see_sdk::client::Client;
//! use see_sdk::config::Config;
//! use see_sdk::url::builder::UrlShortenerRequestBuilder;
//! use see_sdk::url::ShortenService;
//!
//! let config = Config::default().with_api_key("your-api-key");
//! let client = Client::new(config).unwrap();
//!
//! let request = UrlShortenerRequestBuilder::new("https://example.com")
//!     .unwrap()
//!     .build();
//!
//! let response = client.shorten(request).unwrap();
//! println!("Shortened URL: {}", response.data.short_url);
//! ```

pub mod client;

pub mod error;

pub mod url;

pub mod config;

pub mod tag;

pub mod domain;

pub mod file;

pub mod text;

#[cfg(test)]
mod test_helpers;
