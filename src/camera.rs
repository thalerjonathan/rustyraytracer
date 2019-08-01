use glm::*;
use crate::ray;

pub struct Camera {
    view: glm::Matrix4<f32>,    // viewing transformation
    proj: glm::Matrix4<f32>,    // projection transformation
}

impl Camera {
    pub fn new() -> Camera {
        // create camera positioned at origin (0/0/0),
        // oriented with positive y axis upwards
        // lookin down positive z axis
        let eye = vec3(0.,0.,0.);
        let center = vec3(0.,0.,1.);
        let up = vec3(0.,1.,0.);

        Camera { view: glm::ext::look_at(eye, center, up),
                 proj: glm::ext::perspective(90., 90., 0.1, 100.) }
    }

    // create a ray with the given offset on the image plane into the world
    // x and y start with 0, top left corner of the plane
    pub fn create_ray(&self, _x: u32, _y: u32) -> ray::Ray {
        // TODO: transform the x, y coordinates of the image plane into the 
        // 3d viewing coordinates

        // TODO: problem, x and y are in pixel coordinates, how do we get
        // the world coordinates? using inversion of viewing and projection transform

        let start = glm::vec3(0.,0.,0.);
        let direction = glm::vec3(0., 0., 0.);

        ray::Ray::new(start, direction)
    }
}
