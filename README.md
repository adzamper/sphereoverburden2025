# Electromagnetic Field Response Simulator

A professional web-based tool for modeling time-domain electromagnetic (EM) field responses from buried conductive spheres in layered earth models. This application is designed for geophysical surveying, mineral exploration, and electromagnetic induction research.

## Overview

This simulator implements first-order approximation methods for calculating transient induced polarization (IP) survey responses. It accounts for:

- **Spherical conductor response** in conducting media
- **Overburden layer effects** using image theory
- **Geological dip/strike orientations** for tilted targets
- **Time-domain transient decay** at multiple measurement windows

The application runs entirely in the browser using **WebAssembly** (compiled from Rust) for high-performance calculations, with a clean, professional interface inspired by MATLAB and scientific computing tools.

## Features

- **High-Performance Calculations**: Core physics engine compiled to WebAssembly from Rust
- **Interactive Visualization**: Real-time plotting with Plotly.js
- **Professional Interface**: Clean, functional design without distractions
- **Static Site**: No backend required - deploy anywhere
- **Responsive Design**: Works on desktop and tablet devices
- **Comprehensive Parameter Control**: Full control over survey and model parameters

## Architecture

### Technology Stack

- **Backend**: Rust compiled to WebAssembly via wasm-bindgen
- **Frontend**: Vanilla JavaScript with Plotly.js for visualization
- **Styling**: Clean CSS following scientific application design principles
- **Deployment**: Static site compatible with Cloudflare Pages, GitHub Pages, etc.

## Quick Start (Windows)

**New to Rust/WASM? Follow these steps:**

1. **Install Rust** (5 minutes)
   - Go to https://rustup.rs/
   - Download and run `rustup-init.exe`
   - Follow prompts (use default options)
   - **Restart your terminal/PowerShell**

2. **Install wasm-pack** (2 minutes)
   - Open new PowerShell or Command Prompt
   - Run: `cargo install wasm-pack`
   - Wait for installation (may take a few minutes)

3. **Add WASM target** (1 minute)
   - Run: `rustup target add wasm32-unknown-unknown`

4. **Build the application** (5-15 minutes first time)
   - Navigate to project: `cd C:\path\to\sphereoverburden2025`
   - Run: `build.bat`
   - Wait patiently (downloads dependencies on first build)

5. **Test locally**
   - Option A - Python: `python -m http.server 8000 --directory dist`
   - Option B - VS Code: Install "Live Server" extension, open `dist/index.html`, right-click → "Open with Live Server"
   - Open http://localhost:8000 in your browser

**That's it!** You should now see the application running.

### Project Structure

```
sphereoverburden2025/
├── wasm/                   # Rust WASM module
│   ├── src/
│   │   └── lib.rs         # Electromagnetic calculations
│   └── Cargo.toml         # Rust dependencies
├── frontend/               # Web application
│   ├── index.html         # Main HTML page
│   ├── styles.css         # Professional styling
│   ├── app.js             # Application logic
│   └── pkg/               # Built WASM output (generated)
├── oldcode/                # Original implementation (reference)
│   ├── backend/           # Original Rust server
│   └── frontend/          # Original React app
├── dist/                   # Production build (generated)
├── build.sh               # Build script (Linux/Mac)
├── build.bat              # Build script (Windows)
├── serve.sh               # Dev server script (Linux/Mac)
└── README.md              # This file
```

## Prerequisites

Before building the application, you need:

### 1. Rust Toolchain

**Windows:**
- Download and run the installer from https://rustup.rs/
- Or use the direct link: https://win.rustup.rs/x86_64
- Follow the installer prompts (default options work fine)
- Restart your terminal/PowerShell after installation

**Linux/Mac:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:
```bash
rustc --version
cargo --version
```

### 2. wasm-pack (WASM build tool)

After Rust is installed, open a new terminal/PowerShell and run:

```bash
cargo install wasm-pack
```

This works the same on Windows, Linux, and Mac.

### 3. wasm32 target

```bash
rustup target add wasm32-unknown-unknown
```

This works the same on all platforms.

## Building the Application

### Windows Build (PowerShell)

```powershell
# Navigate to project directory
cd C:\path\to\sphereoverburden2025

# Build WASM module
cd wasm
wasm-pack build --target web --release --out-dir ../frontend/pkg
cd ..

# Prepare distribution directory
if (Test-Path dist) { Remove-Item -Recurse -Force dist }
New-Item -ItemType Directory -Path dist

# Copy files to dist
Copy-Item frontend/index.html dist/
Copy-Item frontend/styles.css dist/
Copy-Item frontend/app.js dist/
Copy-Item -Recurse frontend/pkg dist/
```

### Windows Build (Command Prompt)

```cmd
REM Navigate to project directory
cd C:\path\to\sphereoverburden2025

REM Build WASM module
cd wasm
wasm-pack build --target web --release --out-dir ../frontend/pkg
cd ..

REM Prepare distribution directory
if exist dist rmdir /s /q dist
mkdir dist

REM Copy files to dist
copy frontend\index.html dist\
copy frontend\styles.css dist\
copy frontend\app.js dist\
xcopy /E /I frontend\pkg dist\pkg
```

