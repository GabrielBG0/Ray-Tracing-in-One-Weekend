#[allow(non_snake_case)]
use nalgebra::Vector3;

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

fn main() {
    let image_width = 256;
    let image_height = 256;
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color: Color = Vector3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            write_color(&pixel_color);
        }
    }
    eprint!("\nDone.\n");
}
