use crate::colour;

// NOTE: this is a VERY simplified material definition for a simple whitted
// raytracer
pub struct Material {
    pub diffuse: colour::Colour,
    pub reflectivity: f32, // how reflective is the material. if == 1 then perfect reflection, no diffuse colour
    pub transparency: f32, // how transparent is the material. if ==1 then completely transparent, no diffuse colour
    pub refractive_index: f32, // the refractive index in case of transparency
}
