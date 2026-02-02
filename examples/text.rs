/*!
 * Copyright (c) 2026 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: text.rs
 * Author: S.EE Development Team <dev@s.ee>
 * File Created: 2026-01-19 23:39:29
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2026-01-19 23:49:36
 */

use see_sdk::client::Client;
use see_sdk::config::{Config, DEFAULT_DOMAIN};
use see_sdk::text::TextService;
use see_sdk::text::models::{CreateTextRequest, DeleteTextRequest, UpdateTextRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize client
    let api_key = std::env::var("SEE_API_KEY").expect("SEE_API_KEY must be set");
    let config = Config::default().with_api_key(&api_key);
    let client = Client::new(config)?;

    // 2. Create a text snippet
    println!("Creating text snippet...");
    let create_req = CreateTextRequest {
        content: "This is a text snippet created via Rust SDK.".to_string(),
        title: "My Rust Snippet".to_string(),
        domain: None,
        custom_slug: None,
        expire_at: None,
        password: None,
        tag_ids: None,
        text_type: None,
    };

    let create_resp = client.create_text(create_req)?;
    println!("Created! Short URL: {}", create_resp.data.short_url);
    println!("Slug: {}", create_resp.data.slug);

    let slug = create_resp.data.slug;
    let domain = DEFAULT_DOMAIN.to_string(); // In real usage, you might want to parse it from short_url if needed

    // 3. Get available domains
    println!("\nFetching text domains...");
    let domains = client.get_text_domains()?;
    println!("Available domains: {:?}", domains.data.domains);

    // 4. Update the text snippet
    println!("\nUpdating text snippet...");
    let update_req = UpdateTextRequest {
        content: "This content has been updated.".to_string(),
        domain: domain.clone(),
        slug: slug.clone(),
        title: "Updated Rust Snippet".to_string(),
    };

    client.update_text(update_req)?;
    println!("Text snippet updated successfully.");

    // 5. Delete the text snippet
    println!("\nDeleting text snippet...");
    let delete_req = DeleteTextRequest { domain, slug };

    client.delete_text(delete_req)?;
    println!("Text snippet deleted successfully.");

    Ok(())
}
