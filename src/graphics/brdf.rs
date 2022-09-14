/// Useful BRDFs
use rand::{rngs::ThreadRng, Rng};

use crate::geometry::types::*;
use na::vector;

fn dielectric_reflectance(wi: UVec2, eta: f64) -> f64 {
    let cosi = -wi.y;
    let cost = 1.0 - eta * eta * (1.0 - cosi * cosi);
    let rs = (eta * cosi - cost) / (eta * cosi + cost);
    let rp = (eta * cost - cosi) / (eta * cost + cosi);
    (rs * rs + rp * rp) * 0.5
}

/// Sampling routine for a perfectly diffuse surface in 2D
pub fn sample_diffuse(wi: UVec2, n: UVec2, rng: &mut ThreadRng) -> UVec2 {
    let (wi, t) = transform_coords(wi, n);
    let wo = _sample_diffuse(wi, rng);
    inv_transform_coords(wo, n, t)
}

/// Sampling routine for a perfectly diffuse surface in 2D.
///
/// This function assumes normalized coordinates.
fn _sample_diffuse(wi: UVec2, rng: &mut ThreadRng) -> UVec2 {
    let xi = rng.gen_range(0.0..=1.0);
    let sin_theta = 2.0 * xi - 1.0;
    let cos_theta = (1.0f64 - sin_theta * sin_theta).sqrt();
    UVec2::new_unchecked(vector![cos_theta, sin_theta])
}

pub fn sample_mirror(wi: UVec2, n: UVec2) -> UVec2 {
    let t = vector![n.y, -n.x];
    UVec2::new_unchecked(t.dot(&wi) * t - n.dot(&wi) * *n)
}

fn _sample_mirror(wi: UVec2) -> UVec2 {
    // https://math.stackexchange.com/questions/13261/how-to-get-a-reflection-vector
    UVec2::new_unchecked(vector!(wi.x, -wi.y))
}

pub fn sample_dielectric(wi: UVec2, n: UVec2, eta: f64, rng: &mut ThreadRng) -> UVec2 {
    let (wi, t) = transform_coords(wi, n);
    let wo = _sample_dielectric(wi, eta, rng);
    inv_transform_coords(wo, n, t)
}

/// This function assumes normalized coordinates
/// where the normal and tangent vectors are `n = [0; 1]` and `t = [1; 0]`, respectively
fn _sample_dielectric(wi: UVec2, eta: f64, rng: &mut ThreadRng) -> UVec2 {
    // refraction index: eta_r = n1/n2
    let mut eta_r = eta;
    let mut w = *wi;
    let mut n = vector![0.0, 1.0];
    let sign = if w.y > 0.0 { 1.0 } else { -1.0 };
    // for convenience, we assume the ray is coming towards the surface
    // therefore, wi.y needs to be negative
    if w.y > 0.0 {
        eta_r = 1.0 / eta;
        w.y = -w.y;
        n = -n;
    }
    let w = UVec2::new_unchecked(w);
    let n = UVec2::new_unchecked(n);

    let r = dielectric_reflectance(w, eta_r);
    let rand_num = rng.gen_range(0.0..=1.0);
    if rand_num < r {
        _sample_mirror(w)
    } else {
        refract(w, n, eta_r)
    }
}

fn refract(vi: UVec2, n: UVec2, eta: f64) -> UVec2 {
    // Vector refraction
    let dot = n.dot(&vi);
    let k = 1.0 - eta * eta * (1.0 - dot * dot);
    let a = eta * dot + k.sqrt();
    let t = eta * *vi - a * *n;
    UVec2::new_normalize(t)
}

/// Returns the vector `wi` in the basis given by the vector `n`
/// and its corresponding tangent vector `t`.
///
/// It also calculates and returns the tangent vector `t`
/// which is given by a clockwise rotation of `n` by 90 degrees.
fn transform_coords(wi: UVec2, n: UVec2) -> (UVec2, UVec2) {
    // n: normal vector
    // t: tangent vector
    let t = UVec2::new_unchecked(vector![n.y, -n.x]);
    // wi = x . t + y . n
    // x = wi . t
    // y = wi . n
    (UVec2::new_unchecked(vector![t.dot(&wi), n.dot(&wi)]), t)
}

/// Given a vector `wi` in the basis given by the normal vector `n`
/// and its corresponding tangent vector `t`, this function returns
/// `wi` in the original basis in which `t` and `n` are embedded.
fn inv_transform_coords(wi: UVec2, n: UVec2, t: UVec2) -> UVec2 {
    // n: normal vector
    // t: tangent vector
    UVec2::new_normalize(wi.x * *t + wi.y * *n)
}
