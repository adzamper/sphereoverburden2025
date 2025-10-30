@echo off
REM
REM Build Script for Electromagnetic Field Response Simulator (Windows)
REM
REM This script compiles the Rust WASM module and prepares the
REM static site for deployment to Cloudflare Pages or other hosts.
REM
REM Prerequisites:
REM - Rust toolchain (from https://rustup.rs/)
REM - wasm-pack (cargo install wasm-pack)
REM - wasm32-unknown-unknown target

echo ==========================================
echo Building EM Field Response Simulator
echo ==========================================

REM Step 1: Check for required tools
echo.
echo Checking prerequisites...

where rustc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Rust is not installed
    echo Please install from: https://rustup.rs/
    exit /b 1
)

where wasm-pack >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: wasm-pack is not installed
    echo Install with: cargo install wasm-pack
    exit /b 1
)

echo [OK] All prerequisites found

REM Step 2: Build WASM module
echo.
echo Building WebAssembly module...
cd wasm

wasm-pack build --target web --release --out-dir ../frontend/pkg

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] WASM build failed
    cd ..
    exit /b 1
)

echo [OK] WASM module built successfully

cd ..

REM Step 3: Copy frontend files to dist directory
echo.
echo Preparing distribution files...

REM Create dist directory
if exist dist rmdir /s /q dist
mkdir dist

REM Copy frontend files
copy frontend\index.html dist\ >nul
copy frontend\styles.css dist\ >nul
copy frontend\app.js dist\ >nul
xcopy /E /I /Q frontend\pkg dist\pkg >nul

echo [OK] Distribution files prepared in .\dist

REM Step 4: Display build summary
echo.
echo ==========================================
echo Build completed successfully!
echo ==========================================
echo.
echo Output directory: .\dist
echo WASM module: .\frontend\pkg
echo.
echo To test locally:
echo   1. Open a new command prompt/PowerShell
echo   2. Run: python -m http.server 8000 --directory dist
echo   3. Open http://localhost:8000 in your browser
echo.
echo To deploy to Cloudflare Pages:
echo   1. Push changes to your Git repository
echo   2. Connect repository to Cloudflare Pages
echo   3. Set build command: build.bat (or build.sh on Linux)
echo   4. Set build output directory: dist
echo.
