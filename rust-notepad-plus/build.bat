@echo off
REM Build script for Notepad++ Rust Edition (Windows)
REM Usage: build.bat [release|debug|clean|test|check]

setlocal enabledelayedexpansion

REM Script configuration
set "SCRIPT_DIR=%~dp0"
set "BUILD_TYPE=release"
if not "%~1"=="" set "BUILD_TYPE=%~1"

echo.
echo ===============================================================
echo   Notepad++ Rust Edition - Build Script
echo ===============================================================
echo.

REM Change to script directory
cd /d "%SCRIPT_DIR%"

REM Handle special commands
if "%BUILD_TYPE%"=="clean" goto :clean
if "%BUILD_TYPE%"=="test" goto :test
if "%BUILD_TYPE%"=="check" goto :check

REM Check prerequisites
call :check_prereqs
if errorlevel 1 exit /b 1

REM Check formatting
call :check_format

REM Run clippy
call :run_clippy
if errorlevel 1 exit /b 1

REM Run tests
call :run_tests
if errorlevel 1 exit /b 1

REM Build
call :build_project
if errorlevel 1 exit /b 1

REM Create distribution
call :create_dist
if errorlevel 1 exit /b 1

REM Print summary
call :print_summary

goto :end

:check_prereqs
    echo ^> Checking prerequisites...

    REM Check for cargo
    where cargo >nul 2>&1
    if errorlevel 1 (
        echo [ERROR] Rust/Cargo not found
        echo         Install from: https://rustup.rs/
        exit /b 1
    )

    for /f "tokens=2" %%i in ('rustc --version') do (
        echo   [OK] Rust: %%i
        goto :prereqs_done
    )

    :prereqs_done
    exit /b 0

:check_format
    echo ^> Checking code formatting...
    cargo fmt -- --check >nul 2>&1
    if errorlevel 1 (
        echo   [WARNING] Code needs formatting
        echo             Run: cargo fmt
    ) else (
        echo   [OK] Code is formatted correctly
    )
    exit /b 0

:run_clippy
    echo ^> Running clippy...
    cargo clippy --all-targets --all-features -- -D warnings
    if errorlevel 1 (
        echo   [ERROR] Clippy found issues
        exit /b 1
    )
    echo   [OK] No clippy warnings
    exit /b 0

:run_tests
    echo ^> Running tests...
    cargo test --workspace
    if errorlevel 1 (
        echo   [ERROR] Tests failed
        exit /b 1
    )
    echo   [OK] All tests passed
    exit /b 0

:build_project
    echo ^> Building (%BUILD_TYPE%)...

    if "%BUILD_TYPE%"=="release" (
        cargo build --release
    ) else (
        cargo build
    )

    if errorlevel 1 (
        echo   [ERROR] Build failed
        exit /b 1
    )
    echo   [OK] Build successful
    exit /b 0

:create_dist
    echo ^> Creating distribution...

    if not exist "%SCRIPT_DIR%dist" mkdir "%SCRIPT_DIR%dist"

    if "%BUILD_TYPE%"=="release" (
        set "BINARY=%SCRIPT_DIR%target\release\notepad-plus.exe"
    ) else (
        set "BINARY=%SCRIPT_DIR%target\debug\notepad-plus.exe"
    )

    if exist "!BINARY!" (
        copy "!BINARY!" "%SCRIPT_DIR%dist\" >nul
        echo   [OK] Binary copied to: %SCRIPT_DIR%dist\

        REM Get file size
        for %%A in ("%SCRIPT_DIR%dist\notepad-plus.exe") do (
            echo       Size: %%~zA bytes
        )
    ) else (
        echo   [ERROR] Binary not found: !BINARY!
        exit /b 1
    )
    exit /b 0

:print_summary
    echo.
    echo ===============================================================
    echo   Build Complete!
    echo ===============================================================
    echo.

    if "%BUILD_TYPE%"=="release" (
        echo Binary: %SCRIPT_DIR%dist\notepad-plus.exe
        echo Run:    %SCRIPT_DIR%dist\notepad-plus.exe
    ) else (
        echo Binary: %SCRIPT_DIR%target\debug\notepad-plus.exe
        echo Run:    cargo run
    )

    echo.
    exit /b 0

:clean
    echo ^> Cleaning build artifacts...
    cargo clean
    if exist "%SCRIPT_DIR%dist" rmdir /s /q "%SCRIPT_DIR%dist"
    echo   [OK] Clean complete
    goto :end

:test
    call :check_prereqs
    if errorlevel 1 goto :end
    call :run_tests
    goto :end

:check
    call :check_prereqs
    if errorlevel 1 goto :end
    call :check_format
    call :run_clippy
    goto :end

:end
    endlocal
