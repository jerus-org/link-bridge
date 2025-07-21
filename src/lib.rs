//! # link-bridge
//!
//! A lightweight Rust library for creating URL redirects with short names that generate
//! web pages redirecting to longer links on your website.
//!
//! This crate provides a simple and efficient way to create HTML redirect pages that
//! automatically forward users from short, memorable paths to longer URLs on your website.
//! It's perfect for creating user-friendly shortcuts, maintaining backward compatibility
//! after URL changes, or implementing a simple URL shortening system.
//!
//! ## Features
//!
//! - 🚀 **Fast and lightweight** - Minimal dependencies and efficient operation
//! - 🔧 **Simple API** - Easy-to-use interface for creating redirects
//! - 🎯 **URL validation** - Ensures paths contain only valid characters
//! - 📁 **Automatic file management** - Creates directories and HTML files automatically
//! - 🌐 **Standards compliant** - Generates proper HTML5 with multiple redirect methods
//! - 🔒 **Safe** - Built with Rust's memory safety and error handling
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! link-bridge = "0.2"
//! ```
//!
//! ## Basic Usage
//!
//! ```rust
//! use link_bridge::Redirector;
//! use std::fs;
//!
//! // Create a redirector for a URL path
//! let mut redirector = Redirector::new("api/v1/users").unwrap();
//!
//! // Optionally customize the output directory
//! redirector.set_path("redirects");
//!
//! // Generate the redirect HTML file
//! let redirect_path = redirector.write_redirect().unwrap();
//!
//! // Clean up for example
//! fs::remove_dir_all("redirects").ok();
//! ```
//!
//! This creates an HTML file that automatically redirects visitors from your short URL
//! to the longer target path using multiple redirect methods for maximum compatibility.
//!
//! ## How It Works
//!
//! 1. **URL Validation**: Input paths are validated to ensure they contain only safe characters
//! 2. **Unique Naming**: Short file names are generated using base62 encoding and timestamps
//! 3. **HTML Generation**: Complete HTML5 pages are created with multiple redirect methods:
//!    - Meta refresh tag (universal browser support)
//!    - JavaScript redirect (faster when JS is enabled)
//!    - Manual fallback link (accessibility and fail-safe)
//! 4. **File Management**: Directories are created automatically and files are written to disk
//!
//! ## Generated HTML Structure
//!
//! The generated HTML files include:
//!
//! ```html
//! <!DOCTYPE HTML>
//! <html lang="en-US">
//! <head>
//!     <meta charset="UTF-8">
//!     <meta http-equiv="refresh" content="0; url=/your/target/path/">
//!     <script type="text/javascript">
//!         window.location.href = "/your/target/path/";
//!     </script>
//!     <title>Page Redirection</title>
//! </head>
//! <body>
//!     If you are not redirected automatically, follow this
//!     <a href='/your/target/path/'>link</a>.
//! </body>
//! </html>
//! ```
//!
//! ## Error Handling
//!
//! The library uses the [`RedirectorError`] type for comprehensive error handling:
//!
//! ```rust
//! use link_bridge::{Redirector, RedirectorError};
//!
//! match Redirector::new("invalid?path") {
//!     Ok(redirector) => println!("Success!"),
//!     Err(RedirectorError::InvalidUrlPath(e)) => {
//!         println!("Invalid path: {}", e);
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! ```
//!

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

mod redirector;

pub use redirector::Redirector;
pub use redirector::RedirectorError;
