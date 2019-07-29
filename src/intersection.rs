use crate::colour;
use crate::material;

use glm::Vector2;

pub struct Intersection {
    pub normal: glm::Vector3<f32>,
    pub material: material::Material,
}
