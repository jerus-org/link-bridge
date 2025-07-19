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
[mit-url]: https://github.com/jerusdp/pcu/blob/main/LICENSE
[circleci-badge]: https://dl.circleci.com/status-badge/img/gh/jerus-org/pcu/tree/main.svg?style=svg
[circleci-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/pcu/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.81+-orange.svg
[version-url]: https://www.rust-lang.org
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

A lightweight Rust library for creating URL redirects with short names that generate web pages redirecting to longer links on your website.

## Features

- 🚀 Fast and lightweight URL redirection
- 🔧 Simple API for creating short name to long URL mappings

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
link-bridge = "0.1"
```

### Basic Usage

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