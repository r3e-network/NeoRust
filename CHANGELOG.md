# Changelog

All notable changes to NeoRust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New `mock-hsm` feature flag for development and testing with YubiHSM mock functionality
- Comprehensive [Build Configuration Guide](docs/guides/build-configuration.md) for dependency management
- Documentation for handling environment-specific build configurations

### Fixed
- **YubiHSM MockHsm Release Build Error**: Resolved compilation error where MockHsm was included in release builds
  - Removed `mockhsm` feature from default YubiHSM dependency configuration
  - Added optional `mock-hsm` feature flag for development builds
  - Updated documentation with troubleshooting guide for build configuration issues

### Changed
- YubiHSM dependency now uses `["http", "usb"]` features by default (production-safe)
- Mock functionality now requires explicit `mock-hsm` feature flag activation

## [0.2.3] - 2025-05-31

### Added
- Comprehensive release workflow for automated multi-platform binary builds
- Support for Linux (x86_64, ARM64), macOS (Intel, Apple Silicon), and Windows (64-bit, 32-bit)
- Automatic crate publishing to crates.io on release
- Complete documentation website with Docusaurus and beautiful Neo branding
- Placeholder SVG images for all documentation sections

### Fixed
- CLI build paths in release workflow (now builds from neo-cli directory)
- Netlify deployment configuration with correct build commands
- TailwindCSS configuration conflicts causing PostCSS errors
- Missing image assets in documentation with proper SVG placeholders
- Release workflow binary preparation and upload paths

### Changed
- Updated release workflow to exclude website building as requested
- Improved error handling in automated release process
- Enhanced documentation with comprehensive release workflow guide

## [0.2.0] - 2025-05-31

### Added
- Comprehensive documentation website with Docusaurus
- Complete GUI, CLI, and SDK documentation with beautiful design
- Getting started guides for installation, quick start, and first wallet
- Detailed NFT operations guide with minting, trading, and portfolio management
- Developer tools documentation with encoding, hashing, and cryptographic utilities
- Complete CLI commands reference with examples and usage patterns
- Professional website design with Neo branding and responsive layout

### Changed
- Major codebase cleanup removing temporary status and documentation files
- Updated all version numbers from 0.1.9 to 0.2.0 across all packages
- Improved project organization and structure
- Enhanced documentation quality and completeness

### Removed
- Temporary documentation status files (DOCUMENTATION_WEBSITE_STATUS.md, etc.)
- Implementation status tracking files
- Improvement plan documents
- Production status files

## [0.1.9] - 2025-03-05

### Added
- Comprehensive support for Neo N3 network advancements
- Enhanced NeoFS integration with improved object storage capabilities
- Advanced DeFi interactions through well-known contracts
- Full support for latest NEP standards

### Changed
- Updated copyright notices to reflect 2025
- Improved documentation with new tutorials and examples
- Enhanced performance for blockchain operations
- Upgraded dependencies to latest versions
- Bumped version number for release
- Updated all documentation and references to use v0.1.9
- Improved documentation and code organization

### Fixed
- Resolved long-standing issues with transaction signing
- Improved error handling and recovery mechanisms
- Better compatibility with Neo ecosystem projects

### Removed
- Completely removed PDF generation from documentation workflow
- Deleted the docs-pdf.yml workflow file
- Removed PDF references from README.md and configuration files
- Removed PDF output configuration from docs/book.toml

## [0.1.8] - 2025-03-04

### Changed
- Bumped version number for release
- Updated all documentation and references to use v0.1.8
- Improved code stability and documentation clarity

## [0.1.7] - 2025-03-03

### Removed
- Completely removed all SGX-related content from the entire codebase
- Deleted SGX examples directory
- Removed all SGX references from documentation
- Removed SGX references from build and test scripts
- Deleted Makefile.sgx

### Fixed
- Documentation issues with crates.io and docs.rs
- Fixed feature gating for documentation generation
- Added proper feature attributes for conditional compilation

### Changed
- Improved documentation of available features
- Enhanced build configuration for docs.rs
- Added build.rs for better docs.rs integration
- Updated all module header documentation

## [0.1.6] - 2025-03-03

### Removed
- SGX (Intel Software Guard Extensions) support has been completely removed to simplify the codebase and reduce dependencies
- Removed the `neo_sgx` module and all related SGX code
- Removed SGX-related documentation, examples, and references

### Changed
- Updated documentation to reflect the removal of SGX support
- Simplified build and test scripts to remove SGX options
- Updated version references in documentation

## [0.1.5] - 2025-02-15

### Added
- Enhanced support for Neo X EVM compatibility layer
- Improved wallet management features
- Better error handling for network operations

### Fixed
- Various bug fixes and performance improvements
- Resolved issues with transaction serialization
- Fixed memory leaks in long-running operations

## [0.1.4] - 2025-01-10

### Added
- Initial public release on crates.io
- Support for Neo N3 blockchain operations
- Wallet management and transaction capabilities
- Smart contract interaction
- NEP-17 token support
- Neo Name Service (NNS) integration
- NeoFS distributed storage support 