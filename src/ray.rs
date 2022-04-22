use nalgebra::Vector3;

type Point3 = Vector3<f64>;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
