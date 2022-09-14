use nalgebra::distance_squared;

use super::entity::{Entity, EntityIntersection};
use crate::geometry::{shape::Intersection, types::*};

pub(crate) struct Scene {
    pub(crate) entities: Vec<Entity>,
}

impl Scene {
    pub fn intersect_closest(&self, p: Point2, d: UVec2) -> Option<(&Entity, Intersection)> {
        let mut res: Option<(&Entity, Intersection)> = None;
        for e in &self.entities {
            if let Some(intersection) = e.intersect(p, d) {
                res = match res {
                    Some(prev_res) => {
                        if distance_squared(&p, &prev_res.1.point)
                            > distance_squared(&p, &intersection.point)
                        {
                            Some((e, intersection))
                        } else {
                            Some(prev_res)
                        }
                    }
                    None => Some((e, intersection)),
                }
            }
        }
        res
    }
}
