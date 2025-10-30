/**
 * Electromagnetic Field Response Simulator - Application Logic
 *
 * This module handles:
 * - Loading the WASM module
 * - Collecting user input parameters
 * - Calling WASM calculation functions
 * - Visualizing results with Plotly
 *
 * The application uses WebAssembly for high-performance electromagnetic
 * field calculations compiled from Rust code.
 */

// ========== Global State ==========

let wasmModule = null;
let currentResponse = null;

// Physical constants
const MU_0 = 1.2566370614359172e-6; // Magnetic permeability of free space (H/m)

// ========== WASM Module Loading ==========

/**
 * Load and initialize the WebAssembly module
 * The WASM file should be built from the Rust source using wasm-pack
 */
async function initWasm() {
    try {
        // Import the WASM module
        const wasm = await import('./pkg/sphere_overburden_wasm.js');

        // Initialize the WASM module (critical step!)
        await wasm.default();

        // Store the initialized module
        wasmModule = wasm;

        console.log('WASM module loaded and initialized successfully');
        document.getElementById('calculateBtn').disabled = false;

        return true;
    } catch (error) {
        console.error('Failed to load WASM module:', error);

        // Show error to user
        showError(
            'Failed to load calculation engine. ' +
            'Please ensure the WebAssembly module is built and available. ' +
            'Error: ' + error.message
        );

        // Disable calculate button
        document.getElementById('calculateBtn').disabled = true;

        return false;
    }
}

// ========== Parameter Collection ==========

/**
 * Collect all input parameters from the form
 * Returns an object matching the Rust Parameters structure
 */
function collectParameters() {
    return {
        // Survey configuration
        radar: parseFloat(document.getElementById('radar').value),
        mu: MU_0, // Use constant magnetic permeability
        dipole_m: parseFloat(document.getElementById('dipole_m').value),
        base_freq: parseFloat(document.getElementById('base_freq').value),
        period: parseFloat(document.getElementById('period').value),
        pulse_length: parseFloat(document.getElementById('pulse_length').value),
        profile_length: parseFloat(document.getElementById('profile_length').value),

        // Transmitter-Receiver offset
        rtxrx: {
            x: parseFloat(document.getElementById('rtxrx_x').value),
            y: parseFloat(document.getElementById('rtxrx_y').value),
            z: parseFloat(document.getElementById('rtxrx_z').value),
        },

        // Dipole moment direction (should be unit vector)
        mtx: {
            x: parseFloat(document.getElementById('mtx_x').value),
            y: parseFloat(document.getElementById('mtx_y').value),
            z: parseFloat(document.getElementById('mtx_z').value),
        },

        // Sphere parameters
        a: parseFloat(document.getElementById('a').value),
        sigma_sp: parseFloat(document.getElementById('sigma_sp').value),
        rsp: {
            x: parseFloat(document.getElementById('rsp_x').value),
            y: parseFloat(document.getElementById('rsp_y').value),
            z: parseFloat(document.getElementById('rsp_z').value),
        },

        // Overburden parameters
        sigma_ob: parseFloat(document.getElementById('sigma_ob').value),
        thick_ob: parseFloat(document.getElementById('thick_ob').value),

        // Geological orientation
        apply_dip: document.getElementById('apply_dip').checked,
        strike: parseFloat(document.getElementById('strike').value),
        dip: parseFloat(document.getElementById('dip').value),

        // Advanced options
        xsign_negative: document.getElementById('xsign_negative').checked,
    };
}

/**
 * Validate parameters to ensure physical consistency
 * Returns error message if invalid, null if valid
 */
function validateParameters(params) {
    // Check for NaN values
    const numericFields = [
        'radar', 'mu', 'dipole_m', 'base_freq', 'period', 'pulse_length',
        'profile_length', 'a', 'sigma_sp', 'sigma_ob', 'thick_ob', 'strike', 'dip'
    ];

    for (const field of numericFields) {
        if (isNaN(params[field]) || !isFinite(params[field])) {
            return `Invalid value for ${field}`;
        }
    }

    // Check vector fields
    if (isNaN(params.rtxrx.x) || isNaN(params.rtxrx.y) || isNaN(params.rtxrx.z)) {
        return 'Invalid Tx-Rx offset vector';
    }

    if (isNaN(params.rsp.x) || isNaN(params.rsp.y) || isNaN(params.rsp.z)) {
        return 'Invalid sphere position vector';
    }

    if (isNaN(params.mtx.x) || isNaN(params.mtx.y) || isNaN(params.mtx.z)) {
        return 'Invalid dipole moment direction';
    }

    // Check physical constraints
    if (params.a <= 0) {
        return 'Sphere radius must be positive';
    }

    if (params.sigma_sp <= 0) {
        return 'Sphere conductivity must be positive';
    }

    if (params.sigma_ob < 0) {
        return 'Overburden conductivity cannot be negative';
    }

    if (params.thick_ob < 0) {
        return 'Overburden thickness cannot be negative';
    }

    if (params.profile_length <= 0) {
        return 'Profile length must be positive';
    }

    if (params.pulse_length <= 0 || params.period <= 0) {
        return 'Pulse parameters must be positive';
    }

    return null; // Valid
}

