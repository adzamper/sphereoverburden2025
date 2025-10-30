// Electromagnetic Field Response Simulator - WebAssembly Module
//
// This module computes time-domain electromagnetic (EM) field responses from
// buried conductive spheres in layered earth models. It implements first-order
// approximation methods for transient induced polarization (IP) surveying.
//
// The calculations account for:
// - Spherical conductor response in a conducting medium
// - Overburden layer effects using image theory
// - Optional dip/strike rotation for tilted geological targets
// - Time-domain transient decay at multiple time windows

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use nalgebra as na;
use std::f64::consts::PI;

/// 3D vector structure for JavaScript interface
/// Represents positions, offsets, and field directions in 3D space
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Vector3 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
}

impl From<Vector3> for na::Vector3<f64> {
    fn from(v: Vector3) -> Self {
        na::Vector3::new(v.x, v.y, v.z)
    }
}

/// Complete set of electromagnetic survey parameters
/// All units are SI unless otherwise noted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameters {
    /// Transmitter height above surface (meters)
    pub radar: f64,

    /// Magnetic permeability (H/m) - typically μ₀ = 4π × 10⁻⁷
    pub mu: f64,

    /// Transmitter dipole moment magnitude (A·m²)
    pub dipole_m: f64,

    /// Transmitter-Receiver offset vector (meters)
    pub rtxrx: na::Vector3<f64>,

    /// Sphere center position (meters, z-positive downward)
    pub rsp: na::Vector3<f64>,

    /// Sphere radius (meters)
    pub a: f64,

    /// Sphere conductivity (S/m)
    pub sigma_sp: f64,

    /// Magnetic dipole moment direction (unit vector)
    pub mtx: na::Vector3<f64>,

    /// Overburden layer conductivity (S/m)
    pub sigma_ob: f64,

    /// Overburden layer thickness (meters)
    pub thick_ob: f64,

    /// Apply dip/strike rotation for tilted targets
    pub apply_dip: bool,

    /// Strike angle (degrees, 0-360)
    pub strike: f64,

    /// Dip angle (degrees, 0-90)
    pub dip: f64,

    /// Base frequency for survey (Hz)
    pub base_freq: f64,

    /// Sign convention flag for different survey geometries
    pub xsign_negative: bool,

    /// Transmitter pulse length (seconds)
    pub pulse_length: f64,

    /// Pulse repetition period (seconds)
    pub period: f64,
}

/// Three-component magnetic field measurement
/// Values are in nanoTesla (nT) after scaling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldComponents {
    pub x: f64,  // Horizontal inline component
    pub y: f64,  // Horizontal crossline component
    pub z: f64,  // Vertical component (positive downward)
}

/// Complete electromagnetic response data structure
/// Contains field responses at all profile positions and time windows
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseData {
    /// Profile position coordinates (meters)
    x_values: Vec<f64>,

    /// Measurement time windows (seconds after pulse turnoff)
    time_windows: Vec<f64>,

    /// X-component field at each position and time (nT)
    /// Organized as [time_window][position]
    x_components: Vec<Vec<f64>>,

    /// Y-component field (nT)
    y_components: Vec<Vec<f64>>,

    /// Z-component field (nT)
    z_components: Vec<Vec<f64>>,
}

