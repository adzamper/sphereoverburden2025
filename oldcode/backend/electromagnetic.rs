use nalgebra as na;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Parameters {
    pub radar: f64,
    pub mu: f64,
    pub dipole_m: f64,
    pub rtxrx: na::Vector3<f64>,
    pub rsp: na::Vector3<f64>,
    pub a: f64,
    pub sigma_sp: f64,
    pub mtx: na::Vector3<f64>,
    pub sigma_ob: f64,
    pub thick_ob: f64,
    pub apply_dip: bool,
    pub strike: f64,
    pub dip: f64,
    pub base_freq: f64,
    pub xsign_negative: bool,
    pub pulse_length: f64,
    pub period: f64,
}

#[derive(Debug, Clone)]
pub struct FieldComponents {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn static_field(m: &na::Vector3<f64>, r: &na::Vector3<f64>) -> na::Vector3<f64> {
    let one_over_4pi = 1.0 / (4.0 * PI);
    let r2 = r.norm_squared();
    
    if r2 < 1e-20 {
        return na::Vector3::zeros();
    }
    
    let a = one_over_4pi / (r2.sqrt() * r2);
    let b = m.dot(r) * 3.0 / r2;
    (b * r - m) * a
}

pub fn h_ob_xyz(mtx: &na::Vector3<f64>, dipole_m: f64, rtx: &na::Vector3<f64>, 
                rrx: &na::Vector3<f64>, O: f64, mu: f64, sigma_ob: f64, thick_ob: f64) -> na::Vector3<f64> {
    let m_x = dipole_m * mtx[0];
    let m_y = dipole_m * mtx[1];
    let m_z = dipole_m * mtx[2];
    let rtx_x = rtx[0];
    let rtx_y = rtx[1];
    let rtx_z = rtx[2];
    let rrx_x = rrx[0];
    let rrx_y = rrx[1];
    let rrx_z = rrx[2];

    // Calculate x component
    let h_obx = if rrx_z > 0.0 {
        (-1.0 / (4.0 * PI)) * (
            m_x / ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
                   (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(1.5) - 
            (3.0 * (2.0 * rrx_x - 2.0 * rtx_x) * (
                m_x * (rrx_x - rtx_x) - m_z * (rrx_z + rtx_z + (2.0 * O) / 
                (mu * sigma_ob * thick_ob)) + m_y * (rrx_y - rtx_y))) / 
            (2.0 * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
                    (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5))
        )
    } else {
        (-1.0 / (4.0 * PI)) * (
            m_x / ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
                   (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(1.5) - 
            (3.0 * (2.0 * rrx_x - 2.0 * rtx_x) * (
                m_x * (rrx_x - rtx_x) + m_y * (rrx_y - rtx_y) - m_z * 
                (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)))) / 
            (2.0 * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
                    (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5))
        )
    };

    // Calculate z component
    let h_obz = if rrx_z > 0.0 {
        (-1.0 / (4.0 * PI)) * (
            -m_z / ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
                    (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(1.5) - 
            (3.0 * (2.0 * rrx_z + 2.0 * rtx_z + (4.0 * O) / (mu * sigma_ob * thick_ob)) * 
             (m_x * (rrx_x - rtx_x) - m_z * (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)) + 
              m_y * (rrx_y - rtx_y))) / 
            (2.0 * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
                    (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5))
        )
    } else {
        (-1.0 / (4.0 * PI)) * (
            m_z / ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
                   (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(1.5) + 
            (3.0 * (2.0 * rtx_z - 2.0 * rrx_z + (4.0 * O) / (mu * sigma_ob * thick_ob)) * 
             (m_x * (rrx_x - rtx_x) + m_y * (rrx_y - rtx_y) - m_z * 
              (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)))) / 
            (2.0 * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
                    (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5))
        )
    };

    na::Vector3::new(h_obx, 0.0, h_obz)
}

pub fn dh_obdt_xyz(mtx: &na::Vector3<f64>, dipole_m: f64, rtx: &na::Vector3<f64>, 
                   rrx: &na::Vector3<f64>, O: f64, mu: f64, sigma_ob: f64, thick_ob: f64) -> na::Vector3<f64> {
    let m_x = dipole_m * mtx[0];
    let m_y = dipole_m * mtx[1];
    let m_z = dipole_m * mtx[2];
    let rtx_x = rtx[0];
    let rtx_y = rtx[1];
    let rtx_z = rtx[2];
    let rrx_x = rrx[0];
    let rrx_y = rrx[1];
    let rrx_z = rrx[2];

    let dh_obx = if rrx_z > 0.0 {
        (-1.0 / (4.0 * PI)) * (
            (m_z * (6.0 * rrx_x - 6.0 * rtx_x)) / (mu * sigma_ob * thick_ob * 
            ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
             (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5)) - 
            (6.0 * m_x * (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob))) / 
            (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
             (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5)) + 
            (5.0 * (6.0 * rrx_x - 6.0 * rtx_x) * (rrx_z + rtx_z + (2.0 * O) / 
             (mu * sigma_ob * thick_ob)) * (m_x * (rrx_x - rtx_x) - m_z * 
             (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)) + m_y * (rrx_y - rtx_y))) / 
            (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
             (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(3.5))
        )
    } else {
        (-1.0 / (4.0 * PI)) * (
            ((m_z * (6.0 * rrx_x - 6.0 * rtx_x)) / (mu * sigma_ob * thick_ob * 
             ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
              (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5)) - 
             (6.0 * m_x * (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob))) / 
             (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
              (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5))) + 
             (5.0 * (6.0 * rrx_x - 6.0 * rtx_x) * (rtx_z - rrx_z + (2.0 * O) / 
              (mu * sigma_ob * thick_ob)) * (m_x * (rrx_x - rtx_x) + m_y * (rrx_y - rtx_y) - 
               m_z * (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)))) / 
             (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
              (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(3.5))
        )
    };