// ========== Calculation and Results ==========

/**
 * Main calculation function
 * Collects parameters, calls WASM, and updates visualization
 */
async function calculate() {
    try {
        // Collect and validate parameters
        const params = collectParameters();
        const validationError = validateParameters(params);

        if (validationError) {
            showError(validationError);
            return;
        }

        // Show loading indicator
        showLoading(true);
        hideError();

        // Convert parameters to JSON for WASM
        const paramsJson = JSON.stringify(params);

        // Call WASM calculation function
        // This may take several seconds for the full calculation
        const responseData = wasmModule.calculate_em_response(paramsJson);

        // Parse response
        currentResponse = JSON.parse(responseData.toJSON());

        // Update visualization
        updatePlots();

        console.log('Calculation completed successfully');
    } catch (error) {
        console.error('Calculation error:', error);
        showError('Calculation failed: ' + error.message);
    } finally {
        showLoading(false);
    }
}

// ========== Visualization with Plotly ==========

/**
 * Update all plot displays based on current response data
 */
function updatePlots() {
    if (!currentResponse) {
        return;
    }

    const showX = document.getElementById('showX').checked;
    const showY = document.getElementById('showY').checked;
    const showZ = document.getElementById('showZ').checked;

    // Update X component plot
    if (showX) {
        plotComponent('plotX', 'X', currentResponse.x_components);
        document.getElementById('plotX').style.display = 'block';
    } else {
        document.getElementById('plotX').style.display = 'none';
    }

    // Update Y component plot
    if (showY) {
        plotComponent('plotY', 'Y', currentResponse.y_components);
        document.getElementById('plotY').style.display = 'block';
    } else {
        document.getElementById('plotY').style.display = 'none';
    }

    // Update Z component plot
    if (showZ) {
        plotComponent('plotZ', 'Z', currentResponse.z_components);
        document.getElementById('plotZ').style.display = 'block';
    } else {
        document.getElementById('plotZ').style.display = 'none';
    }
}

/**
 * Create a Plotly plot for a single field component
 *
 * @param {string} containerId - DOM element ID for the plot
 * @param {string} component - Component name (X, Y, or Z)
 * @param {Array<Array<number>>} data - 2D array [time_window][position]
 */
function plotComponent(containerId, component, data) {
    const { x_values, time_windows } = currentResponse;

    // Create a trace for each time window
    const traces = time_windows.map((timeWindow, idx) => {
        return {
            x: x_values,
            y: data[idx],
            mode: 'lines',
            name: formatTime(timeWindow),
            line: {
                width: 2,
            },
            hovertemplate:
                '<b>Position:</b> %{x:.1f} m<br>' +
                '<b>Field:</b> %{y:.4f} nT<br>' +
                '<extra></extra>',
        };
    });

    // Refined slate theme matching the interface
    const layout = {
        title: {
            text: `${component}-Component Magnetic Field Response`,
            font: {
                family: 'Inter, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif',
                size: 14,
                color: '#f0f2f7',
                weight: 600,
            },
            pad: { t: 10, b: 10 },
        },
        xaxis: {
            title: {
                text: 'Profile Position (m)',
                font: {
                    family: 'Inter, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif',
                    size: 12,
                    color: '#adb5c7',
                },
            },
            showgrid: true,
            gridcolor: '#2e3441',
            gridwidth: 1,
            zeroline: true,
            zerolinecolor: '#3c4150',
            zerolinewidth: 2,
            color: '#adb5c7',
        },
        yaxis: {
            title: {
                text: 'Magnetic Field (nT)',
                font: {
                    family: 'Inter, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif',
                    size: 12,
                    color: '#adb5c7',
                },
            },
            showgrid: true,
            gridcolor: '#2e3441',
            gridwidth: 1,
            zeroline: true,
            zerolinecolor: '#3c4150',
            zerolinewidth: 1,
            color: '#adb5c7',
        },
        legend: {
            title: {
                text: 'Time Window',
                font: {
                    family: 'Inter, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif',
                    size: 11,
                    color: '#adb5c7',
                },
            },
            x: 1.02,
            y: 1,
            xanchor: 'left',
            bgcolor: 'rgba(22, 24, 29, 0.95)',
            bordercolor: '#2e3441',
            borderwidth: 1,
            font: {
                color: '#adb5c7',
                size: 10,
            },
        },
        plot_bgcolor: '#0f1419',
        paper_bgcolor: '#16181d',
        font: {
            family: 'Inter, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif',
            size: 11,
            color: '#adb5c7',
        },
        hovermode: 'closest',
        margin: {
            l: 70,
            r: 180,
            t: 50,
            b: 50,
        },
    };

    // Plotly configuration
    const config = {
        responsive: true,
        displayModeBar: true,
        displaylogo: false,
        modeBarButtonsToRemove: ['lasso2d', 'select2d'],
        toImageButtonOptions: {
            format: 'png',
            filename: `em_response_${component.toLowerCase()}`,
            height: 600,
            width: 1000,
            scale: 2,
        },
    };

    // Create or update the plot
    Plotly.newPlot(containerId, traces, layout, config);
}

