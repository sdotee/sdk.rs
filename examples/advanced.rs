use see_rust_sdk::client::Client;
use see_rust_sdk::config::Config;
use see_rust_sdk::domain::DomainService;
use see_rust_sdk::tag::TagService;
use see_rust_sdk::url::ShortenService;
use see_rust_sdk::url::builder::UrlShortenerRequestBuilder;

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
        .with_expiration("2025-12-31T23:59:59Z")
        .build();

    let response = client.shorten(request)?;

    println!("✓ URL shortened successfully!");
    println!("  Short URL: {}", response.data.short_url);
    println!("  Custom Slug: {}", response.data.custom_slug);
    println!("  Message: {}", response.message);

    Ok(())
}
