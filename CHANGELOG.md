# Changelog

All notable changes to BRIM will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-02-06

### Added

#### Core Features
- **Dry-run mode (`--dry-run`)**: Preview installation/removal changes without executing them
- **Recipe chaining**: Support for multiple recipe files via comma-separated syntax or multiple `--url` flags
  - Example: `brim --url="base.json,extras.json"` or `brim --url="base.json" --url="extras.json"`
  - Later files override earlier ones for deduplication
  - Mix remote and local files freely
- **Sync command (`--sync`)**: Compare installed packages with recipe files and show diff analysis
  - Shows packages to install (missing from system)
  - Shows extra packages (installed but not in recipe)
  - Shows packages already in sync
- **Recipe validation**: Automatic validation of recipe files with helpful error messages
  - Package name format validation (alphanumeric, dots, hyphens, underscores)
  - URL format validation (must start with http:// or https://)
  - Version format validation (semantic versioning)
  - Empty recipe detection
- **JSON Schema**: Added `recipe-schema.json` for IDE autocomplete and validation support

#### Installation & Distribution
- **Automated release workflow**: GitHub Actions workflow for multi-platform binary releases
  - macOS (Intel x86_64 and Apple Silicon ARM64)
  - Linux (x86_64 and ARM64)
  - Automatic tarball creation and checksum generation
- **Installation script**: One-line install script (`install.sh`) for easy setup
  - Automatic platform detection
  - Downloads latest release binary
  - Sets up PATH configuration
  - Example: `curl -fsSL https://raw.githubusercontent.com/alexandrughinea/brim/main/install.sh | bash`

#### Improvements
- **Enhanced error messages**: Clearer, more helpful error messages throughout the codebase
  - Specific guidance for common issues
  - Formatted with colors for better readability
  - Actionable suggestions for fixing problems
- **Local file support**: Native support for local recipe files (no need for `file://` prefix)
  - Automatic detection of local vs remote files
  - Better error handling for file operations
- **Better help text**: Improved command-line help and examples

### Changed
- Updated `--url` flag to support multiple values and comma-separated syntax
- Improved package fetching logic to handle both local and remote files seamlessly
- Enhanced TUI progress display for chained recipe loading

### Technical
- Bumped version to 0.2.0
- Added comprehensive recipe validation system
- Implemented fetch error types for better error handling
- Created installation automation for macOS and Linux platforms

## [0.1.0] - 2024

### Added
- Initial release
- Beautiful terminal UI with ratatui
- Interactive package selection
- Parallel download mode
- Package installation and removal
- Cask support
- Webhook integration for CI/CD
- Progress tracking
- Color-coded UI

[0.2.0]: https://github.com/alexandrughinea/brim/releases/tag/v0.2.0
[0.1.0]: https://github.com/alexandrughinea/brim/releases/tag/v0.1.0
