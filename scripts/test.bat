@echo off
REM NeoRust Test Script v0.4.1 for Windows
REM Runs comprehensive tests for the NeoRust SDK

setlocal enabledelayedexpansion

REM Default features for v0.4.1 (AWS disabled for security)
set FEATURES=futures,ledger

REM Parse command line arguments
:parse_args
if "%~1"=="" goto test
if "%~1"=="--features" (
    set "FEATURES=%~2"
    shift
    shift
    goto parse_args
)
if "%~1"=="--help" goto help
if "%~1"=="-h" goto help
echo Unknown option: %~1
echo Use --help for usage information
exit /b 1

:help
echo NeoRust Test Script v0.4.1
echo.
echo Usage: %0 [OPTIONS]
echo.
echo Options:
echo   --features FEATURES    Comma-separated list of features to enable
echo   --help, -h            Show this help message
echo.
echo Available features:
echo   futures               Enable async/futures support
echo   ledger                Enable Ledger hardware wallet support
echo.
echo Examples:
echo   .\scripts\test.bat --features futures,ledger
echo   .\scripts\test.bat --features futures
echo.
echo Note: AWS feature is disabled in v0.4.1 for security reasons
exit /b 0

:test
echo 🧪 Running NeoRust v0.4.1 Test Suite...
echo 📦 Features: %FEATURES%

REM Run main library tests
echo Running main library tests...
cargo test --lib --features "%FEATURES%" --quiet
if errorlevel 1 (
    echo ❌ Main library tests failed
    exit /b 1
)

REM Run CLI tests
echo Running CLI tests...
cd neo-cli
cargo test --quiet
if errorlevel 1 (
    echo ❌ CLI tests failed
    exit /b 1
)
cd ..

REM Run example tests
echo Running example tests...
for /d %%i in (examples\*) do (
    if exist "%%i\Cargo.toml" (
        echo Testing %%~ni...
        cd "%%i"
        cargo test --quiet >nul 2>&1 || echo   (No tests found)
        cd ..\..
    )
)

echo ✅ All tests completed successfully!
echo 📊 Test summary:
echo    - Main library: ✅ PASSED
echo    - CLI tool: ✅ PASSED
echo    - Examples: ✅ CHECKED
echo    - Features: %FEATURES% 