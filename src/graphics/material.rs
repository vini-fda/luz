use std::rc::Rc;

use nalgebra::vector;
use rand::{rngs::ThreadRng, Rng};

use crate::geometry::types::*;

use super::color::Color;

fn fresnel(cosi: f64, cost: f64, etai: f64, etat: f64) -> f64 {
    let rs = (etat * cosi - etai * cost) / (etat * cosi + etai * cost);
    let rp = (etat * cost - etai * cosi) / (etat * cost + etai * cosi);
    (rs * rs + rp * rp) * 0.5
}

struct Material {
    pub absorptivity: Color,
}

impl Material {
    /// This function is the BRDF of the material:
    /// for an incoming light direction `wi`, and
    /// an outgoing direction `wr`,
    /// it returns the ratio of reflected radiance exiting along `wr`
    /// to the irradiance incident on the surface from direction `wi`
    //fn brdf(&self, wi: UVec2, wr: UVec2) -> f64;
    fn sample_dir(&self, wi: UVec2, rng: &mut ThreadRng) -> Option<UVec2>;
    /// Samples the emission of light at a sampling angle `ws`
    /// this function assumes the a normalized coordinate space,
    /// i.e. the normal, tangent vectors are `n = [0; 1]`
    /// and `t = [1; 0]`, respectively
    fn emission(&self, ws: Vec2) -> Color;
}

// struct ReflectiveMaterial {
//     pub reflectance: f64,
// }

// impl Material for ReflectiveMaterial {
//     fn sample_dir(&self, wi: UVec2, rng: &mut ThreadRng) -> Option<UVec2> {
//         Some(UVec2::new_unchecked(vector![wi.x, -wi.y]))
//     }
//     fn emission(&self, ws: Vec2) -> Color {
//         Color::black()
//     }
// }

// struct AbsorptiveMaterial {
//     pub absorptivity: Color,
// }

// impl Material for AbsorptiveMaterial {
//     fn sample_dir(&self, wi: UVec2, rng: &mut ThreadRng) -> Option<(UVec2, f64)> {
//         None
//     }
//     fn emission(&self, ws: Vec2) -> Color {
//         Color::black()
//     }
// }

// struct EmissiveMaterial {
//     pub emissive: Color,
// }

// impl Material for EmissiveMaterial {
//     fn sample_dir(&self, wi: UVec2, rng: &mut ThreadRng) -> Option<UVec2> {
//         Some(UVec2::new_unchecked(vector![wi.x, -wi.y]))
//     }
//     fn emission(&self, ws: Vec2) -> Color {
//         self.emissive
//     }
// }

// struct DielectricMaterial {
//     eta: f64, // relative eta: ratio between eta and eta0 (free space)
// }

// impl Material for DielectricMaterial {
//     fn sample_dir(&self, wi: UVec2, rng: &mut ThreadRng) -> Option<UVec2> {
//         // relative eta: eta_r = eta1/eta2
//         let eta_r = if wi.y < 0.0 { 1.0 / self.eta } else { self.eta };

//         unimplemented!()
//     }
//     fn emission(&self, ws: Vec2) -> Color {
//         Color::black()
//     }
// }

// struct DiffuseMaterial {
//     rng: Rc<ThreadRng>,
// }

// impl Material for DiffuseMaterial {
//     fn sample_dir(&self, wi: UVec2, rng: &mut ThreadRng) -> Option<UVec2> {
//         let xi = self.rng.gen_range(0.0..=1.0);
//         let sin_theta = 2.0 * xi - 1.0;
//         let cos_theta = (1.0f64 - sin_theta * sin_theta).sqrt();
//         Some(UVec2::new_unchecked(vector![cos_theta, sin_theta]))
//     }
//     fn emission(&self, ws: Vec2) -> Color {
//         Color::black()
//     }
// }

// pub(crate) trait Material {
//     /// This function is the BRDF of the material:
//     /// for an incoming light direction `wi`, and
//     /// an outgoing direction `wr`,
//     /// it returns the ratio of reflected radiance exiting along `wr`
//     /// to the irradiance incident on the surface from direction `wi`
//     //fn brdf(&self, wi: UVec2, wr: UVec2) -> f64;
//     fn sample_dir(&self, wi: UVec2, rng: &mut ThreadRng) -> Option<UVec2>;
//     /// Samples the emission of light at a sampling angle `ws`
//     /// this function assumes the a normalized coordinate space,
//     /// i.e. the normal, tangent vectors are `n = [0; 1]`
//     /// and `t = [1; 0]`, respectively
//     fn emission(&self, ws: Vec2) -> Color;
// }
