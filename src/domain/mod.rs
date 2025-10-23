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
        self.execute_request(reqwest::Method::GET, "/api/v1/domains", ())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::Config;
    use log::{info, warn};
    use reqwest::StatusCode;
    use std::env;
    use std::result::Result;

    #[test]
    fn test_list_domain() -> Result<(), Box<dyn std::error::Error>> {
        let api_key = env::var("URL_SHORTENER_API_KEY").unwrap_or_else(|_| "".to_string());

        if api_key.is_empty() {
            warn!("Skipping test_shorten_url: URL_SHORTENER_API_KEY not set");
            return Ok(());
        }

        let config = Config::default().with_api_key(&api_key);
        let client = Client::new(config)?;

        let response = client.list()?;

        assert_eq!(response.code, StatusCode::OK);
        response.data.domains.iter().for_each(|domain| {
            info!("Domain: {}", domain);
            assert!(!str::is_empty(domain));
        });

        Ok(())
    }
}
