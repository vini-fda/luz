use crate::geometry::shape::Shape;
use crate::geometry::types::*;
use crate::graphics::color::Color;

pub(crate) struct EntityIntersection {
    pub point: Point2,
    pub normal: UVec2,
    pub emissive: Color,
    pub reflectivity: f64,
    pub eta: f64,
    pub absorption: Color,
}

pub(crate) struct Entity {
    pub shape: Box<dyn Shape + Sync>,
    pub emissive: Color,
    pub reflectivity: f64,
    pub eta: f64,
    pub absorption: Color,
}

impl Entity {
    pub fn intersect(&self, p: Point2, d: UVec2) -> Option<EntityIntersection> {
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
