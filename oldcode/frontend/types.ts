export interface Vector3 {
  x: number;
  y: number;
  z: number;
}

export interface CalculationParams {
  radar: number;
  mu: number;
  dipole_m: number;
  rtxrx: Vector3;
  rsp: Vector3;
  a: number;
  sigma_sp: number;
  mtx: Vector3;
  sigma_ob: number;
  thick_ob: number;
  apply_dip: boolean;
  strike: number;
  dip: number;
  base_freq: number;
  period: number;
  pulse_length: number;
  xsign_negative: boolean;
  profile_length: number;
}

export interface ResponseData {
  x_values: number[];
  time_windows: number[];
  x_components: number[][];
  y_components: number[][];
  z_components: number[][];
} 