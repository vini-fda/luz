mod geometry;
mod graphics;
extern crate nalgebra as na;

use crate::geometry::circle::Circle;
use crate::geometry::polygon::Polygon;
use crate::geometry::types::*;
use crate::graphics::color::Color;
use crate::graphics::scene::Scene;

use graphics::entity::Entity;
use image::{ImageBuffer, Rgb};
use na::distance;
use na::vector;
use na::Rotation2;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use std::cmp::min;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

const W: u32 = 256;
const H: u32 = 256;
const N: u32 = 128;
const MAX_DEPTH: u32 = 6;

fn reflect(vi: Vec2, n: UVec2) -> Vec2 {
    // https://math.stackexchange.com/questions/13261/how-to-get-a-reflection-vector
    // This function reflects the i vector over the vector n
    // The vector n must be normalized
    vi - 2.0 * n.dot(&vi) * *n
}

fn refract(vi: UVec2, n: UVec2, eta: f64) -> Option<UVec2> {
    // Vector refraction
    let dot = n.dot(&vi);
    let k = 1.0 - eta * eta * (1.0 - dot * dot);
    if k < 0.0 {
        return None; // all reflection
    }
    let a = eta * dot + k.sqrt();
    let t = eta * *vi - a * *n;
    Some(UVec2::new_normalize(t))
}

fn fresnel(cosi: f64, cost: f64, etai: f64, etat: f64) -> f64 {
    let rs = (etat * cosi - etai * cost) / (etat * cosi + etai * cost);
    let rp = (etat * cost - etai * cosi) / (etat * cost + etai * cosi);
    (rs * rs + rp * rp) * 0.5
}

/// Models Schlik's approximation for the fresnel equation
fn schlick(cosi: f64, cost: f64, etai: f64, etat: f64) -> f64 {
    let r0 = (etai - etat) / (etai + etat);
    let r0 = r0 * r0;
    let a = if etai < etat { 1.0 - cosi } else { 1.0 - cost };
    let aa = a * a;
    r0 + (1.0 - r0) * aa * aa * a
}

fn beer_lambert(a: Color, d: f64) -> Color {
    Color {
        r: (-a.r * d).exp(),
        g: (-a.g * d).exp(),
        b: (-a.b * d).exp(),
    }
}

fn trace(scene: &Scene, center: Point2, d: UVec2, depth: u32) -> Color {
    if let Some((entity, inter)) = scene.intersect_closest(center, d) {
        let n = inter.normal;
        let dot = d.dot(&n);
        let from_outside = dot < 0.0; // true iff the ray comes into the interface
        let mut sum = Color::black();

        //
        let t = vector![n.y, -n.x];
        // `ws`: `d` decomposed in the normal coordinate space
        // of the material surface
        let ws = vector![t.dot(&d), n.dot(&d)];
        sum = sum + entity.material.emission(ws);
        unimplemented!()
    } else {
        Color::black();
    }
    unimplemented!()
}

// fn trace_old(scene: &Scene, center: Point2, d: UVec2, depth: u32) -> Color {
//     if let Some(r) = scene.intersect(center, d) {
//         let sign = if r.normal.dot(&d) < 0.0 { 1.0 } else { -1.0 };
//         let mut sum = r.emissive;
//         if depth < MAX_DEPTH && (r.reflectivity > 0.0 || r.eta > 0.0) {
//             let mut refl = r.reflectivity;
//             let n = UVec2::new_unchecked(sign * *r.normal);
//             if r.eta > 0.0 {
//                 let eta = if sign < 0.0 { r.eta } else { 1.0 / r.eta };
//                 match refract(d, n, eta) {
//                     Some(refracted) => {
//                         let cosi = -n.dot(&d);
//                         let cost = -n.dot(&refracted);
//                         refl = if sign < 0.0 {
//                             schlick(cosi, cost, r.eta, 1.0)
//                         } else {
//                             schlick(cosi, cost, 1.0, r.eta)
//                         };
//                         // TODO: investigate whether to move the center
//                         sum = sum + trace(scene, r.point, refracted, depth + 1) * (1.0 - refl)
//                     }
//                     None => refl = 1.0,
//                 }
//             }
//             if refl > 0.0 {
//                 let new_dir = UVec2::new_normalize(reflect(*d, n));
//                 sum = sum + trace(scene, r.point, new_dir, depth + 1) * refl;
//             }
//         }
//         if sign < 0.0 {
//             sum = sum * beer_lambert(r.absorption, distance(&center, &r.point));
//         }
//         sum
//     } else {
//         Color::black()
//     }
// }

