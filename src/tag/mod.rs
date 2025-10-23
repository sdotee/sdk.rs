/*!
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co., Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: mod.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-23 17:25:13
 *
 * Modified By: mingcheng <mingcheng@apache.org>
 * Last Modified: 2025-10-24 00:17:26
 */

use std::str;

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: u32,
    pub name: String,
}

pub trait TagService {
    fn list(&self) -> Result<TagListResponse>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagListData {
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagListResponse {
    pub code: u16,
    pub message: String,
    pub data: TagListData,
}

impl TagService for Client {
    fn list(&self) -> Result<TagListResponse> {
        self.execute_request(reqwest::Method::GET, "/api/v1/tags", ())
    }
}

#[cfg(test)]
mod test {
    use crate::{client::Client, config::Config, tag::TagService};
    use log::{info, warn};
    use reqwest::StatusCode;
    use std::env;

    #[test]
    fn test_list_tag() -> Result<(), Box<dyn std::error::Error>> {
        let api_key = env::var("URL_SHORTENER_API_KEY").unwrap_or_else(|_| "".to_string());

        if api_key.is_empty() {
            warn!("Skipping test_shorten_url: URL_SHORTENER_API_KEY not set");
            return Ok(());
        }

        let config = Config::default().with_api_key(&api_key);
        let client = Client::new(config)?;

        let response = client.list()?;

        assert_eq!(response.code, StatusCode::OK);
        response.data.tags.iter().for_each(|tag| {
            info!("Tag ID: {}, Name: {}", tag.id, tag.name);
            assert!(!tag.name.is_empty());
            assert_ne!(!tag.id, 0);
        });

        Ok(())
    }
}
