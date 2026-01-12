# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- add release-hook.sh script for automated changelog generation(pr [#22])
- BREAKING: migrate to circleci-toolkit v4.2.1(pr [#23])

### Changed

- chore-rename CHANGELOG.md to PRLOG.md(pr [#20])
- chore-update release.toml to reference PRLOG.md instead of CHANGELOG.md(pr [#21])

## [0.2.3] - 2025-08-09

### Changed

- ♻️ refactor(redirector)-simplify file creation logic(pr [#18])
- 📝 docs(redirector)-add documentation for short_file_name method(pr [#19])

## [0.2.2] - 2025-07-25

### Added

- ✨ enhance JSON output format for registry(pr [#17])

## [0.2.1] - 2025-07-22

### Added

- ✨ add registry system to manage redirects(pr [#16])

### Changed

- 🔧 chore(release)-add release configuration file(pr [#14])
- ♻️ refactor(redirector)-update method signatures and usage(pr [#15])

## [0.2.0] - 2025-07-21

### Added

- ✨ add redirector module for URL shortening(pr [#5])

### Changed

- 📝 docs(url_path)-add module-level documentation for UrlPath utilities(pr [#8])
- 📝 docs(redirector)-enhance module documentation(pr [#9])
- 📝 docs(lib)-add comprehensive documentation for link-bridge(pr [#10])
- 📝 BREAKING: docs(README)-update badge URLs and enhance documentation(pr [#11])
- 👷 ci(circleci)-remove unnecessary parameters from save_next_version job(pr [#12])
- 👷 ci(circleci)-remove unused parameters from config(pr [#13])

### Fixed

- 🐛 tests: clean up test directory after file removal(pr [#6])
- 🐛 url_path: correct regex to include hash character(pr [#7])

## [0.1.0] - 2025-07-20

### Changed

- Create FUNDING.yml(pr [#3])
- Configure Mend Bolt for GitHub(pr [#1])
- 🌐 i18n(contributing): update spelling to en-GB in contributing guide(pr [#4])

[#3]: https://github.com/jerus-org/link-bridge/pull/3
[#1]: https://github.com/jerus-org/link-bridge/pull/1
[#4]: https://github.com/jerus-org/link-bridge/pull/4
[#5]: https://github.com/jerus-org/link-bridge/pull/5
[#6]: https://github.com/jerus-org/link-bridge/pull/6
[#7]: https://github.com/jerus-org/link-bridge/pull/7
[#8]: https://github.com/jerus-org/link-bridge/pull/8
[#9]: https://github.com/jerus-org/link-bridge/pull/9
[#10]: https://github.com/jerus-org/link-bridge/pull/10
[#11]: https://github.com/jerus-org/link-bridge/pull/11
[#12]: https://github.com/jerus-org/link-bridge/pull/12
[#13]: https://github.com/jerus-org/link-bridge/pull/13
[#14]: https://github.com/jerus-org/link-bridge/pull/14
[#15]: https://github.com/jerus-org/link-bridge/pull/15
[#16]: https://github.com/jerus-org/link-bridge/pull/16
[#17]: https://github.com/jerus-org/link-bridge/pull/17
[#18]: https://github.com/jerus-org/link-bridge/pull/18
[#19]: https://github.com/jerus-org/link-bridge/pull/19
[#20]: https://github.com/jerus-org/link-bridge/pull/20
[#21]: https://github.com/jerus-org/link-bridge/pull/21
[#22]: https://github.com/jerus-org/link-bridge/pull/22
[#23]: https://github.com/jerus-org/link-bridge/pull/23
[Unreleased]: https://github.com/jerus-org/link-bridge/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/jerus-org/link-bridge/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/jerus-org/link-bridge/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/jerus-org/link-bridge/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/jerus-org/link-bridge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jerus-org/link-bridge/releases/tag/v0.1.0
