// struct Plane {
//     center:
//     px: f64,
//     py: f64,
//     nx: f64,
//     ny: f64,
// }

// impl Shape for Plane {
//     fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Option<Intersection> {
//         let a = d.0 * self.nx + d.1 * self.ny;
//         if a.abs() < EPSILON {
//             None
//         } else {
//             let b = (self.px - p.0) * self.nx + (self.py - p.1) * self.ny;
//             let t = b / a;
//             if t > EPSILON {
//                 Some(Intersection {
//                     point: (p.0 + d.0 * t, p.1 + d.1 * t),
//                     normal: (self.nx, self.ny),
//                 })
//             } else {
//                 None
//             }
//         }
//     }

//     fn is_inside(&self, p: (f64, f64)) -> bool {
//         (p.0 - self.px) * self.nx + (p.1 - self.py) * self.ny < 0.0
//     }
// }
