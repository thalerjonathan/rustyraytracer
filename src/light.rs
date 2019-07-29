use glm::Vector3;

// simple point light
pub struct Light {
    pos: glm::Vector3<f32>,
}

impl Light {
    pub fn new(pos: glm::Vector3<f32>) -> Light {
        Light { pos: pos }
    }
}