### Linux/Mac Build

Run the build script:

```bash
./build.sh
```

Or manually:

```bash
# Build WASM module
cd wasm
wasm-pack build --target web --release --out-dir ../frontend/pkg
cd ..

# Prepare distribution
rm -rf dist
mkdir -p dist
cp frontend/index.html dist/
cp frontend/styles.css dist/
cp frontend/app.js dist/
cp -r frontend/pkg dist/
```

**Note:** All builds accomplish the same thing:
1. Compile the Rust code to WebAssembly
2. Generate JavaScript bindings
3. Copy all files to the `dist/` directory

## Local Development

To test the application locally, you need a web server (due to ES module and WASM requirements).

### Option 1: Python (Recommended for Windows)

**If you have Python 3 installed:**

```bash
# Windows, Linux, Mac
python -m http.server 8000 --directory dist
```

Then open http://localhost:8000 in your browser.

**Note for Windows users:**
- Try `python` first (not `python3`)
- If that doesn't work, try `py -m http.server 8000 --directory dist`
- If `--directory` flag isn't recognized (older Python), navigate to the dist folder first:
  ```cmd
  cd dist
  python -m http.server 8000
  ```

### Option 2: Node.js http-server

If you have Node.js installed:

```bash
# Install http-server globally (one-time)
npm install -g http-server

# Run server
http-server dist -p 8000
```

Or use without installation:

```bash
npx http-server dist -p 8000
```

### Option 3: Visual Studio Code Live Server

**Great option for Windows users:**

1. Install VS Code from https://code.visualstudio.com/
2. Install the "Live Server" extension
3. Open the `dist` folder in VS Code
4. Right-click `index.html` and select "Open with Live Server"

### Option 4: PHP

If you have PHP installed:

```bash
php -S localhost:8000 -t dist
```

### Option 5: Simple Windows Tools

**For quick testing on Windows without installing anything:**

- **Browser extension**: "Web Server for Chrome" (works offline)
- **Standalone**: Download "Fenix Web Server" or "Mongoose Web Server"

### Testing the Application

Once your server is running:
1. Open http://localhost:8000 (or the URL shown by your server)
2. You should see the Electromagnetic Field Response Simulator interface
3. Fill in parameters and click "Calculate Response"
4. If you see an error about WASM loading, check the browser console (F12)

## Deployment to Cloudflare Pages

### Automatic Deployment

1. **Push to Git repository**:
   ```bash
   git add .
   git commit -m "Initial commit"
   git push origin main
   ```

2. **Connect to Cloudflare Pages**:
   - Log in to Cloudflare Dashboard
   - Navigate to Pages
   - Click "Create a project"
   - Connect your Git repository

3. **Configure build settings**:
   - **Build command**: `./build.sh`
   - **Build output directory**: `dist`
   - **Root directory**: `/` (default)

4. **Deploy**:
   - Cloudflare will automatically build and deploy
   - Your site will be available at `https://your-project.pages.dev`

### Manual Deployment

Alternatively, you can build locally and upload the `dist/` directory:

```bash
./build.sh
# Then upload the contents of dist/ via Cloudflare Dashboard
```

## Usage

### Basic Workflow

1. **Adjust Survey Parameters**:
   - Set transmitter height and profile length
   - Configure dipole moment and pulse parameters

2. **Define Target Geometry**:
   - Sphere radius, conductivity, and position
   - Overburden layer properties

3. **Optional Settings**:
   - Enable dip/strike rotation for tilted bodies
   - Adjust sign conventions for different survey types

4. **Calculate**:
   - Click "Calculate Response" button
   - Results will appear in ~1-5 seconds

5. **Visualize**:
   - Toggle X, Y, Z components
   - Hover over plots for detailed values
   - Export plots as PNG images

### Parameter Reference

#### Survey Configuration

- **Transmitter Height**: Altitude of EM transmitter above surface (meters)
- **Profile Length**: Half-width of survey transect (meters)
- **Dipole Moment**: Transmitter magnetic moment magnitude (A·m²)
- **Pulse Length**: Duration of transmitter pulse (seconds)
- **Period**: Pulse repetition period (seconds)

#### Target Parameters

- **Sphere Radius**: Size of conductive body (meters)
- **Sphere Conductivity**: Electrical conductivity (S/m)
- **Sphere Position**: 3D coordinates (z-positive downward)

#### Overburden Layer

- **Conductivity**: Overburden electrical conductivity (S/m)
- **Thickness**: Overburden layer depth (meters)

#### Geological Orientation

- **Dip**: Angle from horizontal (0-90°)
- **Strike**: Azimuth direction (0-360°)

## Physics Background

### Calculation Method

The simulator implements a first-order approximation for the electromagnetic response of a conducting sphere in a layered half-space:

