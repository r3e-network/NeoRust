@echo off
REM NeoRust Build Script v0.4.1 for Windows
REM Builds the NeoRust SDK with specified features

setlocal enabledelayedexpansion

REM Default features for v0.4.1 (AWS disabled for security)
set FEATURES=futures,ledger

REM Parse command line arguments
:parse_args
if "%~1"=="" goto build
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
echo NeoRust Build Script v0.4.1
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
echo   .\scripts\build.bat --features futures,ledger
echo   .\scripts\build.bat --features futures
echo.
echo Note: AWS feature is disabled in v0.4.1 for security reasons
exit /b 0

:build
echo 🏗️  Building NeoRust v0.4.1...
echo 📦 Features: %FEATURES%

REM Build main library
echo Building main library...
cargo build --release --features "%FEATURES%"
if errorlevel 1 (
    echo ❌ Main library build failed
    exit /b 1
)

REM Build CLI
echo Building CLI...
cd neo-cli
cargo build --release
if errorlevel 1 (
    echo ❌ CLI build failed
    exit /b 1
)
cd ..

REM Build examples
echo Building examples...
for /d %%i in (examples\*) do (
    if exist "%%i\Cargo.toml" (
        echo Building %%~ni...
        cd "%%i"
        cargo build --release
        if errorlevel 1 (
            echo ❌ Example %%~ni build failed
            exit /b 1
        )
        cd ..\..
    )
)

echo ✅ Build completed successfully!
echo 📊 Build summary:
echo    - Main library: ✅
echo    - CLI tool: ✅
echo    - Examples: ✅
echo    - Features: %FEATURES% 