use crate::colour;
use crate::light;
use crate::ray;

pub struct Scene {
    background: colour::Colour,
    //objects: Vec<Intersectable>,
    //lights: Vec<light::Light>,
}

pub trait Intersectable {
    fn intersect(&self, ray: &ray::Ray) -> Intersection;
}

pub struct Intersection {
    pub colour: colour::Colour,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            background: colour::Colour::new_opaqe(255, 0, 0),
        }
    }

    pub fn intersect(&self, ray: &ray::Ray) -> Intersection {
        // TODO: intersect ray with scene and compute lighting
        Intersection {
            colour: self.background.clone(),
        }
    }
}
