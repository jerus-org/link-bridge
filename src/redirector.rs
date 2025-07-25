//! URL redirection system for generating short links and HTML redirect pages.
//!
//! This module provides the core functionality for creating URL redirects by:
//! - Validating and normalizing URL paths
//! - Generating unique short file names using base62 encoding
//! - Creating HTML redirect pages with meta refresh and JavaScript fallbacks
//! - Writing redirect files to the filesystem
//! - Managing a registry system to prevent duplicate redirects
//!
//! # Example Usage
//!
//! ```rust
//! use link_bridge::Redirector;
//! use std::fs;
//!
//! // Create a redirector for a URL path
//! let mut redirector = Redirector::new("api/v1/users").unwrap();
//!
//! // Optionally set a custom output directory
//! redirector.set_path("doc_test_output");
//!
//! // Write the redirect HTML file
//! let redirect_path = redirector.write_redirect().unwrap();
//!
//! // Clean up test files
//! fs::remove_dir_all("doc_test_output").ok();
//! ```

mod url_path;

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fmt, fs};
use thiserror::Error;

use chrono::Utc;

use crate::redirector::url_path::UrlPath;

/// Errors that can occur during redirect operations.
#[derive(Debug, Error)]
pub enum RedirectorError {
    /// An I/O error occurred while creating or writing redirect files.
    ///
    /// This includes errors like permission denied, disk full, or invalid file paths.
    #[error("Failed to create redirect file")]
    FileCreationError(#[from] std::io::Error),

    /// The short link has not been generated (should not occur in normal usage).
    ///
    /// This error is included for completeness but should not happen since
    /// short links are automatically generated during `Redirector::new()`.
    #[error("Short link not found")]
    ShortLinkNotFound,

    /// The provided URL path is invalid.
    ///
    /// This occurs when the path contains invalid characters like query parameters (?),
    /// semicolons (;), or other forbidden characters.
    #[error("Invalid URL path: {0}")]
    InvalidUrlPath(#[from] url_path::UrlPathError),

    /// An error occurred while reading or writing the redirect registry.
    ///
    /// This occurs when the `registry.json` file cannot be read, parsed, or written.
    /// Common causes include corrupted JSON, permission issues, or filesystem errors.
    #[error("Failed to read redirect registry")]
    FailedToReadRegistry(#[from] serde_json::Error),
}

/// Manages URL redirection by generating short links and HTML redirect pages.
///
/// The `Redirector` creates HTML files that automatically redirect users to longer URLs
/// on your website. It handles the entire process from URL validation to file generation.
///
/// # Key Features
///
/// - **URL Validation**: Ensures paths contain only valid characters
/// - **Unique Naming**: Generates unique file names using base62 encoding and timestamps
/// - **HTML Generation**: Creates complete HTML pages with meta refresh and JavaScript fallbacks
/// - **File Management**: Handles directory creation and file writing operations
/// - **Registry System**: Maintains a JSON registry to track existing redirects and prevent duplicates
///
/// # Short Link Generation
///
/// Short file names are generated using:
/// - Current timestamp in milliseconds
/// - Sum of UTF-16 code units from the URL path
/// - Base62 encoding for compact, URL-safe names
/// - `.html` extension for web server compatibility
///
/// # Registry System
///
/// The redirector maintains a `registry.json` file in each output directory that tracks:
/// - Mapping from URL paths to their corresponding redirect files
/// - Prevents duplicate files for the same URL path
/// - Ensures consistent redirect behaviour across multiple calls
/// - Automatically created and updated when redirects are written
///
/// # HTML Output
///
/// Generated HTML files include:
/// - Meta refresh tag for immediate redirection
/// - JavaScript fallback for better compatibility
/// - User-friendly link for manual navigation
/// - Proper HTML5 structure and encoding
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Redirector {
    /// The validated and normalized URL path to redirect to.
    long_path: UrlPath,
    /// The generated short file name (including .html extension).
    short_file_name: OsString,
    /// The directory path where redirect HTML files will be stored.
    path: PathBuf,
}

impl Redirector {
    /// Creates a new `Redirector` instance for the specified URL path.
    ///
    /// Validates the provided path and automatically generates a unique short file name.
    /// The redirector is initialized with a default output directory of "s".
    ///
    /// # Arguments
    ///
    /// * `long_path` - The URL path to create a redirect for (e.g., "api/v1/users")
    ///
    /// # Returns
    ///
    /// * `Ok(Redirector)` - A configured redirector ready to generate redirect files
    /// * `Err(RedirectorError::InvalidUrlPath)` - If the path contains invalid characters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use link_bridge::Redirector;
    ///
    /// // Valid paths
    /// let redirector1 = Redirector::new("api/v1").unwrap();
    /// let redirector2 = Redirector::new("/docs/getting-started/").unwrap();
    /// let redirector3 = Redirector::new("user-profile").unwrap();
    ///
    /// // Invalid paths (will return errors)
    /// assert!(Redirector::new("api?param=value").is_err()); // Query parameters
    /// assert!(Redirector::new("api;session=123").is_err());  // Semicolons
    /// assert!(Redirector::new("").is_err());                 // Empty string
    /// ```
    pub fn new<S: ToString>(long_path: S) -> Result<Self, RedirectorError> {
        let long_path = UrlPath::new(long_path.to_string())?;

        let short_file_name = Redirector::generate_short_file_name(&long_path);

        Ok(Redirector {
            long_path,
            short_file_name,
            path: PathBuf::from("s"),
        })
    }

    /// Generates a unique short file name based on timestamp and URL path content.
    ///
    /// Creates a unique identifier by combining the current timestamp with the URL path's
    /// UTF-16 character values, then encoding the result using base62 for a compact,
    /// URL-safe file name.
    ///
    /// # Algorithm
    ///
    /// 1. Get current timestamp in milliseconds
    /// 2. Sum all UTF-16 code units from the URL path
    /// 3. Add timestamp and UTF-16 sum together
    /// 4. Encode the result using base62 (0-9, A-Z, a-z)
    /// 5. Append ".html" extension
    ///
    /// # Returns
    ///
    /// An `OsString` containing the generated file name with `.html` extension.
    fn generate_short_file_name(long_path: &UrlPath) -> OsString {
        let name = base62::encode(
            Utc::now().timestamp_millis() as u64
                + long_path.encode_utf16().iter().sum::<u16>() as u64,
        );
        OsString::from(format!("{name}.html"))
    }

    /// Sets the output directory where redirect HTML files will be stored.
    ///
    /// By default, redirector uses "s" as the output directory. Use this method
    /// to specify a custom directory path. The directory will be created automatically
    /// when `write_redirect()` is called if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `path` - A path-like value (String, &str, PathBuf, etc.) specifying the directory
    ///
    /// # Examples
    ///
    /// ```rust
    /// use link_bridge::Redirector;
    ///
    /// let mut redirector = Redirector::new("api/v1").unwrap();
    ///
    /// // Set various types of paths
    /// redirector.set_path("redirects");           // &str
    /// redirector.set_path("output/html".to_string()); // String
    /// redirector.set_path(std::path::PathBuf::from("custom/path")); // PathBuf
    /// ```
    pub fn set_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.path = path.into();
    }

    /// Writes the redirect HTML file to the filesystem with registry support.
    ///
    /// Creates the output directory (if it doesn't exist) and generates a complete
    /// HTML redirect page that automatically redirects users to the target URL.
    /// The file name is the automatically generated short name with `.html` extension.
    ///
    /// # Registry System
    ///
    /// This method maintains a registry (`registry.json`) in the output directory to track
    /// existing redirects. If a redirect for the same URL path already exists, it returns
    /// the path to the existing file instead of creating a duplicate. This ensures:
    /// - No duplicate files for the same URL path
    /// - Consistent redirect behaviour across multiple calls
    /// - Efficient reuse of existing redirects
    ///
    /// # File Structure
    ///
    /// The generated HTML includes:
    /// - DOCTYPE and proper HTML5 structure
    /// - Meta charset and refresh tags for immediate redirection
    /// - JavaScript fallback for better browser compatibility
    /// - User-friendly fallback link for manual navigation
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The path to the created redirect file if successful
    /// * `Err(RedirectorError::FileCreationError)` - If file operations fail
    ///
    /// # Errors
    ///
    /// This method can return the following errors:
    ///
    /// ## `FileCreationError`
    /// - Permission denied (insufficient write permissions)
    /// - Disk full or insufficient space
    /// - Invalid characters in the file path
    /// - Parent directory cannot be created
    ///
    /// ## `FailedToReadRegistry`
    /// - Corrupted or invalid JSON in `registry.json`
    /// - Permission denied when reading/writing registry file
    /// - Registry file locked by another process
    ///
    /// # Examples
    ///
    /// ## Basic Usage
    ///
    /// ```rust
    /// use link_bridge::Redirector;
    /// use std::fs;
    ///
    /// let mut redirector = Redirector::new("api/v1/users").unwrap();
    /// redirector.set_path("doc_test_redirects");
    ///
    /// // First call creates a new redirect file and registry entry
    /// let redirect_path = redirector.write_redirect().unwrap();
    /// println!("Created redirect at: {}", redirect_path);
    ///
    /// // Clean up after the test
    /// fs::remove_dir_all("doc_test_redirects").ok();
    /// ```
    ///
    /// ## Registry behaviour
    ///
    /// ```rust
    /// use link_bridge::Redirector;
    /// use std::fs;
    ///
    /// let mut redirector1 = Redirector::new("api/v1/users").unwrap();
    /// redirector1.set_path("doc_test_registry");
    ///
    /// let mut redirector2 = Redirector::new("api/v1/users").unwrap();
    /// redirector2.set_path("doc_test_registry");
    ///
    /// // First call creates the file
    /// let path1 = redirector1.write_redirect().unwrap();
    ///
    /// // Second call returns the same path (no duplicate file created)
    /// let path2 = redirector2.write_redirect().unwrap();
    /// assert_eq!(path1, path2);
    ///
    /// // Clean up
    /// fs::remove_dir_all("doc_test_registry").ok();
    /// ```
    pub fn write_redirect(&self) -> Result<String, RedirectorError> {
        // create store directory if it doesn't exist
        if !Path::new(&self.path).exists() {
            fs::create_dir_all(&self.path)?;
        }
        const REDIRECT_REGISTRY: &str = "registry.json";
        let mut registry: HashMap<String, String> = HashMap::new();
        if Path::new(&self.path).join(REDIRECT_REGISTRY).exists() {
            registry = serde_json::from_reader::<_, HashMap<String, String>>(File::open(
                self.path.join(REDIRECT_REGISTRY),
            )?)?;
        }

        let file_path = self.path.join(&self.short_file_name);

        if let Some(existing_path) = registry.get(&self.long_path.to_string()) {
            // A link already exists for this path, return the existing file path
            Ok(existing_path.clone())
        } else {
            let file_name = file_path.to_string_lossy();
            let mut file = File::create(file_name.as_ref())?;

            file.write_all(self.to_string().as_bytes())?;
            file.sync_all()?;

            registry.insert(
                self.long_path.to_string(),
                file_path.to_string_lossy().to_string(),
            );

            serde_json::to_writer_pretty(
                File::create(self.path.join(REDIRECT_REGISTRY))?,
                &registry,
            )?;

            Ok(file_path.to_string_lossy().to_string())
        }
    }
}

impl fmt::Display for Redirector {
    /// Generates the complete HTML redirect page content.
    ///
    /// Creates a standard HTML5 page that redirects to the target URL using
    /// multiple methods for maximum compatibility:
    /// - Meta refresh tag (works in all browsers)
    /// - JavaScript redirect (faster, works when JS is enabled)
    /// - Fallback link (for manual navigation if automatic redirect fails)
    ///
    /// The HTML follows web standards and includes proper accessibility features.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target = self.long_path.to_string();
        write!(
            f,
            r#"
    <!DOCTYPE HTML>
    <html lang="en-US">

    <head>
        <meta charset="UTF-8">
        <meta http-equiv="refresh" content="0; url={target}">
        <script type="text/javascript">
            window.location.href = "{target}";
        </script>
        <title>Page Redirection</title>
    </head>

    <body>
        <!-- Note: don't tell people to `click` the link, just tell them that it is a link. -->
        If you are not redirected automatically, follow this <a href='{target}'>link to page</a>.
    </body>

    </html>
    "#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_redirector() {
        let long_link = "/some/path";
        let redirector = Redirector::new(long_link).unwrap();

        assert_eq!(
            redirector.long_path,
            UrlPath::new(long_link.to_string()).unwrap()
        );
        assert!(!redirector.short_file_name.is_empty());
        assert_eq!(redirector.path, PathBuf::from("s"));
    }

    #[test]
    fn test_generate_short_link_unique() {
        let redirector1 = Redirector::new("/some/path").unwrap();
        thread::sleep(Duration::from_millis(1));
        let redirector2 = Redirector::new("/some/path").unwrap();

        assert_ne!(redirector1.short_file_name, redirector2.short_file_name);
    }

    #[test]
    fn test_set_path() {
        let mut redirector = Redirector::new("/some/path/").unwrap();

        redirector.set_path("custom_path");
        assert_eq!(redirector.path, PathBuf::from("custom_path"));

        redirector.set_path("another/path".to_string());
        assert_eq!(redirector.path, PathBuf::from("another/path"));
    }

    #[test]
    fn test_display_renders_html() {
        let redirector = Redirector::new("some/path").unwrap();
        let output = format!("{redirector}");

        assert!(output.contains("<!DOCTYPE HTML>"));
        assert!(output.contains("/some/path/"));
        assert!(output.contains("meta http-equiv=\"refresh\""));
        assert!(output.contains("window.location.href"));
    }

    #[test]
    fn test_display_with_complex_path() {
        let redirector = Redirector::new("api/v2/users").unwrap();

        let output = format!("{redirector}");

        assert!(output.contains("<!DOCTYPE HTML>"));
        assert!(output.contains("/api/v2/users/"));
        assert!(output.contains("meta http-equiv=\"refresh\""));
        assert!(output.contains("window.location.href"));
    }

    #[test]
    fn test_write_redirect_with_valid_path() {
        let test_dir = format!(
            "test_write_redirect_with_valid_path_{}",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.set_path(&test_dir);

        let result = redirector.write_redirect();

        // Should succeed since short link is generated in new()
        assert!(result.is_ok());

        // Clean up
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_write_redirect_success() {
        let test_dir = format!(
            "test_write_redirect_success_{}",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.set_path(&test_dir);

        let result = redirector.write_redirect();
        assert!(result.is_ok());

        let file_path = result.unwrap();

        assert!(Path::new(&file_path).exists());

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("<!DOCTYPE HTML>"));
        assert!(content.contains("meta http-equiv=\"refresh\""));
        assert!(content.contains("window.location.href"));
        assert!(content.contains("If you are not redirected automatically"));
        assert!(content.contains("/some/path/"));

        // Clean up
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_write_redirect_creates_directory() {
        let test_dir = format!(
            "test_write_redirect_creates_directory_{}",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let subdir_path = format!("{test_dir}/subdir");
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.set_path(&subdir_path);

        assert!(!Path::new(&test_dir).exists());

        let result = redirector.write_redirect();
        assert!(result.is_ok());

        assert!(Path::new(&subdir_path).exists());

        let file_path = result.unwrap();
        assert!(Path::new(&file_path).exists());

        // Clean up
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_redirector_clone() {
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.set_path("custom");

        let cloned = redirector.clone();

        assert_eq!(redirector, cloned);
        assert_eq!(redirector.long_path, cloned.long_path);
        assert_eq!(redirector.short_file_name, cloned.short_file_name);
        assert_eq!(redirector.path, cloned.path);
    }

    #[test]
    fn test_redirector_default() {
        let redirector = Redirector::default();

        assert_eq!(redirector.long_path, UrlPath::default());
        assert_eq!(redirector.path, PathBuf::new());
        assert!(redirector.short_file_name.is_empty());
    }

    #[test]
    fn test_write_redirect_returns_correct_path() {
        let test_dir = format!(
            "test_write_redirect_returns_correct_path_{}",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.set_path(&test_dir);

        let result = redirector.write_redirect();
        assert!(result.is_ok());

        let returned_path = result.unwrap();
        let expected_path = redirector.path.join(&redirector.short_file_name);

        assert_eq!(returned_path, expected_path.to_string_lossy());
        assert!(Path::new(&returned_path).exists());

        // Clean up
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_write_redirect_registry_functionality() {
        let test_dir = format!(
            "test_write_redirect_registry_functionality_{}",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let mut redirector1 = Redirector::new("some/path").unwrap();
        redirector1.set_path(&test_dir);

        let mut redirector2 = Redirector::new("some/path").unwrap();
        redirector2.set_path(&test_dir);

        // First call should create a new file
        let result1 = redirector1.write_redirect();
        assert!(result1.is_ok());
        let path1 = result1.unwrap();

        // Second call with same path should return the existing file path
        let result2 = redirector2.write_redirect();
        assert!(result2.is_ok());
        let path2 = result2.unwrap();

        // Should return the same path
        assert_eq!(path1, path2);

        // Verify registry file exists
        let registry_path = PathBuf::from(&test_dir).join("registry.json");
        assert!(registry_path.exists());

        // Clean up
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_write_redirect_different_paths_different_files() {
        let test_dir = format!(
            "test_write_redirect_different_paths_different_files_{}",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let mut redirector1 = Redirector::new("some/path").unwrap();
        redirector1.set_path(&test_dir);

        let mut redirector2 = Redirector::new("other/path").unwrap();
        redirector2.set_path(&test_dir);

        let result1 = redirector1.write_redirect();
        assert!(result1.is_ok());
        let path1 = result1.unwrap();

        let result2 = redirector2.write_redirect();
        assert!(result2.is_ok());
        let path2 = result2.unwrap();

        // Should create different files for different paths
        assert_ne!(path1, path2);
        assert!(Path::new(&path1).exists());
        assert!(Path::new(&path2).exists());

        // Clean up
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_new_redirector_error_handling() {
        // Test invalid path - single segment should be okay now
        let result = Redirector::new("api");
        assert!(result.is_ok());

        // Test empty path
        let result = Redirector::new("");
        assert!(result.is_err());

        // Test invalid characters
        let result = Redirector::new("api?param=value");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_short_link_different_paths() {
        let redirector1 = Redirector::new("api/v1").unwrap();
        let redirector2 = Redirector::new("api/v2").unwrap();

        // Different paths should generate different short links
        assert_ne!(redirector1.short_file_name, redirector2.short_file_name);
    }

    #[test]
    fn test_short_file_name_format() {
        let redirector = Redirector::new("some/path").unwrap();
        let file_name = redirector.short_file_name.to_string_lossy();

        // Should end with .html
        assert!(file_name.ends_with(".html"));
        // Should not be empty
        assert!(!file_name.is_empty());
    }

    #[test]
    fn test_debug_and_partialeq_traits() {
        let redirector1 = Redirector::new("some/path").unwrap();
        let redirector2 = redirector1.clone();

        // Test PartialEq
        assert_eq!(redirector1, redirector2);

        // Test Debug
        let debug_output = format!("{redirector1:?}");
        assert!(debug_output.contains("Redirector"));
    }
}
