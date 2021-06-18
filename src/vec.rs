#![allow(dead_code)]
/// All vec operations will consume their arguments and return a new Vec3
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

    fn any(&self, val: f64) -> bool {
        if float_cmp(self.x(), val) || float_cmp(self.y(), val) || float_cmp(self.z(), val) {
            return true;
        }
        false
    }

    fn dot(self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    fn cross(self, rhs: Self) -> Self {
        let x = self.y() * rhs.z() - self.z() * rhs.y();
        let y = self.x() * rhs.z() - self.z() * rhs.x();
        let z = self.x() * rhs.y() - self.y() * rhs.x();

        Self(x, y, z)
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

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        let z = self.z() - rhs.z();

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

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Div for Vec3 {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.any(0.0) {
            return None;
        }
        Some(Self(
            self.x() / rhs.x(),
            self.y() / rhs.y(),
            self.z() / rhs.z(),
        ))
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Option<Self>;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return None;
        }
        Some(Self(self.x() / rhs, self.y() / rhs, self.z() / rhs))
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

    #[test]
    fn scalar_mult() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 4.0, Vec3::new(4.0, 8.0, 12.0));
    }

    #[test]
    fn div_two_vecs() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 6.0);

        assert_eq!((v2 / v1).unwrap(), Vec3::new(3.0, 2.0, 2.0));

        let v_fail = Vec3::new(1.0, 0.0, 2.0);
        let v = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(v / v_fail, None);
    }

    #[test]
    fn div_by_scalar() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!((v / 2.0).unwrap(), Vec3::new(1.0, 2.0, 3.0));

        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v / 0.0, None);
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v1.dot(v2), 20.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(v2), Vec3::new(-1.0, -2.0, -1.0));
    }
}

fn float_cmp(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}
