mod geometry;
mod graphics;
extern crate nalgebra as na;

use crate::geometry::circle::Circle;
use crate::geometry::polygon::Polygon;
use crate::geometry::types::*;
use crate::graphics::color::Color;
use crate::graphics::material;
use crate::graphics::scene::Scene;

use graphics::entity::Entity;
use graphics::material::Material;
use graphics::material::MaterialType;
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
const N: u32 = 256;
const MAX_DEPTH: u32 = 6;

fn beer_lambert(a: Color, d: f64) -> Color {
    Color {
        r: (-a.r * d).exp(),
        g: (-a.g * d).exp(),
        b: (-a.b * d).exp(),
    }
}

fn trace(scene: &Scene, center: Point2, d: UVec2, rng: &mut ThreadRng, depth: u32) -> Color {
    if let Some((entity, inter)) = scene.intersect_closest(center, d) {
        let n = inter.normal;
        let dot = d.dot(&n);
        let from_outside = dot < 0.0; // true iff the ray comes into the interface
        let mut sum = Color::black();
        if depth < MAX_DEPTH {
            match entity.material.sample(d, n, rng) {
                material::SampleResult::Edge(wo, abs) => {
                    sum = sum + trace(scene, inter.point, wo, rng, depth + 1) * abs;
                }
                material::SampleResult::Node(emission) => {
                    sum = emission;
                }
            }
            if let MaterialType::Dielectric = entity.material.mtype {
                let dist = distance(&center, &inter.point);
                sum = sum * beer_lambert(entity.material.absorptivity, dist);
            }
        }
        sum
    } else {
        Color::black()
    }
}

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
                rng,
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
                rng,
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
                material: Material {
                    mtype: MaterialType::Emissive,
                    emmitivity: Color {
                        r: 5.0,
                        g: 5.0,
                        b: 5.0,
                    },
                    absorptivity: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                    absorbance: 0.0,
                    reflectance: 0.0,
                    eta: 0.0,
                },
            },
            Entity {
                shape: Box::new(Circle {
                    center: Point2::new(0.1, 0.2),
                    r: 0.1,
                }),
                material: Material {
                    mtype: MaterialType::Lambert,
                    emmitivity: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                    absorptivity: Color {
                        r: 1.0,
                        g: 4.0,
                        b: 1.0,
                    },
                    absorbance: 0.0,
                    reflectance: 0.0,
                    eta: 0.0,
                },
            },
            Entity {
                shape: Box::new(Polygon::ngon(Point2::new(0.75, 0.5), 0.25, 5)),
                material: Material {
                    mtype: MaterialType::Dielectric,
                    emmitivity: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                    absorptivity: Color {
                        r: 5.0,
                        g: 10.0,
                        b: 3.33,
                    },
                    absorbance: 0.0,
                    reflectance: 0.0,
                    eta: 1.8,
                },
            },
            Entity {
                shape: Box::new(Polygon::rectangle(Point2::new(0.1, 0.5), -0.2, 0.1, 0.01)),
                material: Material {
                    mtype: MaterialType::Emissive,
                    emmitivity: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                    absorptivity: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                    absorbance: 0.0,
                    reflectance: 0.0,
                    eta: 1.8,
                },
            },
            Entity {
                shape: Box::new(Polygon::rectangle(Point2::new(0.3, 0.35), 1.7, 0.1, 0.01)),
                material: Material {
                    mtype: MaterialType::Lambert,
                    emmitivity: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                    absorptivity: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                    absorbance: 0.0,
                    reflectance: 0.0,
                    eta: 1.8,
                },
            },
            Entity {
                shape: Box::new(Polygon::rectangle(Point2::new(0.1, 0.65), 0.0, 0.1, 0.01)),
                material: Material {
                    mtype: MaterialType::Mirror,
                    emmitivity: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                    absorptivity: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    },
                    absorbance: 0.0,
                    reflectance: 0.0,
                    eta: 1.8,
                },
            },
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
    img_mutex.lock().unwrap().save("out.png").unwrap();
}