1. **Primary Field**: Magnetic dipole field from transmitter
2. **Induced Moment**: Polarization of sphere by primary field
3. **Secondary Field**: Dipole field from induced sphere moment
4. **Overburden Effects**: Reflection/transmission through conductive layer
5. **Time-Domain Response**: Transient decay after pulse turnoff

### Time Windows

The application uses 11 standard time windows based on geophysical literature:

- 154 μs to 9.01 ms after pulse turnoff
- Logarithmically spaced for optimal transient characterization

### Coordinate System

- **X-axis**: Along survey profile (horizontal)
- **Y-axis**: Cross-line (horizontal, perpendicular to profile)
- **Z-axis**: Depth (positive downward into earth)

## Performance

- **Calculation Time**: 1-5 seconds for 201 profile points
- **WASM Module Size**: ~100-200 KB (compressed)
- **Memory Usage**: < 50 MB typical
- **Browser Compatibility**: Modern browsers with WebAssembly support

## Troubleshooting

### WASM Module Fails to Load

**Problem**: "Failed to load calculation engine" error

**Solutions**:
- **Most Common**: Ensure the application is served via HTTP/HTTPS (not file://)
  - You CANNOT open `index.html` directly in browser - it won't work
  - You MUST use a web server (see Local Development section above)
- Check browser console (F12) for detailed error messages
- Verify `frontend/pkg/` directory contains WASM files after building
- Try rebuilding (Windows: `build.bat`, Linux/Mac: `./build.sh`)

### Build Fails on Windows

**Problem**: `wasm-pack build` fails or gives errors

**Solutions**:
- **Restart your terminal/PowerShell** after installing Rust (this is important!)
- Verify Rust is in PATH:
  ```cmd
  rustc --version
  cargo --version
  ```
- If commands not found, add to PATH manually:
  - Default location: `C:\Users\YourName\.cargo\bin`
  - Add to System Environment Variables → Path
- Update Rust: `rustup update`
- Update wasm-pack: `cargo install wasm-pack --force`
- Check internet connection (needs to download dependencies)
- Clear Rust cache: `cargo clean` (from the `wasm` directory)
- **Windows Defender/Antivirus**: May block Rust compilation. Add exclusion for:
  - `C:\Users\YourName\.cargo`
  - Your project directory

### Python Server Won't Start (Windows)

**Problem**: `python -m http.server` doesn't work

**Solutions**:
- Try `python` instead of `python3` on Windows
- Try `py -m http.server 8000`
- If Python not installed:
  - Download from https://www.python.org/downloads/
  - During install, check "Add Python to PATH"
- Alternative: Use VS Code Live Server (see Local Development section)
- Alternative: Use Node.js http-server: `npx http-server dist`

### Port Already in Use

**Problem**: `Address already in use` or `Port 8000 is already in use`

**Solutions**:
- Use a different port:
  ```bash
  python -m http.server 8001 --directory dist
  ```
- Find and stop the process using port 8000:

  **Windows:**
  ```cmd
  netstat -ano | findstr :8000
  taskkill /PID <PID_NUMBER> /F
  ```

  **Linux/Mac:**
  ```bash
  lsof -ti:8000 | xargs kill -9
  ```

### Plots Don't Display

**Problem**: Calculation succeeds but no plots appear

**Solutions**:
- Check browser console (F12) for JavaScript errors
- Verify Plotly.js is loading (check Network tab in F12)
- Ensure component toggles (X, Y, Z) are checked in the interface
- Try refreshing the page (Ctrl+F5 for hard refresh)
- Clear browser cache

### Long Build Times

**Problem**: `wasm-pack build` takes a very long time (first time)

**This is normal!**
- First build downloads and compiles all dependencies (~5-15 minutes)
- Subsequent builds are much faster (30 seconds to 2 minutes)
- Release builds with optimizations take longer than debug builds
- Be patient, especially on slower machines

## Development

### Modifying Calculations

The core physics is in `wasm/src/lib.rs`. After changes:

```bash
cd wasm
wasm-pack build --target web --release --out-dir ../frontend/pkg
```

### Modifying Interface

Frontend files are in `frontend/`:
- `index.html` - Structure
- `styles.css` - Styling
- `app.js` - Logic

No rebuild needed for frontend changes - just refresh browser.

### Testing Changes

Always test locally before deploying:

```bash
./build.sh
python3 -m http.server 8000 --directory dist
```

## References

### Scientific Background

- Geophysical electromagnetic induction theory
- Time-domain induced polarization methods
- Sphere scattering in conducting media

### Technical Documentation

- [WebAssembly](https://webassembly.org/)
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
- [Plotly.js](https://plotly.com/javascript/)
- [Cloudflare Pages](https://pages.cloudflare.com/)

## License

[Add your license information here]

## Contact

[Add contact information here]

## Acknowledgments

This application modernizes electromagnetic modeling tools for web-based deployment, making geophysical simulations more accessible to researchers and practitioners.
