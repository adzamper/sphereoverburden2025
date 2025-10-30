use actix_web::{post, web, HttpResponse};
use nalgebra as na;
use serde::{Deserialize, Serialize};
use crate::electromagnetic::*;

#[derive(Deserialize, Clone)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl From<Vector3> for na::Vector3<f64> {
    fn from(v: Vector3) -> Self {
        na::Vector3::new(v.x, v.y, v.z)
    }
}

#[derive(Deserialize)]
pub struct CalculationParams {
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

#[derive(Serialize)]
pub struct ResponseData {
    x_values: Vec<f64>,
    time_windows: Vec<f64>,
    x_components: Vec<Vec<f64>>,
    y_components: Vec<Vec<f64>>,
    z_components: Vec<Vec<f64>>,
}

#[post("/calculate")]
pub async fn handle_calculation(params: web::Json<CalculationParams>) -> HttpResponse {
    // Time windows from paper
    let wc = vec![
        0.000154600000000000, 0.000236000000000000,
        0.000333700000000000, 0.000447600000000000,
        0.000577800000000000, 0.000740600000000000,
        0.000944000000000000, 0.00118820000000000,
        0.00151370000000000, 0.00192060000000000,
        0.00253090000000000, 0.00334470000000000,
        0.00456540000000000, 0.00619300000000000,
        0.00901430000000000
    ];

    let n_points = 201; // Increased number of points for smoother curves
    let profile_length = params.profile_length;
    let x_range: Vec<f64> = (0..n_points)
        .map(|i| -profile_length + (2.0 * profile_length * i as f64 / (n_points - 1) as f64))
        .collect();

    let em_params = Parameters {
        radar: params.radar,
        mu: params.mu,
        dipole_m: params.dipole_m,
        rtxrx: params.rtxrx.clone().into(),
        rsp: params.rsp.clone().into(),
        a: params.a,
        sigma_sp: params.sigma_sp,
        mtx: params.mtx.clone().into(),
        sigma_ob: params.sigma_ob,
        thick_ob: params.thick_ob,
        apply_dip: params.apply_dip,
        strike: params.strike,
        dip: params.dip,
        base_freq: params.base_freq,
        period: params.period,
        pulse_length: params.pulse_length,
        xsign_negative: params.xsign_negative,
    };

    let mut x_components = vec![vec![0.0; n_points]; wc.len()];
    let mut y_components = vec![vec![0.0; n_points]; wc.len()];
    let mut z_components = vec![vec![0.0; n_points]; wc.len()];

    for (i, &x) in x_range.iter().enumerate() {
        let response_vec = crate::electromagnetic::calculate_response(x, &wc, &em_params);
        for (j, response) in response_vec.iter().enumerate() {
            x_components[j][i] = response.x;
            y_components[j][i] = response.y;
            z_components[j][i] = response.z;
        }
    }

    let response_data = ResponseData {
        x_values: x_range,
        time_windows: wc,
        x_components,
        y_components,
        z_components,
    };

    HttpResponse::Ok().json(response_data)
}