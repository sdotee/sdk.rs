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
        self.execute_request(reqwest::Method::GET, "/tags", ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::helpers::{assert_status_ok, create_test_client, get_api_key_or_skip};

    #[test]
    fn test_list_tags() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Skip test if API key is not set
        if get_api_key_or_skip().is_none() {
            return Ok(());
        }

        let client = create_test_client()?;
        let response = client.list()?;

        assert_status_ok(response.code);
        assert!(!response.data.tags.is_empty(), "Expected at least one tag");

        // Verify each tag has valid data
        for tag in &response.data.tags {
            assert!(!tag.name.is_empty(), "Tag name should not be empty");
            assert_ne!(tag.id, 0, "Tag ID should not be zero");
        }

        Ok(())
    }
}
