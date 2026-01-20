/*!
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co., Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: mod.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-23 22:13:53
 *
 * Modified By: mingcheng <mingcheng@apache.org>
 * Last Modified: 2025-10-24 00:20:59
 */

use std::str;

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub trait DomainService {
    fn list(&self) -> Result<DomainListResponse>;
}

type Domain = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainListData {
    pub domains: Vec<Domain>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainListResponse {
    pub code: u16,
    pub message: String,
    pub data: DomainListData,
}

impl DomainService for Client {
    fn list(&self) -> Result<DomainListResponse> {
        self.execute_request(reqwest::Method::GET, "/domains", ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::helpers::{assert_status_ok, create_test_client, get_api_key_or_skip};

    #[test]
    fn test_list_domains() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Skip test if API key is not set
        if get_api_key_or_skip().is_none() {
            return Ok(());
        }

        let client = create_test_client()?;
        let response = client.list()?;

        assert_status_ok(response.code);
        assert!(
            !response.data.domains.is_empty(),
            "Expected at least one domain"
        );

        // Verify each domain is a valid non-empty string
        for domain in &response.data.domains {
            assert!(!domain.is_empty(), "Domain should not be empty");
        }

        Ok(())
    }
}
