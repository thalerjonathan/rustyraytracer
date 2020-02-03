use crate::colour;
use crate::intersection;
use crate::light;
use crate::material;
use crate::ray;
use crate::sphere;
use glm::*;

pub struct Scene {
    background: colour::Colour,
    objects: Vec<sphere::Sphere>,
    lights: Vec<light::Light>,
}

impl Scene {
    pub fn new() -> Scene {
        let m1 = material::Material {
            diffuse: colour::Colour::new_opaqe(255, 255, 51), // yellow
            reflectivity: 0.,     // diffuse only
            transparency: 0.,     // diffuse only
            refractive_index: 0., // diffuse only
        };

        let m2 = material::Material {
            diffuse: colour::Colour::new_opaqe(255, 255, 255),
            reflectivity: 0.,
            transparency: 1.,       // perfect transparent
            refractive_index: 2.42, // diamond
        };

        let m3 = material::Material {
            diffuse: colour::Colour::new_opaqe(255, 255, 255),
            reflectivity: 0.8, // nearly completely reflective
            transparency: 0.,
            refractive_index: 0.,
        };

        let l = light::Light::new(vec3(0., 10., 0.));
        let s1 = sphere::Sphere::new(vec3(-4., 2., 0.), 2., m1);
        let s2 = sphere::Sphere::new(vec3(0., 2., 0.), 2., m2);
        let s3 = sphere::Sphere::new(vec3(4., 2., 0.), 2., m3);

        let os = vec![s1, s2, s3];
        let ls = vec![l];

        Scene {
            background: colour::Colour::new_opaqe(135, 206, 235), // sky blue
            objects: os,
            lights: ls,
        }
    }

    pub fn intersect(&self, ray: &ray::Ray) -> colour::Colour {
        for o in &self.objects {
            match o.intersect(ray) {
                Some(i) => {
                    let mut c = i.material.diffuse;

                    // material is (perfect) reflective, 
                    if i.material.reflectivity > 0. {
                        // TODO: shoot outgoing ray mirrored at normal
                        // TODO: add up color to c
                    }

                    // material is transparent
                    if i.material.transparency > 0. {
                        // TODO: shoot refractive ray following refraction index
                        // TODO: add up colors to c
                    }

                    // TODO: shoot ray to each light
                    c
                }
                None => self.background.clone(),
            };
        }

        // in case there are no objects...
        self.background.clone()
    }
}
