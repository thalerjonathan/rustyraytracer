use crate::intersection;
use crate::material;
use crate::ray;
use glm::Vector3;

pub struct Sphere {
    pos: glm::Vector3<f32>,
    radius: f32,
    material: material::Material,
}

impl Sphere {
    pub fn new(pos: glm::Vector3<f32>, radius: f32, material: material::Material) -> Sphere {
        Sphere {
            pos: pos,
            radius: radius,
            material: material,
        }
    }

    pub fn intersect(&self, ray: &ray::Ray) -> Option<intersection::Intersection> {
        // TODO: implement ray-sphere intersection
        
        None
    }
}
