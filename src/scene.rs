use crate::colour;
use crate::ray;

pub struct Scene {
    background: colour::Colour,
}

pub struct Intersection {
    pub colour: colour::Colour,
}

impl Scene {
    pub fn intersect(&self, ray: &ray::Ray) -> Intersection {
        // TODO: intersect ray with scene and compute lighting
        Intersection {
            colour: self.background.clone(),
        }
    }
}

pub fn new() -> Scene {
    Scene {
        background: colour::new_opaqe(255, 0, 0),
    }
}
