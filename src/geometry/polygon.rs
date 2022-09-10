use crate::geometry::shape::Intersection;
use crate::geometry::shape::Shape;
use crate::geometry::types::*;
use na::Rotation2;
use nalgebra::distance_squared;
use std::f64::consts::PI;
use std::f64::EPSILON;
pub(crate) struct Polygon {
    pub points: Vec<Point2>, // counterclockwise
}

fn cross(v0: Vec2, v1: Vec2) -> f64 {
    v0[0] * v1[1] - v0[1] * v1[0]
}

impl Polygon {
    pub fn new(points: Vec<Point2>) -> Self {
        if points.len() > 1 {
            Self { points }
        } else {
            panic!("Too few points!");
        }
    }

    pub fn rectangle(center: Point2, theta: f64, w: f64, h: f64) -> Self {
        Self::new(
            [
                Vec2::new(w / 2.0, -h / 2.0),
                Vec2::new(-w / 2.0, -h / 2.0),
                Vec2::new(-w / 2.0, h / 2.0),
                Vec2::new(w / 2.0, h / 2.0),
            ]
            .iter()
            .map(|&v| Rotation2::new(theta) * v)
            .map(|v| center + v)
            .collect(),
        )
    }

    pub fn ngon(center: Point2, r: f64, n: u32) -> Self {
        Self::new(
            (0..n)
                .map(|i| i as f64 * 2.0 * PI / n as f64)
                .map(|theta| r * Vec2::new(theta.cos(), -theta.sin()))
                .map(|v| center + v)
                .collect(),
        )
    }
}

impl Shape for Polygon {
    fn intersect(&self, p: Point2, d: UVec2) -> Option<Intersection> {
        let mut res: Option<Intersection> = None;
        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = if i + 1 == self.points.len() {
                self.points[0]
            } else {
                self.points[i + 1]
            };
            let va = a - p;
            let vb = b - p;
            let cross1 = cross(va, *d);
            let cross2 = cross(vb, *d);
            if cross1 * cross2 < 0.0 {
                let n = Vec2::new(va[1] - vb[1], vb[0] - va[0]);
                let n = UVec2::new_normalize(n);
                let c1 = d.dot(&n);
                if c1.abs() > EPSILON {
                    let c2 = n.dot(&(a - p));
                    let t = c2 / c1;
                    if t > EPSILON {
                        let intersect = Intersection {
                            point: p + t * (*d),
                            normal: n,
                        };
                        res = match res {
                            Some(other_intersect) => {
                                if distance_squared(&p, &intersect.point)
                                    < distance_squared(&p, &other_intersect.point)
                                {
                                    Some(intersect)
                                } else {
                                    Some(other_intersect)
                                }
                            }
                            None => Some(intersect),
                        }
                    }
                }
            }
        }
        res
    }

    fn is_inside(&self, p: Point2) -> bool {
        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = if i + 1 == self.points.len() {
                self.points[0]
            } else {
                self.points[i + 1]
            };
            let vab = b - a;
            let vap = p - a;
            if cross(vab, vap) >= 0.0 {
                return false;
            }
        }
        true
    }
}
