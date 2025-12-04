/*!
 * Copyright (c) 2025 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: basic.rs
 * Author: S.EE Development Team <dev@s.ee>
 * File Created: 2025-10-23 15:16:01
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2025-12-04 19:25:02
 */

use see_sdk::client::Client;
use see_sdk::config::Config;
use see_sdk::url::ShortenService;
use see_sdk::url::builder::UrlShortenerRequestBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with default configuration
    let config = Config::default().with_api_key("your-api-key-here");

    let client = Client::new(config)?;

    // Shorten a simple URL
    let request = UrlShortenerRequestBuilder::new("https://www.example.com/very/long/url")?.build();

    let response = client.shorten(request)?;

    println!("Original URL: https://www.example.com/very/long/url");
    println!("Shortened URL: {}", response.data.short_url);
    println!("Slug: {}", response.data.slug);

    Ok(())
}
