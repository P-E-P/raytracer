fn main() {
    let (width, height) = (256, 256);

    println!("P3\n{} {}\n255\n", width, height);

    for j in (0..height).rev() {
        eprintln!("Scanline remaining: {}", j);
        for i in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as usize;
            let ig = (255.999 * g) as usize;
            let ib = (255.999 * b) as usize;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
