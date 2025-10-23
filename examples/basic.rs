use see_rust_sdk::client::Client;
use see_rust_sdk::config::Config;
use see_rust_sdk::url::builder::UrlShortenerRequestBuilder;
use see_rust_sdk::url::ShortenService;

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
