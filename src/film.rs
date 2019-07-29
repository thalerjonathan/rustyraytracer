use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::vec::Vec;

use crate::camera;
use crate::colour;
use crate::scene;

// a film is a representation of the 2D image plane and stores the color values
// of every pixel in that plane
pub struct Film {
    resolution: (u32, u32),
    plane: Vec<Vec<colour::Colour>>,
}

impl Film {
    // creates a film with an imageplane of the given resolution and intialises
    // it with black opaqe pixels
    // TODO: make it an associated method with Film
    pub fn new(width: u32, height: u32) -> Film {
        // NOTE: u32 always fits into usize
        let mut plane = Vec::with_capacity(height as usize);

        for _y in 0..height {
            let mut row = Vec::with_capacity(width as usize);

            // film is unlit initially, initialise with black opaque pixels,
            for _x in 0..width {
                row.push(colour::Colour::new());
            }

            plane.push(row);
        }

        Film {
            resolution: (width, height),
            plane: plane,
        }
    }

    pub fn illuminate(&mut self, camera: &camera::Camera, scene: &scene::Scene) {
        print!("Illuminating film... ");
        io::stdout().flush().unwrap(); // print! needs a flush...

        for y in 0..self.resolution.1 {
            for x in 0..self.resolution.0 {
                let ray = camera.create_ray(x, y);
                let c = scene.intersect(&ray);
                // NOTE: u32 fits into usize
                self.plane[y as usize][x as usize] = c;
            }
        }

        println!("finished.");
    }

    pub fn write_to_file(&self, filename: &str) {
        print!("Writing film to file... ");
        io::stdout().flush().unwrap(); // print! needs a flush...

        let path = Path::new(filename);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.resolution.0 as u32, self.resolution.1 as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        // allocate a buffer to hold RGB for every pixel
        let channels = 3; // ignore alpha channel
        let data_size = self.resolution.0 * self.resolution.1 * channels;
        let mut data = Vec::with_capacity(data_size as usize);

        for row in &self.plane {
            for p in row {
                data.push(p.r);
                data.push(p.g);
                data.push(p.b);
                // ignore alpha channel
            }
        }

        writer.write_image_data(data.as_slice()).unwrap();

        println!("finished.");
    }
}
