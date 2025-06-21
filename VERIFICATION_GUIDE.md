# 🚀 Local CI/CD Verification Guide

This repository includes comprehensive local verification scripts that mirror the GitHub Actions workflows exactly. Use these scripts to verify your changes locally before pushing to ensure they pass CI/CD.

## 📝 Available Scripts

### Individual Verification Scripts

- **`./scripts/verify-rust.sh`** - Complete Rust verification
  - Code formatting check (`cargo fmt --check`)
  - Clippy linting (zero warnings policy)
  - Build verification (no-default-features + all-features)
  - Unit tests execution
  - Documentation generation
  - Security audit with cargo-audit
  - Benchmark compilation check

- **`./scripts/verify-frontend.sh`** - Complete frontend verification
  - TypeScript type checking
  - ESLint linting (max 35 warnings allowed)
  - Prettier formatting check
  - Unit tests with coverage
  - Application build
  - E2E tests (optional, with timeout)
  - npm security audit

- **`./scripts/verify-security.sh`** - Security audits
  - Rust dependency security scan
  - Frontend dependency security scan
  - Common security pattern checks
  - Hardcoded secrets detection
  - Unsafe code detection

- **`./scripts/verify-docs.sh`** - Documentation verification
  - Rust documentation build
  - mdBook documentation build (if available)
  - Missing documentation warnings
  - Documentation structure validation

### Master Script

- **`./scripts/verify-all.sh`** - Complete verification suite
  - Runs all verification scripts in sequence
  - Comprehensive summary with pass/fail status
  - Color-coded output with progress tracking
  - Runtime tracking and performance metrics

## 🎯 Quick Start

### Run Complete Verification
```bash
# Verify everything before pushing
./scripts/verify-all.sh
```

### Run Specific Checks
```bash
# Only check Rust code
./scripts/verify-rust.sh

# Only check frontend code  
./scripts/verify-frontend.sh

# Only run security audits
./scripts/verify-security.sh

# Only check documentation
./scripts/verify-docs.sh
```

## 🔧 Prerequisites

### Rust Environment
- Rust toolchain (stable or nightly)
- `cargo-audit` (install with: `cargo install cargo-audit`)

### Frontend Environment  
- Node.js (v18+ recommended)
- npm or yarn
- Playwright browsers (auto-installed by script)

### Documentation (Optional)
- `mdbook` (install with: `cargo install mdbook`)
- `mdbook-linkcheck` (install with: `cargo install mdbook-linkcheck`)

## 📊 Understanding Output

### ✅ Success Indicators
- **Green checkmarks (✓)** - All checks passed
- **"READY FOR CI/CD"** - Your code will pass GitHub Actions

### ⚠️ Warning Indicators  
- **Yellow warnings (⚠)** - Non-critical issues that won't fail CI/CD
- **E2E test warnings** - Expected in headless environments

### ❌ Error Indicators
- **Red X marks (✗)** - Critical issues that will fail CI/CD
- **"NOT READY"** - Fix issues before pushing

## 🛠 Common Fixes

### Rust Issues
```bash
# Fix formatting
cargo fmt --all

# Fix Clippy warnings (review each one)
cargo clippy --fix --allow-dirty

# Update dependencies for security
cargo update
```

### Frontend Issues
```bash
cd neo-gui

# Fix formatting
npm run format

# Fix linting (review each warning)
npm run lint --fix

# Update dependencies
npm audit fix
```

### Documentation Issues
```bash
# Build and check documentation
cargo doc --open

# Fix broken links (manual review needed)
# Update README.md and module documentation
```

## 🚀 Workflow Integration

### Pre-Commit Hook (Recommended)
```bash
# Add to .git/hooks/pre-commit
#!/bin/bash
./scripts/verify-all.sh
if [ $? -ne 0 ]; then
    echo "❌ Verification failed. Fix issues before committing."
    exit 1
fi
```

### IDE Integration
Most IDEs can be configured to run these scripts as build tasks or external tools.

### CI/CD Simulation
The scripts are designed to exactly mirror GitHub Actions workflows:
- Same commands and flags
- Same environment variables
- Same success/failure criteria

## 📈 Performance Tips

### Faster Verification
```bash
# Skip network tests for faster Rust verification
export NEORUST_SKIP_NETWORK_TESTS=1
./scripts/verify-rust.sh

# Skip E2E tests for faster frontend verification
# (E2E tests auto-timeout in 60 seconds anyway)
```

### Parallel Execution
The master script runs components sequentially for clear output, but you can run individual scripts in parallel:
```bash
# Run multiple checks simultaneously
./scripts/verify-rust.sh & ./scripts/verify-frontend.sh & wait
```

## 🔒 Security Features

- **Zero secrets policy** - Scripts detect hardcoded secrets
- **Dependency scanning** - Both Rust and npm vulnerabilities
- **Unsafe code detection** - Highlights potentially dangerous code
- **Security pattern analysis** - Common security antipatterns

## 📚 Troubleshooting

### Common Issues

**Script not executable:**
```bash
chmod +x scripts/verify-*.sh
```

**Missing dependencies:**
```bash
# Install cargo-audit
cargo install cargo-audit

# Install Node.js dependencies
cd neo-gui && npm ci
```

**Permission errors:**
```bash
# Ensure proper file permissions
find scripts -name "*.sh" -exec chmod +x {} \;
```

## 💡 Best Practices

1. **Run verification before every commit**
2. **Fix all errors, review warnings carefully**
3. **Keep dependencies updated regularly**
4. **Review security audit results thoroughly**
5. **Use scripts during development, not just before commits**

## 🎉 Success Metrics

When all scripts pass:
- ✅ Your code follows project standards
- ✅ No security vulnerabilities detected
- ✅ All tests pass
- ✅ Documentation is complete
- ✅ Ready for production deployment

---

**Happy coding! 🦀🌐**