/*!
 * Copyright (c) 2026 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: mod.rs
 * Author: S.EE Development Team <dev@s.ee>
 * File Created: 2026-01-19 23:39:17
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2026-01-20 00:01:09
 */

use crate::client::Client;
use crate::error::Result;
use crate::text::models::{
    CreateTextRequest, CreateTextResponse, DeleteTextRequest, DeleteTextResponse,
    TextDomainsResponse, UpdateTextRequest, UpdateTextResponse,
};

pub mod models;

pub trait TextService {
    /// Create a new text sharing entry
    fn create_text(&self, request: CreateTextRequest) -> Result<CreateTextResponse>;

    /// Update an existing text sharing entry
    fn update_text(&self, request: UpdateTextRequest) -> Result<UpdateTextResponse>;

    /// Delete a text sharing entry
    fn delete_text(&self, request: DeleteTextRequest) -> Result<DeleteTextResponse>;

    /// Get available domains for text sharing
    fn get_text_domains(&self) -> Result<TextDomainsResponse>;
}

impl TextService for Client {
    /// Create a new text sharing
    fn create_text(&self, request: CreateTextRequest) -> Result<CreateTextResponse> {
        self.execute_request(reqwest::Method::POST, "/text", request)
    }

    /// Update an existing text sharing
    fn update_text(&self, request: UpdateTextRequest) -> Result<UpdateTextResponse> {
        self.execute_request(reqwest::Method::PUT, "/text", request)
    }

    /// Delete a text sharing
    fn delete_text(&self, request: DeleteTextRequest) -> Result<DeleteTextResponse> {
        self.execute_request(reqwest::Method::DELETE, "/text", request)
    }

    /// Get available text domains
    fn get_text_domains(&self) -> Result<TextDomainsResponse> {
        self.execute_request_no_body(reqwest::Method::GET, "/text/domains")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DEFAULT_DOMAIN;
    use crate::test_helpers::helpers::{create_test_client, get_api_key_or_skip};

    #[test]
    fn test_text_lifecycle() {
        if get_api_key_or_skip().is_none() {
            return;
        }

        let client = create_test_client().unwrap();

        // 1. Create Text
        let create_req = CreateTextRequest {
            content: "Hello from Rust SDK".to_string(),
            title: "Rust SDK Test".to_string(),
            domain: None,
            custom_slug: None,
            expire_at: None,
            password: None,
            tag_ids: None,
            text_type: None,
        };

        let create_resp = client.create_text(create_req).unwrap();
        // The API seems to return 200 for success instead of 0 as documented
        assert_eq!(create_resp.code, 200);
        assert!(!create_resp.data.short_url.is_empty());

        let slug = create_resp.data.slug.clone();
        // Assuming default domain is used if not specified,
        // or we need to extract domain from response if it was dynamic.
        // But response only gives short_url. We'll use DEFAULT_DOMAIN or parsed domain.
        // For simpler test, let's assume we use the domain from short_url or DEFAULT_DOMAIN.
        // The API might return the domain used, but here we only have short_url.
        // Let's assume standard domain "s.ee" or whatever is configured.
        let domain = create_resp
            .data
            .short_url
            .split('/')
            .nth(2)
            .unwrap_or(DEFAULT_DOMAIN)
            .to_string();

        // 2. Update Text
        let update_req = UpdateTextRequest {
            content: "Updated content from Rust SDK".to_string(),
            domain: domain.clone(),
            slug: slug.clone(),
            title: "Rust SDK Updated".to_string(),
        };

        let update_resp = client.update_text(update_req).unwrap();
        assert!(update_resp.code == 0 || update_resp.code == 200);

        // 3. Get Domains
        let domains_resp = client.get_text_domains().unwrap();
        assert_eq!(domains_resp.code, 200);
        assert!(!domains_resp.data.domains.is_empty());

        // 4. Delete Text
        let delete_req = DeleteTextRequest { domain, slug };
        let delete_resp = client.delete_text(delete_req).unwrap();
        assert_eq!(delete_resp.code, 200);
    }
}
