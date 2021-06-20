#![allow(dead_code)]
use crate::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a, b }
    }

    fn origin(&self) -> Vec3 {
        self.a
    }

    fn direction(&self) -> Vec3 {
        self.b
    }

    fn point_at(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }

    pub fn color(&self) -> Option<Vec3> {
        let unit = match self.direction().unit_vec() {
            Some(v) => v,
            None => return None,
        };

        let t = 0.5 * (unit.y() + 1.0);
        // creates a background gradient
        Some((1.0 - t) * Vec3::new(1.0, 1.0, 2.0) + t * Vec3::new(0.5, 0.7, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_works() {
        let a = Vec3::new(0.0, 1.0, 0.0);
        let b = Vec3::new(1.0, 1.0, 1.0);

        let r = Ray::new(a, b);

        let result = r.point_at(2.0);
        assert_eq!(result, Vec3::new(2.0, 3.0, 2.0));
    }
}
