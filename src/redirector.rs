use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fmt, fs};
use thiserror::Error;

use chrono::Utc;

#[derive(Debug, Error)]
pub enum RedirectorError {
    #[error("Failed to create redirect file")]
    FileCreationError(#[from] std::io::Error),
    #[error("Short link not found")]
    ShortLinkNotFound,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Redirector {
    long_link: String,
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
    pub fn new(long_link: String) -> Self {
        Redirector {
            long_link,
            short_link: None,
            path: "s".to_string(),
        }
    }

    /// Generates a short link based on the current timestamp and the long link's characters.
    ///
    /// The short link is created using base62 encoding of a combination of the current
    /// timestamp in milliseconds and the sum of UTF-16 code units of the long link.
    pub fn generate_short_link(&mut self) {
        let short_link = base62::encode(
            Utc::now().timestamp_millis() as u64
                + self.long_link.encode_utf16().sum::<u16>() as u64,
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