#[wasm_bindgen]
impl ResponseData {
    /// Export response data as JSON string for JavaScript consumption
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(self)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

/// Calculate static magnetic dipole field in free space
///
/// Uses the classic dipole field formula:
/// H(r) = (1/4π) * [3(m·r̂)r̂ - m] / r³
///
/// # Arguments
/// * `m` - Magnetic dipole moment vector (A·m²)
/// * `r` - Position vector from dipole to observation point (m)
///
/// # Returns
/// Magnetic field vector (A/m)
fn static_field(m: &na::Vector3<f64>, r: &na::Vector3<f64>) -> na::Vector3<f64> {
    let one_over_4pi = 1.0 / (4.0 * PI);
    let r2 = r.norm_squared();

    // Avoid singularity at dipole location
    if r2 < 1e-20 {
        return na::Vector3::zeros();
    }

    let a = one_over_4pi / (r2.sqrt() * r2);
    let b = m.dot(r) * 3.0 / r2;
    (b * r - m) * a
}

/// Calculate overburden layer field response using image theory
///
/// Models the reflected field from a conductive overburden layer.
/// Uses image dipole method to account for the conductivity contrast.
/// Handles two cases: receiver above or below overburden.
///
/// # Arguments
/// * `mtx` - Transmitter dipole moment direction
/// * `dipole_m` - Dipole moment magnitude (A·m²)
/// * `rtx` - Transmitter position (m)
/// * `rrx` - Receiver position (m)
/// * `O` - Time parameter for transient response (s)
/// * `mu` - Magnetic permeability (H/m)
/// * `sigma_ob` - Overburden conductivity (S/m)
/// * `thick_ob` - Overburden thickness (m)
///
/// # Returns
/// Overburden field contribution (A/m)
pub fn h_ob_xyz(
    mtx: &na::Vector3<f64>,
    dipole_m: f64,
    rtx: &na::Vector3<f64>,
    rrx: &na::Vector3<f64>,
    O: f64,
    mu: f64,
    sigma_ob: f64,
    thick_ob: f64,
) -> na::Vector3<f64> {
    // Extract moment components
    let m_x = dipole_m * mtx[0];
    let m_y = dipole_m * mtx[1];
    let m_z = dipole_m * mtx[2];

    // Extract position components
    let rtx_x = rtx[0];
    let rtx_y = rtx[1];
    let rtx_z = rtx[2];
    let rrx_x = rrx[0];
    let rrx_y = rrx[1];
    let rrx_z = rrx[2];

    // Image depth parameter - accounts for skin depth in overburden
    let image_correction = (2.0 * O) / (mu * sigma_ob * thick_ob);

    // Calculate X-component field
    // Different formulas for receiver above (z > 0) or below (z < 0) overburden
    let h_obx = if rrx_z > 0.0 {
        // Receiver above overburden
        let dx = rrx_x - rtx_x;
        let dy = rrx_y - rtx_y;
        let dz = rrx_z + rtx_z + image_correction;
        let r2 = dx * dx + dy * dy + dz * dz;
        let r_pow_1_5 = r2.powf(1.5);
        let r_pow_2_5 = r2.powf(2.5);
        let m_dot_r = m_x * dx + m_y * dy - m_z * dz;

        (-1.0 / (4.0 * PI)) * (
            m_x / r_pow_1_5 -
            (3.0 * (2.0 * dx) * m_dot_r) / (2.0 * r_pow_2_5)
        )
    } else {
        // Receiver below overburden
        let dx = rrx_x - rtx_x;
        let dy = rrx_y - rtx_y;
        let dz = rtx_z - rrx_z + image_correction;
        let r2 = dx * dx + dy * dy + dz * dz;
        let r_pow_1_5 = r2.powf(1.5);
        let r_pow_2_5 = r2.powf(2.5);
        let m_dot_r = m_x * dx + m_y * dy - m_z * dz;

        (-1.0 / (4.0 * PI)) * (
            m_x / r_pow_1_5 -
            (3.0 * (2.0 * dx) * m_dot_r) / (2.0 * r_pow_2_5)
        )
    };

    // Calculate Z-component field
    let h_obz = if rrx_z > 0.0 {
        // Receiver above overburden
        let dx = rrx_x - rtx_x;
        let dy = rrx_y - rtx_y;
        let dz = rrx_z + rtx_z + image_correction;
        let r2 = dx * dx + dy * dy + dz * dz;
        let r_pow_1_5 = r2.powf(1.5);
        let r_pow_2_5 = r2.powf(2.5);
        let m_dot_r = m_x * dx + m_y * dy - m_z * dz;

        (-1.0 / (4.0 * PI)) * (
            -m_z / r_pow_1_5 -
            (3.0 * (2.0 * dz) * m_dot_r) / (2.0 * r_pow_2_5)
        )
    } else {
        // Receiver below overburden
        let dx = rrx_x - rtx_x;
        let dy = rrx_y - rtx_y;
        let dz = rtx_z - rrx_z + image_correction;
        let r2 = dx * dx + dy * dy + dz * dz;
        let r_pow_1_5 = r2.powf(1.5);
        let r_pow_2_5 = r2.powf(2.5);
        let m_dot_r = m_x * dx + m_y * dy - m_z * dz;

        (-1.0 / (4.0 * PI)) * (
            m_z / r_pow_1_5 +
            (3.0 * (2.0 * dz) * m_dot_r) / (2.0 * r_pow_2_5)
        )
    };

    // Y-component is typically zero for inline surveys
    na::Vector3::new(h_obx, 0.0, h_obz)
}

/// Calculate time derivative of overburden field response
///
/// Computes ∂H_ob/∂t for transient response calculations.
/// This is used in the convolution integral for time-domain modeling.
///
/// # Returns
/// Time derivative of overburden field (A/m/s)
pub fn dh_obdt_xyz(
    mtx: &na::Vector3<f64>,
    dipole_m: f64,
    rtx: &na::Vector3<f64>,
    rrx: &na::Vector3<f64>,
    O: f64,
    mu: f64,
    sigma_ob: f64,
    thick_ob: f64,
) -> na::Vector3<f64> {
    let m_x = dipole_m * mtx[0];
    let m_y = dipole_m * mtx[1];
    let m_z = dipole_m * mtx[2];
    let rtx_x = rtx[0];
    let rtx_y = rtx[1];
    let rtx_z = rtx[2];
    let rrx_x = rrx[0];
    let rrx_y = rrx[1];
    let rrx_z = rrx[2];

    let image_correction = (2.0 * O) / (mu * sigma_ob * thick_ob);
    let denom = mu * sigma_ob * thick_ob;

    // X-component time derivative
    let dh_obx = if rrx_z > 0.0 {
        let dx = rrx_x - rtx_x;
        let dy = rrx_y - rtx_y;
        let dz = rrx_z + rtx_z + image_correction;
        let r2 = dx * dx + dy * dy + dz * dz;
        let r_pow_2_5 = r2.powf(2.5);
        let r_pow_3_5 = r2.powf(3.5);
        let m_dot_r = m_x * dx + m_y * dy - m_z * dz;

        (-1.0 / (4.0 * PI)) * (
            (m_z * (6.0 * dx)) / (denom * r_pow_2_5) -
            (6.0 * m_x * dz) / (denom * r_pow_2_5) +
            (5.0 * (6.0 * dx) * dz * m_dot_r) / (denom * r_pow_3_5)
        )
    } else {
        let dx = rrx_x - rtx_x;
        let dy = rrx_y - rtx_y;
        let dz = rtx_z - rrx_z + image_correction;
        let r2 = dx * dx + dy * dy + dz * dz;
        let r_pow_2_5 = r2.powf(2.5);
        let r_pow_3_5 = r2.powf(3.5);
        let m_dot_r = m_x * dx + m_y * dy - m_z * dz;

        (-1.0 / (4.0 * PI)) * (
            (m_z * (6.0 * dx)) / (denom * r_pow_2_5) -
            (6.0 * m_x * dz) / (denom * r_pow_2_5) +
            (5.0 * (6.0 * dx) * dz * m_dot_r) / (denom * r_pow_3_5)
        )
    };

    // Z-component time derivative
    let dh_obz = if rrx_z > 0.0 {
        let dx = rrx_x - rtx_x;
        let dy = rrx_y - rtx_y;
        let dz = rrx_z + rtx_z + image_correction;
        let r2 = dx * dx + dy * dy + dz * dz;
        let r_pow_2_5 = r2.powf(2.5);
        let r_pow_3_5 = r2.powf(3.5);
        let m_dot_r = m_x * dx + m_y * dy - m_z * dz;

        (-1.0 / (4.0 * PI)) * (
            (6.0 * m_z * dz) / (denom * r_pow_2_5) -
            (6.0 * m_dot_r) / (denom * r_pow_2_5) +
            (m_z * (6.0 * dz + 6.0 * image_correction)) / (denom * r_pow_2_5) +
            (5.0 * (6.0 * dz + 6.0 * image_correction) * dz * m_dot_r) / (denom * r_pow_3_5)
        )
    } else {
        let dx = rrx_x - rtx_x;
        let dy = rrx_y - rtx_y;
        let dz = rtx_z - rrx_z + image_correction;
        let r2 = dx * dx + dy * dy + dz * dz;
        let r_pow_2_5 = r2.powf(2.5);
        let r_pow_3_5 = r2.powf(3.5);
        let m_dot_r = m_x * dx + m_y * dy - m_z * dz;

        (-1.0 / (4.0 * PI)) * (
            (6.0 * m_dot_r) / (denom * r_pow_2_5) -
            (m_z * (6.0 * dz + 6.0 * image_correction)) / (denom * r_pow_2_5) -
            (6.0 * m_z * dz) / (denom * r_pow_2_5) -
            (5.0 * (6.0 * dz + 6.0 * image_correction) * dz * m_dot_r) / (denom * r_pow_3_5)
        )
    };

    na::Vector3::new(dh_obx, 0.0, dh_obz)
}

/// Theta function for sphere transient response
///
/// Computes the step-response function for a conducting sphere using
/// an infinite series expansion. This function represents the polarization
/// buildup/decay in the sphere as a function of time.
///
/// The series converges rapidly for most practical time ranges.
/// Maximum 100 terms are evaluated with convergence checking.
///
/// # Arguments
/// * `t` - Observation time (s)
/// * `O` - Reference time offset (s)
/// * `o` - Pulse turnoff time (s)
/// * `mu` - Magnetic permeability (H/m)
/// * `sigma_sp` - Sphere conductivity (S/m)
/// * `a` - Sphere radius (m)
/// * `T` - Pulse length (s)
///
/// # Returns
/// Dimensionless theta function value (0 to ~1)
pub fn thetafunction_step(
    t: f64,
    O: f64,
    o: f64,
    mu: f64,
    sigma_sp: f64,
    a: f64,
    T: f64,
) -> f64 {
    // Characteristic time constant for sphere
    let ss = mu * sigma_sp * a * a;

    let mut theta = 0.0;
    let mut solver = 0.0;
    let mut k = 0;

    // Sum infinite series until convergence
    while solver < 1.0 {
        k += 1;
        let k_pi = (k as f64) * PI;

        // Series term accounts for pulse excitation and exponential decay
        let excitation_factor = 1.0 / (1.0 + (-T / 2.0 * k_pi.powi(2) / ss).exp());
        let decay_factor = (6.0 / k_pi.powi(2)) * ((o + O - t) * k_pi.powi(2) / ss).exp();
        let temp = excitation_factor * decay_factor;

        theta += temp;

        // Check relative convergence
        solver = temp / theta;

        if k > 1 && temp.abs() < theta.abs() * 1e-10 {
            break; // Converged to 10 significant figures
        }

        if k > 100 {
            break; // Safety limit on iterations
        }
    }

    theta
}

/// Calculate total sphere response using convolution integral
///
/// Integrates the overburden field time derivative convolved with the
/// sphere response function to get the total transient response.
/// Uses trapezoidal rule numerical integration with 100 intervals.
///
/// # Returns
/// Total transient field response from sphere (A/m)
pub fn dh_tot_step(
    mtx: &na::Vector3<f64>,
    dipole_m: f64,
    rtx: &na::Vector3<f64>,
    rsp: &na::Vector3<f64>,
    mu: f64,
    sigma_ob: f64,
    thick_ob: f64,
    t: f64,
    o: f64,
    sigma_sp: f64,
    a: f64,
    T: f64,
) -> na::Vector3<f64> {
    // Get overburden field at sphere location
    let ob_array = h_ob_xyz(mtx, dipole_m, rtx, rsp, -o, mu, sigma_ob, thick_ob);
    let thetaz = thetafunction_step(t, 0.0, o, mu, sigma_sp, a, T);

    // Numerical integration setup
    let n = 100;
    let dt = (t - o) / n as f64;
    let mut resultx = 0.0;
    let mut resultz = 0.0;

    // Trapezoidal integration over time from pulse turnoff to observation time
    for i in 0..=n {
        let tau = o + (i as f64) * dt;

        // Trapezoidal rule weights: 0.5 for endpoints, 1.0 for interior
        let weight = if i == 0 || i == n { 0.5 } else { 1.0 };

        let dh_ob = dh_obdt_xyz(mtx, dipole_m, rtx, rsp, tau, mu, sigma_ob, thick_ob);
        let theta = thetafunction_step(t, tau, o, mu, sigma_sp, a, T);

        // Convolution: -∫ (∂H/∂τ) * θ(t-τ) dτ
        resultx -= weight * dh_ob.x * theta;
        resultz -= weight * dh_ob.z * theta;
    }

    resultx *= dt;
    resultz *= dt;

    // Add static term (field at observation time convolved with theta)
    na::Vector3::new(
        resultx + (ob_array.x * thetaz),
        0.0,
        resultz + (ob_array.z * thetaz),
    )
}

/// Calculate complete first-order electromagnetic response
///
/// Main calculation function that combines:
/// 1. Induced dipole moment in the sphere
/// 2. Static dipole field from induced moment
/// 3. Overburden reflection effects
/// 4. Optional dip/strike rotation for geological targets
///
/// # Returns
/// Three-component magnetic field (A/m) scaled by μ/10^-12 to get nT
pub fn h_total_step_1storder(
    mtx: &na::Vector3<f64>,
    dipole_m: f64,
    rtx: &na::Vector3<f64>,
    offset_tx_rx: &na::Vector3<f64>,
    rsp: &na::Vector3<f64>,
    t: f64,
    mu: f64,
    sigma_ob: f64,
    thick_ob: f64,
    sigma_sp: f64,
    a: f64,
    P: f64,
    apply_dip: bool,
    dip: f64,
    strike: f64,
    _wave: f64,
    T: f64,
    xsign: bool,
) -> na::Vector3<f64> {
    // Calculate induced magnetic moment in sphere
    // Factor of 2π comes from sphere geometry in first-order approximation
    let moment = 2.0 * PI * a.powi(3)
        * dh_tot_step(mtx, dipole_m, rtx, rsp, mu, sigma_ob, thick_ob, t, 0.0, sigma_sp, a, T);

    // Apply dip/strike rotation if modeling tilted geological body
    let msp = if apply_dip {
        // Convert geological angles to normal vector
        let dip_rad = (90.0 - dip) * PI / 180.0;
        let strike_rad = (strike - 90.0) * PI / 180.0;
        let norm = na::Vector3::new(
            dip_rad.cos() * strike_rad.cos(),
            strike_rad.sin() * dip_rad.cos(),
            dip_rad.sin(),
        )
        .normalize();

        // Project moment onto dip direction
        let mspdotnorm = moment.dot(&norm);
        norm * mspdotnorm
    } else {
        moment
    };

    // Calculate receiver position relative to sphere
    let offset = na::Vector3::new(
        -offset_tx_rx[0],
        -offset_tx_rx[1],
        rtx[2] - offset_tx_rx[2],
    ) - na::Vector3::new(-rtx[0], -rtx[1], rsp[2]);

    // Static field from induced moment
    let statics = static_field(&msp, &offset);

    // Apply sign convention for different survey configurations
    let h_tot = if xsign {
        na::Vector3::new(-(statics.x), statics.y, statics.z)
    } else {
        statics
    };

    // Calculate overburden field at receiver
    let h_ob = h_ob_xyz(
        mtx,
        dipole_m,
        &na::Vector3::new(0.0, 0.0, rtx[2]),
        &na::Vector3::new(
            -offset_tx_rx[0],
            -offset_tx_rx[1],
            rtx[2] - offset_tx_rx[2],
        ),
        t,
        mu,
        sigma_ob,
        thick_ob,
    );

    // Combine sphere and overburden responses
    // Note: Z-component has different sign convention due to coordinate system
    if xsign {
        na::Vector3::new(
            -(h_tot.x + h_ob.x),
            h_tot.y + h_ob.y,
            h_tot.z - h_ob.z,
        )
    } else {
        na::Vector3::new(h_tot.x + h_ob.x, h_tot.y + h_ob.y, h_tot.z - h_ob.z)
    }
}

/// Calculate electromagnetic response at a single profile position
///
/// Loops over all time windows and computes the three-component field response.
/// Output is scaled to nanoTesla (nT) units.
///
/// # Arguments
/// * `x` - Profile position coordinate (m)
/// * `wc` - Array of time windows (s)
/// * `params` - Complete parameter set
///
/// # Returns
/// Vector of field components for each time window
pub fn calculate_response(x: f64, wc: &[f64], params: &Parameters) -> Vec<FieldComponents> {
    let mut results = Vec::with_capacity(wc.len());

    // Transmitter position along profile
    let rtx = na::Vector3::new(x, 0.0, params.radar);
    let wave = 1.0; // Default wave parameter
    let P = params.pulse_length;

    // Calculate response at each time window
    for &t in wc {
        let response = h_total_step_1storder(
            &params.mtx,
            params.dipole_m,
            &rtx,
            &params.rtxrx,
            &params.rsp,
            t,
            params.mu,
            params.sigma_ob,
            params.thick_ob,
            params.sigma_sp,
            params.a,
            P,
            params.apply_dip,
            params.dip,
            params.strike,
            wave,
            params.period,
            params.xsign_negative,
        );

        // Scale to nanoTesla: multiply by μ and convert to nT (×10^12 / 10^-12 = 1)
        results.push(FieldComponents {
            x: (params.mu / 1e-12) * response.x,
            y: (params.mu / 1e-12) * response.y,
            z: (params.mu / 1e-12) * response.z,
        });
    }

    results
}

/// Main WASM entry point for electromagnetic calculation
///
/// Computes the complete EM response along a 2D survey profile for all time windows.
/// This is the function called from JavaScript.
///
/// # Arguments
/// * `params_json` - JSON string containing all survey parameters
///
/// # Returns
/// ResponseData containing field responses at 201 profile positions and 15 time windows
#[wasm_bindgen]
pub fn calculate_em_response(params_json: &str) -> Result<ResponseData, JsValue> {
    // Parse input parameters from JSON
    #[derive(Deserialize)]
    struct InputParams {
        radar: f64,
        mu: f64,
        dipole_m: f64,
        rtxrx: Vector3,
        rsp: Vector3,
        a: f64,
        sigma_sp: f64,
        mtx: Vector3,
        sigma_ob: f64,
        thick_ob: f64,
        apply_dip: bool,
        strike: f64,
        dip: f64,
        base_freq: f64,
        period: f64,
        pulse_length: f64,
        xsign_negative: bool,
        profile_length: f64,
    }

    let input: InputParams = serde_json::from_str(params_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

    // Convert to internal parameter structure
    let params = Parameters {
        radar: input.radar,
        mu: input.mu,
        dipole_m: input.dipole_m,
        rtxrx: input.rtxrx.into(),
        rsp: input.rsp.into(),
        a: input.a,
        sigma_sp: input.sigma_sp,
        mtx: input.mtx.into(),
        sigma_ob: input.sigma_ob,
        thick_ob: input.thick_ob,
        apply_dip: input.apply_dip,
        strike: input.strike,
        dip: input.dip,
        base_freq: input.base_freq,
        xsign_negative: input.xsign_negative,
        pulse_length: input.pulse_length,
        period: input.period,
    };

    // Standard time windows from geophysical literature (seconds)
    // These represent the gates at which the transient decay is measured
    let time_windows = vec![
        0.000154, 0.000236, 0.000361, 0.000553, 0.000847, 0.001297, 0.001986, 0.003041, 0.004657,
        0.007129, 0.009014,
    ];

    // Generate 201 evenly-spaced profile positions
    let num_points = 201;
    let x_min = -input.profile_length;
    let x_max = input.profile_length;
    let dx = (x_max - x_min) / (num_points - 1) as f64;

    let mut x_values = Vec::with_capacity(num_points);
    let mut x_components = vec![Vec::with_capacity(num_points); time_windows.len()];
    let mut y_components = vec![Vec::with_capacity(num_points); time_windows.len()];
    let mut z_components = vec![Vec::with_capacity(num_points); time_windows.len()];

    // Calculate response at each profile position
    for i in 0..num_points {
        let x = x_min + (i as f64) * dx;
        x_values.push(x);

        // Get field components for all time windows at this position
        let responses = calculate_response(x, &time_windows, &params);

        // Organize data by time window (transpose structure)
        for (time_idx, response) in responses.iter().enumerate() {
            x_components[time_idx].push(response.x);
            y_components[time_idx].push(response.y);
            z_components[time_idx].push(response.z);
        }
    }

    Ok(ResponseData {
        x_values,
        time_windows,
        x_components,
        y_components,
        z_components,
    })
}
