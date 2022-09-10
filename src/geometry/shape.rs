use crate::geometry::types::*;

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub point: Point2,
    pub normal: UVec2,
}

/// The Shape trait is characterized by an intersection function
/// and and is_inside function.
pub trait Shape {
    /// Returns an intersection, if it exists
    ///
    /// # Arguments
    ///
    /// * `p` - A vector which represents a point in space
    /// * `d` - A direction, which is represented by a unit vector
    fn intersect(&self, p: Point2, d: UVec2) -> Option<Intersection>;
    fn is_inside(&self, p: Point2) -> bool;
}
