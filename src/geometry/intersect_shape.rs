use crate::geometry::shape::Intersection;
use crate::geometry::shape::Shape;
use crate::geometry::types::*;
use nalgebra::distance_squared;
struct IntersectShape {
    a: Box<dyn Shape + Sync>,
    b: Box<dyn Shape + Sync>,
}

impl IntersectShape {
    fn new(a: Box<dyn Shape + Sync>, b: Box<dyn Shape + Sync>) -> IntersectShape {
        IntersectShape { a, b }
    }
}

impl Shape for IntersectShape {
    fn intersect(&self, p: Point2, d: UVec2) -> Option<Intersection> {
        match (self.a.intersect(p, d), self.b.intersect(p, d)) {
            (Some(i1), Some(i2)) => {
                if self.a.is_inside(i2.point) && self.b.is_inside(i1.point) {
                    let d1 = distance_squared(&p, &i1.point);
                    let d2 = distance_squared(&p, &i2.point);
                    if d1 < d2 {
                        Some(i1)
                    } else {
                        Some(i2)
                    }
                } else if self.a.is_inside(i2.point) {
                    Some(i2)
                } else if self.b.is_inside(i1.point) {
                    Some(i1)
                } else {
                    None
                }
            }
            (None, Some(i2)) => {
                if self.a.is_inside(i2.point) {
                    Some(i2)
                } else {
                    None
                }
            }
            (Some(i1), None) => {
                if self.b.is_inside(i1.point) {
                    Some(i1)
                } else {
                    None
                }
            }
            (None, None) => None,
        }
    }

    fn is_inside(&self, p: Point2) -> bool {
        self.a.is_inside(p) && self.b.is_inside(p)
    }
}
