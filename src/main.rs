use crate::vec3::Vec3;

type Point3 = Vec3;
type Color = Vec3;


mod vec3;

fn main() {
    let (width, height) = (256, 256);

    println!("P3\n{} {}\n255\n", width, height);

    for j in (0..height).rev() {
        eprintln!("Scanline remaining: {}", j);
        for i in 0..width {
            let pixel_color = Color::new(i as f64 / (width - 1) as f64, j as f64 / (height - 1) as f64, 0.25);
            println!("{}", colorize(pixel_color));
        }
    }
}

fn colorize(color: Color) -> String {
    format!(
        "{} {} {}",
        (255.999 * color.x()) as usize,
        (255.999 * color.y()) as usize,
        (255.999 * color.z()) as usize
    )
}
