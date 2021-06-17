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
}
