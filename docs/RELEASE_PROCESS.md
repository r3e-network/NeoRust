# NeoRust Automated Release Process

This document describes the release process for NeoRust using GitHub Actions and tag-based triggers.

## 🚀 Quick Start for New Releases

### 1. Add the crates.io API key to GitHub Secrets

**⚠️ SECURITY**: Never commit API keys to the repository!

1. Go to your GitHub repository → **Settings** → **Secrets and variables** → **Actions**
2. Click **"New repository secret"**
3. Name it `CARGO_TOKEN` (preferred) or `CRATES_IO_TOKEN` (legacy fallback)
4. Value: Your crates.io API token
5. Click **"Add secret"**

### 2. Create a New Release

```bash
# 1) Update Cargo.toml, neo-cli/Cargo.toml, Cargo.lock, and CHANGELOG.md
# 2) Run local quality checks
cargo fmt --all -- --check
cargo clippy --workspace --locked --all-features --all-targets -- -D warnings
cargo test --workspace --locked
RUSTDOCFLAGS="-D warnings" cargo doc -p neo3 --locked --all-features --no-deps
cargo audit
cargo deny check --hide-inclusion-graph
cargo publish --dry-run --locked

# 3) Tag and push (release workflow triggers on vX.Y.Z tags)
VERSION=2.1.0
git tag -a "v${VERSION}" -m "Release neo3 v${VERSION}"
git push origin "v${VERSION}"
```

### 3. Monitor the Automated Release

The GitHub Actions workflow will automatically:
- Validate root/CLI versions, locked metadata, and the dated changelog section
- Run formatting, Clippy, workspace tests, rustdoc, security audit, and supply-chain policy checks
- Build Linux, macOS, and Windows CLI artifacts
- Publish `neo3` to crates.io, failing if credentials are unavailable
- Create the GitHub release only after publication succeeds

## 🔄 Automated Workflow Details

## 🎯 Release Strategy

### Versioning Scheme

