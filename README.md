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
├── build.sh               # Build script
└── README.md              # This file
```

## Prerequisites

Before building the application, you need:

1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack** (WASM build tool)
   ```bash
   cargo install wasm-pack
   ```

3. **wasm32 target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Building the Application

### Quick Build

Run the build script:

```bash
./build.sh
```

This will:
1. Compile the Rust code to WebAssembly
2. Generate JavaScript bindings
3. Copy all files to the `dist/` directory

### Manual Build

If you prefer to build manually:

```bash
# Build WASM module
cd wasm
wasm-pack build --target web --release --out-dir ../frontend/pkg
cd ..

# Prepare distribution
mkdir -p dist
cp frontend/index.html dist/
cp frontend/styles.css dist/
cp frontend/app.js dist/
cp -r frontend/pkg dist/
```

## Local Development

To test the application locally, you need a web server (due to ES module and WASM requirements):

### Using Python

```bash
python3 -m http.server 8000 --directory dist
```

Then open http://localhost:8000 in your browser.

### Using Node.js

```bash
npx serve dist
```

### Using PHP

```bash
php -S localhost:8000 -t dist
```

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
- Ensure the application is served via HTTP/HTTPS (not file://)
- Check browser console for detailed error messages
- Verify `frontend/pkg/` directory contains WASM files
- Try rebuilding: `./build.sh`

### Build Fails

**Problem**: `wasm-pack build` fails

**Solutions**:
- Update Rust: `rustup update`
- Update wasm-pack: `cargo install wasm-pack --force`
- Check internet connection (downloads dependencies)
- Clear Rust cache: `cargo clean`

### Plots Don't Display

**Problem**: Calculation succeeds but no plots appear

**Solutions**:
- Check browser console for JavaScript errors
- Verify Plotly.js is loading (check network tab)
- Ensure component toggles (X, Y, Z) are checked
- Try refreshing the page

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
