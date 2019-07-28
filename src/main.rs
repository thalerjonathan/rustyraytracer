mod camera;
mod colour;
mod film;
mod light;
mod ray;
mod scene;

fn main() {
    let filename = "image.png";

    let mut film = film::Film::new(800, 600);
    let camera = camera::Camera::new();
    let scene = scene::Scene::new();

    film.illuminate(&camera, &scene);
    film.write_to_file(&filename);
}
