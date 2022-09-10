use crate::geometry::shape::Intersection;
use crate::geometry::shape::Shape;
use crate::geometry::types::*;
use nalgebra::distance_squared;
use std::f64::EPSILON;

struct Circle {
    center: Point2,
    r: f64,
}

impl Shape for Circle {
    fn intersect(&self, p: Point2, d: UVec2) -> Option<Intersection> {
        let vcp = p - self.center;
        let b = 2.0 * d.dot(&vcp);
        let c = vcp.norm_squared() - self.r * self.r;
        let delta = b * b - 4.0 * c;
        if delta < 0.0 {
            None
        } else {
            let t1 = (-b - delta.sqrt()) / 2.0;
            let t2 = (-b + delta.sqrt()) / 2.0;
            let t = f64::max(t1, t2);
            if t > EPSILON {
                let ip = p + t * *d; // intersection point
                let n = UVec2::new_normalize(ip - self.center);
                Some(Intersection {
                    point: ip,
                    normal: n,
                })
            } else {
                None
            }
        }
    }

    fn is_inside(&self, p: Point2) -> bool {
        distance_squared(&p, &self.center) < self.r * self.r
    }
}