/**
 * Format time value for display
 * Converts seconds to appropriate units (μs or ms)
 */
function formatTime(seconds) {
    if (seconds < 0.001) {
        // Display in microseconds
        return `${(seconds * 1e6).toFixed(1)} μs`;
    } else {
        // Display in milliseconds
        return `${(seconds * 1e3).toFixed(2)} ms`;
    }
}

// ========== UI Helper Functions ==========

/**
 * Show or hide the loading indicator
 */
function showLoading(show) {
    const loading = document.getElementById('loading');
    const calculateBtn = document.getElementById('calculateBtn');

    loading.style.display = show ? 'block' : 'none';
    calculateBtn.disabled = show;

    if (show) {
        calculateBtn.textContent = 'Calculating...';
    } else {
        calculateBtn.textContent = 'Calculate Response';
    }
}

/**
 * Display an error message
 */
function showError(message) {
    const errorDiv = document.getElementById('error');
    errorDiv.textContent = message;
    errorDiv.style.display = 'block';
}

/**
 * Hide the error message
 */
function hideError() {
    document.getElementById('error').style.display = 'none';
}

// ========== Event Handlers ==========

/**
 * Initialize event listeners
 */
function initEventListeners() {
    // Calculate button
    document.getElementById('calculateBtn').addEventListener('click', calculate);

    // Component toggle checkboxes
    document.getElementById('showX').addEventListener('change', updatePlots);
    document.getElementById('showY').addEventListener('change', updatePlots);
    document.getElementById('showZ').addEventListener('change', updatePlots);

    // Tab switching
    document.querySelectorAll('.tab-btn').forEach((btn) => {
        btn.addEventListener('click', (e) => {
            const tabName = e.target.dataset.tab;
            switchTab(tabName);
        });
    });

    // Enable calculation on Enter key in input fields
    document.querySelectorAll('input[type="number"]').forEach((input) => {
        input.addEventListener('keypress', (event) => {
            if (event.key === 'Enter') {
                calculate();
            }
        });
    });
}

/**
 * Switch to a different parameter tab
 */
function switchTab(tabName) {
    // Remove active class from all tabs and content
    document.querySelectorAll('.tab-btn').forEach((btn) => {
        btn.classList.remove('active');
    });
    document.querySelectorAll('.tab-content').forEach((content) => {
        content.classList.remove('active');
    });

    // Add active class to selected tab and content
    const tabBtn = document.querySelector(`.tab-btn[data-tab="${tabName}"]`);
    const tabContent = document.getElementById(`tab-${tabName}`);

    if (tabBtn && tabContent) {
        tabBtn.classList.add('active');
        tabContent.classList.add('active');
    }
}

// ========== Application Initialization ==========

/**
 * Initialize the application
 * Called when DOM is fully loaded
 */
async function init() {
    console.log('Initializing Electromagnetic Field Response Simulator');

    // Initialize event listeners
    initEventListeners();

    // Initially disable calculate button until WASM loads
    document.getElementById('calculateBtn').disabled = true;
    document.getElementById('calculateBtn').textContent = 'Loading...';

    // Load WASM module
    const wasmLoaded = await initWasm();

    if (wasmLoaded) {
        document.getElementById('calculateBtn').textContent = 'Calculate Response';
        console.log('Application ready');
    } else {
        document.getElementById('calculateBtn').textContent = 'WASM Load Failed';
    }
}

// Start the application when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}

// Export for module systems
export { init, calculate, updatePlots, switchTab };
