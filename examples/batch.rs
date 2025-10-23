use std::{thread, time::Duration};

use see_rust_sdk::client::Client;
use see_rust_sdk::config::Config;
use see_rust_sdk::error::Error;
use see_rust_sdk::url::builder::UrlShortenerRequestBuilder;
use see_rust_sdk::url::models::ShortenResponse;
use see_rust_sdk::url::ShortenService;

/// Helper function to shorten a URL with retry logic for rate limiting
fn shorten_with_retry(
    client: &Client,
    url: &str,
    max_retries: u32,
) -> Result<ShortenResponse, Box<dyn std::error::Error>> {
    let mut attempts = 0;

    loop {
        attempts += 1;
        let request = UrlShortenerRequestBuilder::new(url)?.build();

        match client.shorten(request) {
            Ok(response) => return Ok(response),
            Err(Error::RateLimited) if attempts < max_retries => {
                println!("  Rate limited, retrying in {} seconds...", attempts);
                thread::sleep(Duration::from_secs(attempts as u64));
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default().with_api_key("your-api-key-here");
    let client = Client::new(config)?;

    let urls_to_shorten = vec![
        "https://www.example1.com/page",
        "https://www.example2.com/article",
        "https://www.example3.com/product",
    ];

    println!("Shortening {} URLs...\n", urls_to_shorten.len());

    let mut successful = 0;
    let mut failed = 0;

    for url in urls_to_shorten {
        print!("Shortening {}... ", url);
        match shorten_with_retry(&client, url, 3) {
            Ok(response) => {
                println!("✓ {}", response.data.short_url);
                successful += 1;
            }
            Err(e) => {
                println!("✗ Error: {}", e);
                failed += 1;
            }
        }
    }

    println!("\nSummary:");
    println!("  Successful: {}", successful);
    println!("  Failed: {}", failed);

    Ok(())
}
