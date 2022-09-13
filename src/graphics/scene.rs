use nalgebra::distance_squared;

use super::entity::{Entity, EntityIntersection};
use crate::geometry::{shape::Intersection, types::*};

pub(crate) struct Scene {
    pub(crate) entities: Vec<Entity>,
}

impl Scene {
    pub fn intersect_closest(&self, p: Point2, d: UVec2) -> Option<(Entity, Intersection)> {
        unimplemented!()
    }
    // pub fn intersect(&self, p: Point2, d: UVec2) -> Option<EntityIntersection> {
    //     let mut res: Option<EntityIntersection> = None;
    //     for e in &self.entities {
    //         if let Some(intersection) = e.intersect(p, d) {
    //             res = match res {
    //                 Some(r) => {
    //                     if distance_squared(&p, &r.point)
    //                         > distance_squared(&p, &intersection.point)
    //                     {
    //                         Some(intersection)
    //                     } else {
    //                         Some(r)
    //                     }
    //                 }
    //                 None => Some(intersection),
    //             }
    //         }
    //     }
    //     res
    // }
}