fn sample(scene: &Scene, rng: &mut ThreadRng, point: Point2) -> Color {
    let v: Vec<f64> = (0..N)
        .map(|i| 2.0 * PI * (i as f64 + rng.gen_range(0.0..=1.0)) / N as f64)
        .collect();
    let sum: Color = v
        .iter()
        .map(|a| {
            trace(
                scene,
                point,
                UVec2::new_unchecked(Vec2::new(a.cos(), a.sin())),
                0,
            )
        })
        .sum();
    sum * (1.0 / N as f64)
}

fn sample_par(scene: &Scene, point: Point2) -> Color {
    let sum: Color = (0..N)
        .into_par_iter()
        .map_init(thread_rng, |rng, i| {
            let a = 2.0 * PI * (i as f64 + rng.gen_range(0.0..=1.0)) / N as f64;
            trace(
                scene,
                point,
                UVec2::new_unchecked(Vec2::new(a.cos(), a.sin())),
                0,
            )
        })
        .sum();
    sum * (1.0 / N as f64)
}

fn main() {
    let mut img = ImageBuffer::from_pixel(W, H, Rgb([0u8, 0u8, 0u8]));
    let scene = Scene {
        entities: vec![
            Entity {
                shape: Box::new(Circle {
                    center: Point2::new(0.5, -0.2),
                    r: 0.1,
                }),
                emissive: Color {
                    r: 10.0,
                    g: 10.0,
                    b: 10.0,
                },
                reflectivity: 0.0,
                eta: 0.0,
                absorption: Color::black(),
            },
            Entity {
                shape: Box::new(Circle {
                    center: Point2::new(0.1, 0.2),
                    r: 0.1,
                }),
                emissive: Color::black(),
                reflectivity: 0.0,
                eta: 0.5,
                absorption: Color {
                    r: 1.0,
                    g: 4.0,
                    b: 1.0,
                },
            },
            Entity {
                shape: Box::new(Polygon::ngon(Point2::new(0.75, 0.5), 0.25, 5)),
                emissive: Color::black(),
                reflectivity: 0.1,
                eta: 0.8,
                absorption: Color {
                    r: 1.0,
                    g: 3.6,
                    b: 4.0,
                },
            },
            // Entity {
            //     shape: Box::new(Polygon::rectangle(0.6, 0.1, 0.0, 0.5, 0.01)),
            //     emissive: Color::black(),
            //     reflectivity: 0.0,
            //     eta: 1.0,
            //     absorption: Color {
            //         r: 1.0,
            //         g: 4.0,
            //         b: 1.0,
            //     },
            // },
        ],
    };
    // let mut rng = thread_rng();
    // for x in 0..W {
    //     for y in 0..H {
    //         let xx = x as f64 / W as f64;
    //         let yy = y as f64 / H as f64;
    //         let color = sample(&scene, &mut rng, xx, yy);
    //         let r = min((color.r * 255.0) as u32, 255) as u8;
    //         let g = min((color.g * 255.0) as u32, 255) as u8;
    //         let b = min((color.b * 255.0) as u32, 255) as u8;
    //         img.put_pixel(x, y, Rgb([r, g, b]));
    //     }
    // }
    // img.save("out.png").unwrap();
    let img_mutex = Arc::new(Mutex::new(img));

    (0..W).into_par_iter().for_each_init(thread_rng, |rng, x| {
        for y in 0..H {
            let xx = x as f64 / W as f64;
            let yy = y as f64 / H as f64;
            let p = Point2::new(xx, yy);
            let color = sample(&scene, rng, p);
            let r = min((color.r * 255.0) as u32, 255) as u8;
            let g = min((color.g * 255.0) as u32, 255) as u8;
            let b = min((color.b * 255.0) as u32, 255) as u8;
            img_mutex.lock().unwrap().put_pixel(x, y, Rgb([r, g, b]));
        }
    });
    //img_mutex.lock().unwrap().save("out.png").unwrap();
}
