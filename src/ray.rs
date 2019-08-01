use glm::Vector3;

pub struct Ray {
    pub start: glm::Vector3<f32>,
    pub direction: glm::Vector3<f32>,
}

impl Ray {
    pub fn new(start: glm::Vector3<f32>, direction: glm::Vector3<f32>) -> Ray {
      Ray { start: start, direction: direction }
    }
}