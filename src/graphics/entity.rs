use crate::geometry::shape::{Intersection, Shape};
use crate::geometry::types::*;
use crate::graphics::color::Color;

use super::material::Material;

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
    pub material: Box<dyn Material + Sync>,
    // pub emissive: Color,
    // pub reflectivity: f64,
    // pub eta: f64,
    // pub absorption: Color,
}

impl Entity {
    pub fn intersect(&self, p: Point2, d: UVec2) -> Option<Intersection> {
        self.shape.intersect(p, d)
    }
}
