use crate::ray;

pub struct Camera {}

pub fn new() -> Camera {
    Camera {}
}

impl Camera {
    pub fn create_ray(&self, x: u32, y: u32) -> ray::Ray {
        // TODO: create a ray with the given offset on the image plane into the world
        ray::Ray {}
    }
}