    let dh_obz = if rrx_z > 0.0 {
        (-1.0 / (4.0 * PI)) * (
            (6.0 * m_z * (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob))) / 
            (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
             (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5)) - 
            (6.0 * (m_x * (rrx_x - rtx_x) - m_z * (rrx_z + rtx_z + (2.0 * O) / 
             (mu * sigma_ob * thick_ob)) + m_y * (rrx_y - rtx_y))) / (mu * sigma_ob * thick_ob * 
             ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + (rrx_z + rtx_z + (2.0 * O) / 
              (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5)) + 
            (m_z * (6.0 * rrx_z + 6.0 * rtx_z + (12.0 * O) / (mu * sigma_ob * thick_ob))) / 
            (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
             (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5)) + 
            (5.0 * (6.0 * rrx_z + 6.0 * rtx_z + (12.0 * O) / (mu * sigma_ob * thick_ob)) * 
             (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)) * 
             (m_x * (rrx_x - rtx_x) - m_z * (rrx_z + rtx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)) + 
              m_y * (rrx_y - rtx_y))) / (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + 
              (rrx_y - rtx_y).powi(2) + (rrx_z + rtx_z + (2.0 * O) / 
               (mu * sigma_ob * thick_ob)).powi(2)).powf(3.5))
        )
    } else {
        (-1.0 / (4.0 * PI)) * (
            (6.0 * (m_x * (rrx_x - rtx_x) + m_y * (rrx_y - rtx_y) - m_z * 
             (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)))) / 
            (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
             (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5)) - 
            (m_z * (6.0 * rtx_z - 6.0 * rrx_z + (12.0 * O) / (mu * sigma_ob * thick_ob))) / 
            (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
             (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5)) - 
            (6.0 * m_z * (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob))) / 
            (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
             (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(2.5)) - 
            (5.0 * (6.0 * rtx_z - 6.0 * rrx_z + (12.0 * O) / (mu * sigma_ob * thick_ob)) * 
             (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)) * 
             (m_x * (rrx_x - rtx_x) + m_y * (rrx_y - rtx_y) - m_z * 
              (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)))) / 
            (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x).powi(2) + (rrx_y - rtx_y).powi(2) + 
             (rtx_z - rrx_z + (2.0 * O) / (mu * sigma_ob * thick_ob)).powi(2)).powf(3.5))
        )
    };

    na::Vector3::new(dh_obx, 0.0, dh_obz)
}

pub fn thetafunction_step(t: f64, O: f64, o: f64, mu: f64, sigma_sp: f64, a: f64, T: f64) -> f64 {
    let ss = mu * sigma_sp * a * a;
    let mut theta = 0.0;
    let mut solver = 0.0;
    let mut k = 0;

    while solver < 1.0 {
        k += 1;
        let k_pi = (k as f64) * PI;
        
        let temp = (1.0 / (1.0 + (-T/2.0 * k_pi.powi(2) / ss).exp())) * 
                  (6.0 / k_pi.powi(2)) * ((o + O - t) * k_pi.powi(2) / ss).exp();
        
        theta += temp;
        
        solver = temp / theta;
        
        if k > 1 && temp.abs() < theta.abs() * 1e-10 {
            break;
        }
        
        if k > 100 {
            break;
        }
    }
    
    theta
}

