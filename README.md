# link-bridge

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![circleci-badge]][circleci-url]
[![MIT licensed][mit-badge]][mit-url]
[![Rust 1.81+][version-badge]][version-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/link-bridge.svg
[crates-url]: https://crates.io/crates/link-bridge
[docs-badge]: https://docs.rs/link-bridge/badge.svg
[docs-url]: https://docs.rs/link-bridge
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerus-org/link-bridge/blob/main/LICENSE
[circleci-badge]: https://dl.circleci.com/status-badge/img/gh/jerus-org/link-bridge/tree/main.svg?style=svg
[circleci-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/link-bridge/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.81+-orange.svg
[version-url]: https://www.rust-lang.org
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

A lightweight Rust library for creating URL redirects with short names that generate web pages redirecting to longer links on your website.

This crate provides a simple and efficient way to create HTML redirect pages that automatically forward users from short, memorable paths to longer URLs on your website. Perfect for creating user-friendly shortcuts, maintaining backward compatibility after URL changes, or implementing a simple URL shortening system.

## Features

- 🚀 **Fast and lightweight** - Minimal dependencies and efficient operation
- 🔧 **Simple API** - Easy-to-use interface for creating redirects
- 🎯 **URL validation** - Ensures paths contain only valid characters
- 📁 **Automatic file management** - Creates directories and HTML files automatically
- 🌐 **Standards compliant** - Generates proper HTML5 with multiple redirect methods
- 🔒 **Safe** - Built with Rust's memory safety and error handling

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
link-bridge = "0.2"
```

### Basic Usage

```rust
use link_bridge::Redirector;

// Create a redirector for a URL path
let mut redirector = Redirector::new("api/v1/users").unwrap();

// Optionally customize the output directory
redirector.set_path("redirects");

// Generate the redirect HTML file
redirector.write_redirects().unwrap();
```

This creates an HTML file that automatically redirects visitors from your short URL to the longer target path using multiple redirect methods for maximum compatibility.

## How It Works

1. **URL Validation**: Input paths are validated to ensure they contain only safe characters
2. **Unique Naming**: Short file names are generated using base62 encoding and timestamps
3. **HTML Generation**: Complete HTML5 pages are created with multiple redirect methods:
   - Meta refresh tag (universal browser support)
   - JavaScript redirect (faster when JS is enabled)
   - Manual fallback link (accessibility and fail-safe)
4. **File Management**: Directories are created automatically and files are written to disk

## Generated HTML Structure

The library creates complete HTML5 pages that work across all browsers:

```html
<!DOCTYPE HTML>
<html lang="en-US">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="refresh" content="0; url=/your/target/path/">
    <script type="text/javascript">
        window.location.href = "/your/target/path/";
    </script>
    <title>Page Redirection</title>
</head>
<body>
    If you are not redirected automatically, follow this 
    <a href='/your/target/path/'>link</a>.
</body>
</html>
```

## Error Handling

The library uses comprehensive error handling:

```rust
use link_bridge::{Redirector, RedirectorError};

match Redirector::new("invalid?path") {
    Ok(redirector) => println!("Success!"),
    Err(RedirectorError::InvalidUrlPath(e)) => {
        println!("Invalid path: {}", e);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Documentation

For comprehensive API documentation, examples, and advanced usage patterns, visit:

📚 **[Documentation on docs.rs](https://docs.rs/link-bridge)**

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
git clone https://github.com/jerus-org/link-bridge.git
cd link-bridge
cargo build
cargo test
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out html
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and breaking changes.

## Licence

This project is licensed under the MIT Licence - see the [LICENCE](LICENSE) file for details.

## Acknowledgments

- Inspired by URL shortening services like bit.ly and tinyurl
- Thanks to the Rust community for feedback and contributions

---

For questions, issues, or feature requests, please [open an issue](https://github.com/jerus-org/link-bridge/issues) on GitHub.