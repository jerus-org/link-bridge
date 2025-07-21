mod url_path;

use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fmt, fs};
use thiserror::Error;

use chrono::Utc;

use crate::redirector::url_path::UrlPath;

#[derive(Debug, Error)]
pub enum RedirectorError {
    #[error("Failed to create redirect file")]
    FileCreationError(#[from] std::io::Error),
    #[error("Short link not found")]
    ShortLinkNotFound,
    #[error("Invalid URL path: {0}")]
    InvalidUrlPath(#[from] url_path::UrlPathError),
}

/// The `Redirector` struct is responsible for managing URL redirection.
///
/// It allows you to create a short link from a long URL on your website,
/// generate the HTML for redirection, and write the redirection HTML file
/// to the filesystem.
///
/// It also provides functionality to set the directory path for storing
/// redirect files.
///
/// It uses base62 encoding for generating short links based on the current
/// timestamp and the long URL's characters.
///
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Redirector {
    /// The path to the long URL that will be shortened.
    long_path: UrlPath,
    /// The file name for the short link.
    short_file_name: OsString,
    /// The path where the redirect HTML files will be stored.
    path: PathBuf,
}

impl Redirector {
    /// Creates a new `Redirector` instance with the specified long link.
    ///
    /// # Arguments
    ///
    /// * `long_link` - The original URL to be shortened
    ///
    /// # Returns
    ///
    /// Returns a new `Redirector` instance with default path "s" and no short link.
    pub fn new<S: ToString>(long_path: S) -> Result<Self, RedirectorError> {
        let long_path = UrlPath::new(long_path.to_string())?;

        let short_file_name = Redirector::generate_short_file_name(&long_path);

        Ok(Redirector {
            long_path,
            short_file_name,
            path: PathBuf::from("s"),
        })
    }

    /// Generates a short link based on the current timestamp and the long link's characters.
    ///
    /// The short link is created using base62 encoding of a combination of the current
    /// timestamp in milliseconds and the sum of UTF-16 code units of the long link.
    fn generate_short_file_name(long_path: &UrlPath) -> OsString {
        let name = base62::encode(
            Utc::now().timestamp_millis() as u64
                + long_path.encode_utf16().iter().sum::<u16>() as u64,
        );
        OsString::from(format!("{name}.html"))
    }

    /// Sets the directory path where redirect files will be stored.
    ///
    /// # Arguments
    ///
    /// * `path` - A string-like value that specifies the directory path
    pub fn set_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.path = path.into();
    }

    /// Writes the redirect HTML file to the filesystem.
    ///
    /// Creates the directory if it doesn't exist and generates an HTML file
    /// containing redirect logic for the shortened URL.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if successful, or a `RedirectorError` if any operation fails.
    ///
    /// # Errors
    ///
    /// Will return `RedirectorError::FileCreationError` if file operations fail.
    /// Will return `RedirectorError::ShortLinkNotFound` if no short link has been generated.
    pub fn write_redirects(&self) -> Result<(), RedirectorError> {
        // create store directory if it doesn't exist
        if !Path::new(&self.path).exists() {
            fs::create_dir_all(&self.path)?;
        }

        let file_path = self.path.join(&self.short_file_name);
        let file_name = file_path.to_string_lossy();
        let mut file = File::create(file_name.as_ref())?;

        file.write_all(self.to_string().as_bytes())?;
        file.sync_all()?;

        Ok(())
    }
}

impl fmt::Display for Redirector {
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
        If you are not redirected automatically, follow this <a href='{target}'>link to example</a>.
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
    fn test_write_redirects_with_valid_path() {
        let redirector = Redirector::new("some/path").unwrap();

        let result = redirector.write_redirects();

        // Should succeed since short link is generated in new()
        assert!(result.is_ok());

        // Clean up
        let file_name = redirector.short_file_name.into_string().unwrap();
        let file_path = format!("s/{file_name}");
        if Path::new(&file_path).exists() {
            fs::remove_file(&file_path).ok();
            fs::remove_dir("s").ok();
        }
    }

    #[test]
    fn test_write_redirects_success() {
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.set_path("test_output");

        let result = redirector.write_redirects();
        assert!(result.is_ok());

        let file_path = redirector.path.join(&redirector.short_file_name);

        assert!(Path::new(&file_path).exists());

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("<!DOCTYPE HTML>"));
        assert!(content.contains("meta http-equiv=\"refresh\""));
        assert!(content.contains("window.location.href"));
        assert!(content.contains("If you are not redirected automatically"));
        assert!(content.contains("/some/path/"));

        fs::remove_file(&file_path).unwrap();
        fs::remove_dir("test_output").unwrap();
    }

    #[test]
    fn test_write_redirects_creates_directory() {
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.set_path("test_dir/subdir");

        assert!(!Path::new("test_dir").exists());

        let result = redirector.write_redirects();
        assert!(result.is_ok());

        assert!(Path::new("test_dir/subdir").exists());

        let file_path = redirector.path.join(&redirector.short_file_name);
        assert!(Path::new(&file_path).exists());

        fs::remove_file(&file_path).unwrap();
        fs::remove_dir_all("test_dir").unwrap();
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
