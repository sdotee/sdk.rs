/*!
 * Copyright (c) 2026 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: files.rs
 * Author: S.EE Development Team <dev@s.ee>
 * File Created: 2026-01-19 23:34:24
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2026-01-19 23:49:47
 */

use see_sdk::client::Client;
use see_sdk::config::Config;
use see_sdk::file::FileService;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = match env::var("SEE_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: SEE_API_KEY environment variable not set.");
            eprintln!("Please set it to run this example.");
            return Ok(());
        }
    };

    let config = Config::default().with_api_key(&api_key);
    let client = Client::new(config)?;

    // Create a dummy file for testing
    let file_path = "test_image.txt";
    let mut file = File::create(file_path)?;
    writeln!(file, "This is a test file for upload")?;

    println!("Uploading file: {}", file_path);
    let upload_resp = client.upload_file(file_path)?;
    println!("Upload success. File URL: {}", upload_resp.data.url);
    println!("File Hash: {}", upload_resp.data.hash);

    println!("Fetching file domains...");
    let domains = client.get_file_domains()?;
    println!("File domains: {:?}", domains.data.domains);

    println!("Deleting file: {}", upload_resp.data.hash);
    let delete_resp = client.delete_file(&upload_resp.data.hash)?;
    println!("Delete success: {}", delete_resp.success);

    // Cleanup
    if Path::new(file_path).exists() {
        std::fs::remove_file(file_path)?;
    }

    Ok(())
}
