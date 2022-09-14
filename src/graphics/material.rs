use rand::{rngs::ThreadRng, Rng};

use super::brdf;
use crate::geometry::types::*;

use super::color::Color;

fn fresnel(cosi: f64, cost: f64, etai: f64, etat: f64) -> f64 {
    let rs = (etat * cosi - etai * cost) / (etat * cosi + etai * cost);
    let rp = (etat * cost - etai * cosi) / (etat * cost + etai * cosi);
    (rs * rs + rp * rp) * 0.5
}

pub(crate) enum MaterialType {
    Lambert, // i.e. diffuse material
    Dielectric,
    Mirror,
    Emissive,
    DirectionalEmissive,
}

pub(crate) enum SampleResult {
    Edge(UVec2, f64),
    Node(Color),
}

/// Absorbance + Reflectance + Transmittance = 1.0
pub(crate) struct Material {
    pub mtype: MaterialType,
    pub absorptivity: Color, // Lambert-beer model
    pub absorbance: f64,     // range = [0, 100%]
    pub reflectance: f64,    // range = [0, 100%]
    pub eta: f64,            // range = [1.0, infty)
    pub emmitivity: Color,
}

impl Material {
    /// This function is the BRDF of the material:
    /// for an incoming light direction `wi`, and
    /// an outgoing direction `wr`,
    /// it returns the ratio of reflected radiance exiting along `wr`
    /// to the irradiance incident on the surface from direction `wi`
    //fn brdf(&self, wi: UVec2, wr: UVec2) -> f64;
    pub fn sample(&self, wi: UVec2, n: UVec2, rng: &mut ThreadRng) -> SampleResult {
        match self.mtype {
            MaterialType::Lambert => {
                let cos = wi.dot(&n);
                if cos < 0.0 {
                    let wo = brdf::sample_diffuse(wi, n, rng);
                    SampleResult::Edge(wo, 1.0)
                } else {
                    SampleResult::Node(Color::black())
                }
            }
            MaterialType::Dielectric => {
                let wo = brdf::sample_dielectric(wi, n, self.eta, rng);
                SampleResult::Edge(wo, 1.0)
            }
            MaterialType::Mirror => {
                let wo = brdf::sample_mirror(wi, n);
                SampleResult::Edge(wo, 1.0)
            }
            MaterialType::Emissive => SampleResult::Node(self.emmitivity),
            MaterialType::DirectionalEmissive => {
                let cos = wi.dot(&n);
                if cos < -0.9999 {
                    SampleResult::Node(self.emmitivity)
                } else {
                    SampleResult::Node(Color::black())
                }
            }
        }
    }
}

/// Models Schlik's approximation for the fresnel equation
fn schlick(cosi: f64, cost: f64, etai: f64, etat: f64) -> f64 {
    let r0 = (etai - etat) / (etai + etat);
    let r0 = r0 * r0;
    let a = if etai < etat { 1.0 - cosi } else { 1.0 - cost };
    let aa = a * a;
    r0 + (1.0 - r0) * aa * aa * a
}
/// Uses sellmier equation
fn eta(lambda: f64) -> f64 {
    let b1 = 12.0 * 1.03961212;
    let b2 = 12.0 * 0.231792344;
    let b3 = 12.0 * 1.01046945;
    let c1 = 12.0 * 0.00600069867;
    let c2 = 12.0 * 0.0200179144;
    let c3 = 12.0 * 103.560653;

    (_sm(lambda, b1, c1) + _sm(lambda, b2, c2) + _sm(lambda, b3, c3)).sqrt()
}

fn _sm(lambda: f64, b: f64, c: f64) -> f64 {
    let l = lambda * lambda;
    b * l / (l - c)
}

fn color(lambda: f64) -> Color {
    let gamma = 0.80;
    let factor;
    let red;
    let green;
    let blue;

    if (lambda >= 380.0) && (lambda < 440.0) {
        red = -(lambda - 440.0) / (440.0 - 380.0);
        green = 0.0;
        blue = 1.0;
    } else if (lambda >= 440.0) && (lambda < 490.0) {
        red = 0.0;
        green = (lambda - 440.0) / (490.0 - 440.0);
        blue = 1.0;
    } else if (lambda >= 490.0) && (lambda < 510.0) {
        red = 0.0;
        green = 1.0;
        blue = -(lambda - 510.0) / (510.0 - 490.0);
    } else if (lambda >= 510.0) && (lambda < 580.0) {
        red = (lambda - 510.0) / (580.0 - 510.0);
        green = 1.0;
        blue = 0.0;
    } else if ((lambda >= 580.0) && (lambda < 645.0)) {
        red = 1.0;
        green = -(lambda - 645.0) / (645.0 - 580.0);
        blue = 0.0;
    } else if ((lambda >= 645.0) && (lambda < 781.0)) {
        red = 1.0;
        green = 0.0;
        blue = 0.0;
    } else {
        red = 0.0;
        green = 0.0;
        blue = 0.0;
    };

    // Let the intensity fall off near the vision limits

    if (380.0..420.0).contains(&lambda) {
        factor = 0.3 + 0.7 * (lambda - 380.0) / (420.0 - 380.0);
    } else if (420.0..701.0).contains(&lambda) {
        factor = 1.0;
    } else if (701.0..781.0).contains(&lambda) {
        factor = 0.3 + 0.7 * (780.0 - lambda) / (780.0 - 700.0);
    } else {
        factor = 0.0;
    };

    Color {
        r: f64::powf(red * factor, gamma),
        g: f64::powf(green * factor, gamma),
        b: f64::powf(blue * factor, gamma),
    }
}
