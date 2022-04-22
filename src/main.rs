#[allow(non_snake_case)]
mod ray;

use nalgebra::Vector3;
use ray::Ray;

type Color = Vector3<f64>;
type Point3 = Vector3<f64>;

fn write_color(color: &Color) {
    println!(
        "{} {} {}",
        (color.x * 255.99) as u8,
        (color.y * 255.99) as u8,
        (color.z * 255.99) as u8
    );
}

fn ray_color(r: &Ray) -> Color {
    let unity_direction: Vector3<f64> = r.direction.normalize();
    let t = 0.5 * (unity_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    //Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    //Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    //Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color: Color = ray_color(&ray);
            write_color(&pixel_color);
        }
    }
    eprint!("\nDone.\n");
}
