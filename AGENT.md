# AGENT.md - AI Coding Assistant Guide

## Build/Test Commands
- `cargo build` - Build the project
- `cargo test` - Run all tests
- `cargo test <testname>` - Run specific test by name
- `cargo clippy` - Run linter
- `cargo fmt` - Format code
- `cargo tarpaulin --out html` - Run coverage analysis

## Architecture
**Single-purpose Rust library** for URL redirection with HTML file generation.
- `src/lib.rs` - Main library interface, exports `Redirector` and `RedirectorError`
- `src/redirector.rs` - Core redirect logic and HTML generation
- No database or external services - generates static HTML files for redirects

## Code Style
- **Dependencies**: `base62`, `chrono`, `thiserror` - check imports before adding new ones
- **Error handling**: Use `thiserror` for custom errors, return `Result<T, RedirectorError>`
- **Naming**: Snake_case for variables/functions, PascalCase for types/structs
- **Documentation**: Required for public APIs with `///` doc comments
- **Formatting**: Standard rustfmt, no custom rules
- **Types**: Prefer explicit typing, use `impl Into<String>` for flexible string parameters
