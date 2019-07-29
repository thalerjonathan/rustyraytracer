use crate::ray;

pub struct Camera {}

impl Camera {
    pub fn new() -> Camera {
        Camera {}
    }

    pub fn create_ray(&self, _x: u32, _y: u32) -> ray::Ray {
        // TODO: create a ray with the given offset on the image plane into the world
        ray::Ray {}
    }
}
