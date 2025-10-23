/*!
 * Copyright (c) 2025 Hangzhou Guanwaii Technology Co., Ltd.
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: builder.rs
 * Author: mingcheng <mingcheng@apache.org>
 * File Created: 2025-10-23 11:30:17
 *
 * Modified By: mingcheng <mingcheng@apache.org>
 * Last Modified: 2025-10-23 17:31:41
 */

use std::error::Error;

use url::Url;

use crate::url::models::ShortenRequest;

pub struct UrlShortenerRequestBuilder {
    data: ShortenRequest,
}

impl UrlShortenerRequestBuilder {
    pub fn new(url: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        let url = Url::parse(&url.into())?;

        Ok(Self {
            data: ShortenRequest {
                target_url: url.into(),
                ..Default::default()
            },
        })
    }

    pub fn with_custom_alias(mut self, alias: impl Into<String>) -> Self {
        self.data.custom_slug = Some(alias.into());
        self
    }

    pub fn with_expiration(mut self, expiration: impl Into<String>) -> Self {
        self.data.expire_at = Some(expiration.into());
        self
    }

    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.data.domain = domain.into();
        self
    }

    pub fn build(self) -> ShortenRequest {
        self.data
    }
}

#[cfg(test)]
mod test {
    use crate::url::builder::UrlShortenerRequestBuilder;

    #[test]
    fn test_url_shortener_request_builder() {
        let request = UrlShortenerRequestBuilder::new("https://example.com/")
            .unwrap()
            .with_custom_alias("my-alias")
            .build();

        assert_eq!(request.target_url, "https://example.com/");
        assert_eq!(request.custom_slug.unwrap(), "my-alias");
    }
}
