/*!
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co., Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: lib.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-23 11:07:28
 *
 * Modified By: mingcheng <mingcheng@apache.org>
 * Last Modified: 2025-10-23 22:38:06
 */

//! A Rust SDK for URL shortening services.
//!
//! This crate provides a simple and elegant interface for shortening URLs
//! using the s.ee service.
//!
//! # Example
//!
//! ```no_run
//! use see_rust_sdk::client::Client;
//! use see_rust_sdk::config::Config;
//! use see_rust_sdk::url::builder::UrlShortenerRequestBuilder;
//! use see_rust_sdk::url::ShortenService;
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
