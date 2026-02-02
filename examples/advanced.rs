/*!
 * Copyright (c) 2025 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: advanced.rs
 * Author: S.EE Development Team <dev@s.ee>
 * File Created: 2025-10-23 15:16:03
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2025-12-04 19:24:58
 */

use see_sdk::client::Client;
use see_sdk::config::Config;
use see_sdk::domain::DomainService;
use see_sdk::tag::TagService;
use see_sdk::url::ShortenService;
use see_sdk::url::builder::UrlShortenerRequestBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default().with_api_key("your-api-key-here");
    let client = Client::new(config)?;

    // List available tags
    println!("📋 Fetching available tags...");
    match TagService::list(&client) {
        Ok(tag_response) => {
            println!("✓ Available tags:");
            for tag in &tag_response.data.tags {
                println!("  - {} (ID: {})", tag.name, tag.id);
            }
        }
        Err(e) => println!("  ⚠ Failed to fetch tags: {}", e),
    }

    // List available domains
    println!("\n🌐 Fetching available domains...");
    match DomainService::list(&client) {
        Ok(domain_response) => {
            println!("✓ Available domains:");
            for domain in &domain_response.data.domains {
                println!("  - {}", domain);
            }
        }
        Err(e) => println!("  ⚠ Failed to fetch domains: {}", e),
    }

    // Create a URL with all available options
    println!("\n🔗 Creating shortened URL with custom options...");
    let request = UrlShortenerRequestBuilder::new("https://www.example.com/product/123")?
        .with_custom_alias("summer-sale")
        .with_expiration(1830297599) // 2027-12-31T23:59:59Z
        .build();

    let response = client.shorten(request)?;

    println!("✓ URL shortened successfully!");
    println!("  Short URL: {}", response.data.short_url);
    if let Some(slug) = &response.data.custom_slug {
        println!("  Custom Slug: {}", slug);
    }
    println!("  Message: {}", response.message);

    Ok(())
}
