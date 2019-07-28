mod camera;
mod colour;
mod film;
mod ray;
mod scene;

fn main() {
    let filename = "image.png";

    let mut film = film::new(19, 10);
    let camera = camera::new();
    let scene = scene::new();

    film.illuminate(&camera, &scene);
    film.write_to_file(&filename);
}
