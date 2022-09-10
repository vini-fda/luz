use crate::geometry::shape::Shape;
use crate::geometry::types::*;
use crate::Color;

struct EntityIntersection {
    point: Point2,
    normal: UVec2,
    emissive: Color,
    reflectivity: f64,
    eta: f64,
    absorption: Color,
}

struct Entity {
    shape: Box<dyn Shape + Sync>,
    emissive: Color,
    reflectivity: f64,
    eta: f64,
    absorption: Color,
}

impl Entity {
    fn intersect(&self, p: Point2, d: UVec2) -> Option<EntityIntersection> {
        self.shape
            .intersect(p, d)
            .map(|intersection| EntityIntersection {
                point: intersection.point,
                normal: intersection.normal,
                emissive: self.emissive,
                reflectivity: self.reflectivity,
                eta: self.eta,
                absorption: self.absorption,
            })
    }
}
