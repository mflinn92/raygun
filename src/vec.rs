use std::ops;

#[derive(Debug, PartialEq)]
struct Vec3(f64, f64, f64);

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }

    fn r(&self) -> f64 {
        self.x()
    }

    fn g(&self) -> f64 {
        self.y()
    }

    fn b(&self) -> f64 {
        self.z()
    }

    fn length(&self) -> f64 {
        f64::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z())
    }

    /// Consumes self and makes returns an `Option<Vec3>` new unit vector in the direction of self
    /// If length of self is 0 `None` will be returned
    fn unit_vec(self) -> Option<Self> {
        if self.length() == 0.0 {
            return None;
        }
        let divisor = 1.0 / self.length();
        Some(Self(
            self.x() * divisor,
            self.y() * divisor,
            self.z() * divisor,
        ))
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();

        Self(x, y, z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();

        Self(x, y, z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_and_get_vec_fields() {
        let v = Vec3::new(1 as f64, 2 as f64, 3 as f64);

        let x = v.x();
        let r = v.r();
        assert_eq!(x, r);

        let y = v.y();
        let g = v.g();
        assert_eq!(g, y);

        let z = v.z();
        let b = v.b();
        assert_eq!(b, z);
    }

    #[test]
    fn calculate_vec_length() {
        let v = Vec3::new(0.0, 4.0, 3.0);
        assert_eq!(v.length(), 5.0)
    }

    #[test]
    fn add_two_vecs() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0))
    }

    #[test]
    fn subtract_two_vecs() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v1 - v2, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn negate_vectors() {
        let v = Vec3::new(2.0, -5.0, 1.0);
        assert_eq!(-v, Vec3::new(-2.0, 5.0, -1.0));
    }

    #[test]
    fn unit_vec() {
        let v = Vec3::new(0.0, 4.0, 3.0);
        let u = v.unit_vec().unwrap();
        assert!(u.length() == 1.0);
    }

    #[test]
    fn multiply_vecs() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
    }
}
