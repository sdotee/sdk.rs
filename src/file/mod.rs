/*!
 * Copyright (c) 2026 S.EE Development Team
 *
 * This source code is licensed under the MIT License,
 * which is located in the LICENSE file in the source tree's root directory.
 *
 * File: mod.rs
 * Author: S.EE Development Team <dev@s.ee>
 * File Created: 2026-01-19 23:32:47
 *
 * Modified By: S.EE Development Team <dev@s.ee>
 * Last Modified: 2026-01-20 10:55:11
 */

use crate::client::Client;
use crate::error::{Error, Result};
use crate::file::models::{FileDeleteResponse, FileDomainsResponse, FileUploadResponse};
use reqwest::blocking::multipart::Form;
use std::path::Path;

pub mod models;

pub trait FileService {
    /// Upload a file for sharing
    fn upload_file<P: AsRef<Path>>(&self, file_path: P) -> Result<FileUploadResponse>;

    /// Delete a shared file
    fn delete_file(&self, key: &str) -> Result<FileDeleteResponse>;

    /// Get available domains for file sharing
    fn get_file_domains(&self) -> Result<FileDomainsResponse>;
}

impl FileService for Client {
    /// Upload a file to the service
    fn upload_file<P: AsRef<Path>>(&self, file_path: P) -> Result<FileUploadResponse> {
        let path = file_path.as_ref();

        let form = Form::new().file("file", path).map_err(Error::IoError)?;

        self.execute_multipart_request(reqwest::Method::POST, "/file/upload", form)
    }

    /// Delete a file by key
    fn delete_file(&self, key: &str) -> Result<FileDeleteResponse> {
        let path = format!("/file/delete/{}", key);
        self.execute_request_no_body(reqwest::Method::GET, &path)
    }

    /// Get available file domains
    fn get_file_domains(&self) -> Result<FileDomainsResponse> {
        self.execute_request_no_body(reqwest::Method::GET, "/file/domains")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::helpers::{create_test_client, get_api_key_or_skip};
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_file_upload_and_delete() {
        if get_api_key_or_skip().is_none() {
            return;
        }

        let client = create_test_client().unwrap();

        // Create a temporary file manually
        let mut temp_path = env::temp_dir();
        temp_path.push("test_upload_file.txt");

        let mut file = File::create(&temp_path).unwrap();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        writeln!(file, "Hello, world! - {}", timestamp).unwrap();

        // 1. Test Upload
        let upload_response = client.upload_file(&temp_path).unwrap();
        assert_eq!(upload_response.code, 200);
        assert!(!upload_response.data.hash.is_empty());

        let file_key = &upload_response.data.hash;

        // 2. Test Get Domains
        let domains_response = client.get_file_domains().unwrap();
        assert_eq!(domains_response.code, 200);

        // 3. Test Delete
        let delete_response = client.delete_file(file_key).unwrap();
        assert!(delete_response.success);

        // Cleanup
        let _ = fs::remove_file(temp_path);
    }
}
