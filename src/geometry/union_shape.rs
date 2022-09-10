use crate::geometry::shape::Intersection;
use crate::geometry::shape::Shape;
use crate::geometry::types::*;
use nalgebra::distance_squared;

struct UnionShape {
    a: Box<dyn Shape + Sync>,
    b: Box<dyn Shape + Sync>,
}

impl UnionShape {
    fn new(a: Box<dyn Shape + Sync>, b: Box<dyn Shape + Sync>) -> UnionShape {
        UnionShape { a, b }
    }
}

impl Shape for UnionShape {
    fn intersect(&self, p: Point2, d: UVec2) -> Option<Intersection> {
        match (self.a.intersect(p, d), self.b.intersect(p, d)) {
            (Some(i1), Some(i2)) => {
                let d1 = distance_squared(&p, &i1.point);
                let d2 = distance_squared(&p, &i2.point);
                if d1 < d2 {
                    Some(i1)
                } else {
                    Some(i2)
                }
            }
            (None, r2) => r2,
            (r1, None) => r1,
        }
    }

    fn is_inside(&self, p: Point2) -> bool {
        self.a.is_inside(p) || self.b.is_inside(p)
    }
}