pub fn dh_tot_step(mtx: &na::Vector3<f64>, dipole_m: f64, rtx: &na::Vector3<f64>, 
                   rsp: &na::Vector3<f64>, mu: f64, sigma_ob: f64, thick_ob: f64, t: f64, 
                   o: f64, sigma_sp: f64, a: f64, T: f64) -> na::Vector3<f64> {
    let ob_array = h_ob_xyz(mtx, dipole_m, rtx, rsp, -o, mu, sigma_ob, thick_ob);
    let thetaz = thetafunction_step(t, 0.0, o, mu, sigma_sp, a, T);

    // Set up numerical integration
    let n = 100;
    let dt = (t - o) / n as f64;
    let mut resultx = 0.0;
    let mut resultz = 0.0;

    for i in 0..=n {
        let tau = o + (i as f64) * dt;
        let weight = if i == 0 || i == n { 0.5 } else { 1.0 };
        
        let dh_ob = dh_obdt_xyz(mtx, dipole_m, rtx, rsp, tau, mu, sigma_ob, thick_ob);
        let theta = thetafunction_step(t, tau, o, mu, sigma_sp, a, T);
        
        resultx -= weight * dh_ob.x * theta;
        resultz -= weight * dh_ob.z * theta;
    }

    resultx *= dt;
    resultz *= dt;

    na::Vector3::new(
        resultx + (ob_array.x * thetaz),
        0.0,
        resultz + (ob_array.z * thetaz)
    )
}

pub fn h_total_step_1storder(mtx: &na::Vector3<f64>, dipole_m: f64, rtx: &na::Vector3<f64>, 
                            offset_tx_rx: &na::Vector3<f64>, rsp: &na::Vector3<f64>, t: f64, 
                            mu: f64, sigma_ob: f64, thick_ob: f64, sigma_sp: f64, a: f64, 
                            P: f64, apply_dip: bool, dip: f64, strike: f64, wave: f64, 
                            T: f64, xsign: bool) -> na::Vector3<f64> {
    let moment = 2.0 * PI * a.powi(3) * 
                dh_tot_step(mtx, dipole_m, rtx, rsp, mu, sigma_ob, thick_ob, t, 0.0, sigma_sp, a, T);

    let msp = if apply_dip {
        let dip_rad = (90.0 - dip) * PI / 180.0;
        let strike_rad = (strike - 90.0) * PI / 180.0;
        let norm = na::Vector3::new(
            dip_rad.cos() * strike_rad.cos(),
            strike_rad.sin() * dip_rad.cos(),
            dip_rad.sin()
        ).normalize();
        
        let mspdotnorm = moment.dot(&norm);
        norm * mspdotnorm
    } else {
        moment
    };

    let offset = na::Vector3::new(
        -offset_tx_rx[0],
        -offset_tx_rx[1],
        rtx[2] - offset_tx_rx[2]
    ) - na::Vector3::new(-rtx[0], -rtx[1], rsp[2]);

    let statics = static_field(&msp, &offset);

    let h_tot = if xsign {
        na::Vector3::new(
            -(statics.x),
            statics.y,
            statics.z
        )
    } else {
        statics
    };

    // Calculate overburden field
    let h_ob = h_ob_xyz(mtx, dipole_m, 
        &na::Vector3::new(0.0, 0.0, rtx[2]),
        &na::Vector3::new(-offset_tx_rx[0], -offset_tx_rx[1], rtx[2] - offset_tx_rx[2]),
        t, mu, sigma_ob, thick_ob);

    // Changed sign convention for z component
    if xsign {
        na::Vector3::new(
            -(h_tot.x + h_ob.x),
            h_tot.y + h_ob.y,
           (h_tot.z - h_ob.z)  // Removed negation here
        )
    } else {
        na::Vector3::new(
            h_tot.x + h_ob.x,
            h_tot.y + h_ob.y,
            (h_tot.z - h_ob.z)  // Removed negation here
        )
    }
}

pub fn calculate_response(x: f64, wc: &[f64], params: &Parameters) -> Vec<FieldComponents> {
    let mut results = Vec::with_capacity(wc.len());
    
    let rtx = na::Vector3::new(x, 0.0, params.radar);
    let wave = 1.0;  // Default wave value
    let P = params.pulse_length;
    
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
            params.xsign_negative
        );
        
        results.push(FieldComponents {
            x: (params.mu/1e-12) * response.x,
            y: (params.mu/1e-12) * response.y,
            z: (params.mu/1e-12) * response.z,
        });
    }
    
    results
}