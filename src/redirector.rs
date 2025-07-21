mod url_path;

use std::fs::File;
use std::io::Write;
use std::path::Path;
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
    short_link: Option<String>,
    path: String,
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
    pub fn new<S: ToString>(long_link: S) -> Result<Self, RedirectorError> {
        let long_link = UrlPath::new(long_link.to_string())?;

        Ok(Redirector {
            long_path: long_link,
            short_link: None,
            path: "s".to_string(),
        })
    }

    /// Generates a short link based on the current timestamp and the long link's characters.
    ///
    /// The short link is created using base62 encoding of a combination of the current
    /// timestamp in milliseconds and the sum of UTF-16 code units of the long link.
    pub fn generate_short_link(&mut self) {
        let short_link = base62::encode(
            Utc::now().timestamp_millis() as u64
                + self.long_path.encode_utf16().iter().sum::<u16>() as u64,
        );
        self.short_link = Some(short_link);
    }

    /// Sets the directory path where redirect files will be stored.
    ///
    /// # Arguments
    ///
    /// * `path` - A string-like value that specifies the directory path
    pub fn set_path<S: Into<String>>(&mut self, path: S) {
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

        let Some(short_link) = &self.short_link else {
            Err(RedirectorError::ShortLinkNotFound)?
        };

        let file_name = format!("{}/{}.html", self.path, short_link);
        let mut file = File::create(file_name)?;

        file.write_all(self.to_string().as_bytes())?;
        file.sync_all()?;

        Ok(())
    }
}

impl fmt::Display for Redirector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref short_link) = self.short_link {
            write!(
                f,
                r#"
    <!DOCTYPE HTML>
    <html lang="en-US">
    
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="refresh" content="0; url={short_link}">
        <script type="text/javascript">
            window.location.href = "{short_link}";
        </script>
        <title>Page Redirection</title>
    </head>
    
    <body>
        <!-- Note: don't tell people to `click` the link, just tell them that it is a link. -->
        If you are not redirected automatically, follow this <a href='{short_link}'>link to example</a>.
    </body>
    
    </html>
    "#
            )
        } else {
            write!(f, "Short link not generated")
        }
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
        assert_eq!(redirector.short_link, None);
        assert_eq!(redirector.path, "s");
    }

    #[test]
    fn test_generate_short_link() {
        let mut redirector = Redirector::new("/some/path").unwrap();

        assert_eq!(redirector.short_link, None);

        redirector.generate_short_link();

        assert!(redirector.short_link.is_some());
        let short_link = redirector.short_link.unwrap();
        assert!(!short_link.is_empty());
    }

    #[test]
    fn test_generate_short_link_unique() {
        let mut redirector1 = Redirector::new("/some/path").unwrap();
        let mut redirector2 = Redirector::new("/some/path").unwrap();

        redirector1.generate_short_link();
        thread::sleep(Duration::from_millis(1));
        redirector2.generate_short_link();

        assert_ne!(redirector1.short_link, redirector2.short_link);
    }

    #[test]
    fn test_set_path() {
        let mut redirector = Redirector::new("/some/path/").unwrap();

        redirector.set_path("custom_path");
        assert_eq!(redirector.path, "custom_path");

        redirector.set_path("another/path".to_string());
        assert_eq!(redirector.path, "another/path");
    }

    #[test]
    fn test_display_without_short_link() {
        let redirector = Redirector::new("some/path").unwrap();
        let output = format!("{redirector}");

        assert_eq!(output, "Short link not generated");
    }

    #[test]
    fn test_display_with_short_link() {
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.short_link = Some("abc123".to_string());

        let output = format!("{redirector}");

        assert!(output.contains("<!DOCTYPE HTML>"));
        assert!(output.contains("abc123"));
        assert!(output.contains("meta http-equiv=\"refresh\""));
        assert!(output.contains("window.location.href"));
    }

    #[test]
    fn test_write_redirects_without_short_link() {
        let redirector = Redirector::new("some/path").unwrap();

        let result = redirector.write_redirects();

        assert!(result.is_err());
        match result.unwrap_err() {
            RedirectorError::ShortLinkNotFound => (),
            _ => panic!("Expected ShortLinkNotFound error"),
        }
    }

    #[test]
    fn test_write_redirects_success() {
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.set_path("test_output");
        redirector.generate_short_link();

        let result = redirector.write_redirects();
        assert!(result.is_ok());

        let short_link = redirector.short_link.as_ref().unwrap();
        let file_path = format!("test_output/{short_link}.html");

        assert!(Path::new(&file_path).exists());

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("<!DOCTYPE HTML>"));
        assert!(content.contains(short_link));

        fs::remove_file(&file_path).unwrap();
        fs::remove_dir("test_output").unwrap();
    }

    #[test]
    fn test_write_redirects_creates_directory() {
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.set_path("test_dir/subdir");
        redirector.generate_short_link();

        assert!(!Path::new("test_dir").exists());

        let result = redirector.write_redirects();
        assert!(result.is_ok());

        assert!(Path::new("test_dir/subdir").exists());

        let short_link = redirector.short_link.as_ref().unwrap();
        let file_path = format!("test_dir/subdir/{short_link}.html");
        assert!(Path::new(&file_path).exists());

        fs::remove_file(&file_path).unwrap();
        fs::remove_dir_all("test_dir").unwrap();
    }

    #[test]
    fn test_redirector_clone() {
        let mut redirector = Redirector::new("some/path").unwrap();
        redirector.generate_short_link();
        redirector.set_path("custom");

        let cloned = redirector.clone();

        assert_eq!(redirector, cloned);
        assert_eq!(redirector.long_path, cloned.long_path);
        assert_eq!(redirector.short_link, cloned.short_link);
        assert_eq!(redirector.path, cloned.path);
    }

    #[test]
    fn test_redirector_default() {
        let redirector = Redirector::default();

        assert_eq!(redirector.long_path, UrlPath::default());
        assert_eq!(redirector.short_link, None);
        assert_eq!(redirector.path, "");
    }
}
