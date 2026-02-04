fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let r_precise = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g_precise = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b_precise = 0.0;

            let r = (255.999 * r_precise) as u8;
            let g = (255.999 * g_precise) as u8;
            let b = (255.999 * b_precise) as u8;

            println!("{r} {g} {b}");
        }
    }
    eprintln!("Done!");
}
