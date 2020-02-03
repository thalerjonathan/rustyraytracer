use glm::*;
use crate::ray;

pub struct Camera {
    view: glm::Matrix4<f32>,    // viewing transformation
    proj: glm::Matrix4<f32>,    // projection transformation
  
    rasterToScreen: glm::Matrix4<f32>,
    screenToRaster: glm::Matrix4<f32>,
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
    // x and y are raster coordinates (pixels), starting top left of the image 
    // plane with 0/0 (pixels)
    pub fn create_ray(&self, x: u32, y: u32) -> ray::Ray {
        // TODO: transform the x, y coordinates of the image plane into the 
        // 3d viewing coordinates
        let rc = vec4(x as f32, y as f32, 1., 0.);

        // tansform raster coordinates into screen coordinates ranging
        // from 0 to 1 in both dimensions

        // transform screen coordinates into NDC (normalised device coordinates)
        // ranging from -1 to +1 in all 3 dimensions

        // TODO: it is unclear what we need to do, look at PBRT camera model
        // we basically need to do the inverse of an OpenGL viewing pipeline

        let start = glm::vec3(0.,0.,0.);
        let direction = glm::vec3(0., 0., 0.);

        ray::Ray::new(start, direction)
    }
}
