@echo off
REM NeoRust CI Check Script - Local GitHub Workflow Replication (Windows)
REM This script runs all the same checks as the GitHub CI workflow

setlocal enabledelayedexpansion

echo [CI CHECK] Starting NeoRust CI Check Pipeline...
echo ==============================================

REM Set environment variables
set CARGO_TERM_COLOR=always
set RUST_BACKTRACE=1
set RUSTDOCFLAGS=-D warnings

REM Track overall success
set OVERALL_SUCCESS=0

REM 1. Check formatting
echo [CI CHECK] Running: Code Formatting Check
cargo fmt --all -- --check
if !errorlevel! neq 0 (
    echo [WARNING] Running cargo fmt --all to fix formatting...
    cargo fmt --all
    set OVERALL_SUCCESS=1
) else (
    echo [SUCCESS] Code Formatting Check passed
)

REM 2. Run clippy
echo [CI CHECK] Running: Clippy Analysis
cargo clippy --all-targets --all-features -- -D warnings
if !errorlevel! neq 0 (
    echo [ERROR] Clippy Analysis failed
    set OVERALL_SUCCESS=1
) else (
    echo [SUCCESS] Clippy Analysis passed
)

REM 3. Build without default features
echo [CI CHECK] Running: Build (no default features)
cargo build --verbose --no-default-features
if !errorlevel! neq 0 (
    echo [ERROR] Build (no default features) failed
    set OVERALL_SUCCESS=1
) else (
    echo [SUCCESS] Build (no default features) passed
)

REM 4. Build with all features
echo [CI CHECK] Running: Build (all features)
cargo build --verbose --all-features
if !errorlevel! neq 0 (
    echo [ERROR] Build (all features) failed
    set OVERALL_SUCCESS=1
) else (
    echo [SUCCESS] Build (all features) passed
)

REM 5. Run main tests
echo [CI CHECK] Running: Main Test Suite
cargo test --all-features --no-fail-fast -- --nocapture
if !errorlevel! neq 0 (
    echo [ERROR] Main Test Suite failed
    set OVERALL_SUCCESS=1
) else (
    echo [SUCCESS] Main Test Suite passed
)

REM 6. Run CLI tests
echo [CI CHECK] Running: CLI Tests
cd neo-cli
cargo test --all-features --no-fail-fast -- --nocapture
if !errorlevel! neq 0 (
    echo [ERROR] CLI Tests failed
    set OVERALL_SUCCESS=1
) else (
    echo [SUCCESS] CLI Tests passed
)
cd ..

REM 7. Security audit
echo [CI CHECK] Running: Security Audit
cargo audit 2>nul
if !errorlevel! neq 0 (
    echo [WARNING] Security audit failed - checking if cargo-audit is installed...
    cargo install cargo-audit
    cargo audit
    if !errorlevel! neq 0 (
        set OVERALL_SUCCESS=1
    )
) else (
    echo [SUCCESS] Security Audit passed
)

REM 8. Dependency check
echo [CI CHECK] Running: Dependency Check
cargo deny check 2>nul
if !errorlevel! neq 0 (
    echo [WARNING] Dependency check failed - checking if cargo-deny is installed...
    cargo install cargo-deny
    cargo deny check
    if !errorlevel! neq 0 (
        set OVERALL_SUCCESS=1
    )
) else (
    echo [SUCCESS] Dependency Check passed
)

REM 9. Documentation check
echo [CI CHECK] Running: Documentation Build
cargo doc --all-features --no-deps --document-private-items
if !errorlevel! neq 0 (
    echo [ERROR] Documentation Build failed
    set OVERALL_SUCCESS=1
) else (
    echo [SUCCESS] Documentation Build passed
)

REM 10. Show Rust version
echo [CI CHECK] Checking current Rust version...
rustc --version

echo ==============================================

if !OVERALL_SUCCESS! equ 0 (
    echo [SUCCESS] 🎉 ALL CI CHECKS PASSED! Ready to push to GitHub.
) else (
    echo [ERROR] ❌ Some CI checks failed. Please fix the issues above before pushing.
    exit /b 1
) 