NeoRust follows [Semantic Versioning (SemVer)](https://semver.org/) with the format `MAJOR.MINOR.PATCH`:

- **MAJOR**: Breaking changes that require user migration
- **MINOR**: New features that are backwards compatible
- **PATCH**: Bug fixes and security updates

### Pre-Release Versions

Pre-release versions use suffixes to indicate stability:
- `alpha` - Early development, significant changes expected
- `beta` - Feature complete, testing and stabilization phase
- `rc` - Release candidate, final testing before stable

Examples: `2.0.0-alpha.1`, `2.0.0-beta.2`, `2.0.0-rc.1`

### Release Channels

1. **Stable** - Production-ready releases (no suffix)
2. **Beta** - Preview releases for testing new features
3. **Alpha** - Early access to development features

## 📋 Release Checklist

### Pre-Release Phase

#### 1. Version Planning
- [ ] Review feature completeness and breaking changes
- [ ] Determine appropriate version number (MAJOR.MINOR.PATCH)
- [ ] Create release milestone in GitHub
- [ ] Update project roadmap and documentation

#### 2. Code Quality Gates
- [ ] All CI tests passing on main branch
- [ ] Code coverage meets minimum threshold (>80%)
- [ ] No high or critical security vulnerabilities
- [ ] Clippy lints passing with no warnings
- [ ] Code formatting consistent (`cargo fmt --check`)

#### 3. Documentation Updates
- [ ] Update CHANGELOG.md with all changes
- [ ] Update version numbers in all Cargo.toml files
- [ ] Update README.md if needed
- [ ] Regenerate API documentation
- [ ] Review and update user guides

#### 4. Testing Phase
- [ ] Run full test suite on all supported platforms
- [ ] Test examples and tutorials
- [ ] Performance benchmarks regression testing
- [ ] Integration testing with real Neo N3 networks

### Release Phase

#### 1. Version Tagging
```bash
# Set the release version, then create and push the version tag
VERSION=2.1.0
git tag -a "v${VERSION}" -m "Release neo3 v${VERSION}"
git push origin "v${VERSION}"
```

#### 2. Automated Release (GitHub Actions)
The release workflow automatically:
- [ ] Validates version format and changelog
- [ ] Runs the release verification suite
- [ ] Builds binaries for all platforms
- [ ] Publishes `neo3` to crates.io
- [ ] Creates GitHub release with artifacts
- [ ] Extracts release notes from `CHANGELOG.md`

#### 3. Manual Verification
- [ ] Verify crates.io publication
- [ ] Test installation from crates.io
- [ ] Verify GitHub release artifacts
- [ ] Verify docs.rs build for `neo3`

### Post-Release Phase

#### 1. Communication
- [ ] Announce release on GitHub Discussions
- [ ] Update community channels (Discord, Reddit)
- [ ] Create blog post for major releases
- [ ] Update project website

#### 2. Monitoring
- [ ] Monitor for bug reports and issues
- [ ] Track download statistics
- [ ] Monitor security alerts
- [ ] Collect user feedback

## 🤖 Automation Workflows

### Build & Test (`build-test.yml`)
Runs on pushes and pull requests:
- Multi-platform build/test (Linux, Windows, macOS)
- `cargo fmt` / strict `cargo clippy`
- `cargo build` / `cargo test` (no-default-features)
- `cargo doc` build
- Security audit (RustSec advisories fail; configured informational warnings are reported)
- Optional code coverage (main/master only)

### Release Automation (`release.yml`)
Triggered by version tags:
- Version validation
- Comprehensive testing
- Multi-platform binary builds
- Crate publishing to crates.io (`neo3`)
- GitHub release creation
- Release notes extracted from `CHANGELOG.md`

## 📦 Publishing Notes

The current release workflow publishes the `neo3` crate. The workspace contains additional applications
(`neo-cli`) that are built as release artifacts but are not published as crates.

If/when the SDK is split into multiple crates, update this document and `release.yml` accordingly.
See `WORKSPACE_REORGANIZATION.md` for the tracked plan.

### Version Synchronization
The release tag, `neo3` crate, bundled `neo-cli` artifact, and CLI dependency requirement use the same
version. `neo-cli` remains explicitly non-publishable and is distributed only through release artifacts.

## 🔍 Quality Gates

### Automated Checks
- **Compilation**: All crates must compile successfully
- **Tests**: All tests must pass (unit, integration, doc tests)
- **Linting**: Clippy must pass with no warnings
- **Formatting**: Code must be properly formatted
- **Security**: No known vulnerabilities in dependencies
- **Documentation**: All public APIs must be documented

### Manual Reviews
- **Breaking Changes**: Review impact and migration path
- **Security Changes**: Security team review required
- **Performance**: Benchmark regression analysis
- **User Experience**: Documentation and API usability review

## 🚨 Hotfix Process

For critical security issues or major bugs:

1. **Assessment**: Determine severity and impact
2. **Branch Creation**: Create hotfix branch from latest release tag
3. **Fix Development**: Implement minimal fix with tests
4. **Expedited Review**: Fast-track code review process
5. **Release**: Follow abbreviated release process
6. **Communication**: Immediate security advisory if needed

### Hotfix Version Numbers
- Increment PATCH version: `2.1.0` → `2.1.1`
- For security fixes that require validation before general availability, use a standard pre-release such as `2.1.1-rc.1`

## 📊 Release Metrics

### Success Criteria
- **Build Success**: 100% CI pass rate
- **Test Coverage**: >80% line coverage maintained
- **Security**: Zero high/critical vulnerabilities
- **Performance**: No significant regressions
- **Documentation**: Complete API coverage

### Monitoring
- Download statistics from crates.io
- GitHub release download counts
- Issue reports and bug tracking
- Community feedback and adoption metrics

## 🔧 Tools and Scripts

### Local Validation
```bash
cargo fmt --all -- --check
cargo clippy --workspace --locked --all-features --all-targets -- -D warnings
cargo test --workspace --locked
RUSTDOCFLAGS="-D warnings" cargo doc -p neo3 --locked --all-features --no-deps
cargo audit
cargo deny check --hide-inclusion-graph
cargo publish --dry-run --locked
```

## 🔗 Related Documentation

- [Contributing Guidelines](../CONTRIBUTING.md)
- [Security Policy](../SECURITY.md)
- [API Guidelines](../API_GUIDELINES.md)
- [Project Structure](../PROJECT_STRUCTURE.md)

## 📞 Support

For release-related questions:
- **GitHub Discussions**: Technical questions and feedback
- **GitHub Issues**: Bug reports and feature requests
- **Security**: security@r3e.network for security issues

---

This release process ensures high-quality, reliable releases while maintaining development velocity and community trust.
