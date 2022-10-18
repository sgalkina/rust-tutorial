mod vec;
mod color;

use vec::Color;
use color::write_color;

use std::fs::File;
use std::io::Write;
use std::io::stdout;

fn main() -> std::io::Result<()> {
    let im_width = 256;
    let im_height = 256;

    println!("Rendering an image with width {} and height {}", im_width, im_height);
    let mut file = File::create("image.ppm").expect("Unable to create file");
    let line = format!("P3\n{} {}\n255\n", im_width, im_height);
    file.write_all(line.as_bytes()).expect("Failed to write");
    for h in (0..im_height).rev() {
        print!("\rScanlines remaining: {}", h);
        stdout().flush();
        for w in 0..im_width {
            let color = Color {
                x: w as f64 / (im_width as f64 - 1.0),
                y: h as f64 / (im_height as f64 - 1.0),
                z: 0.25,
            };

            let line = write_color(&color);
            file.write_all(line.as_bytes()).expect("Failed to write");
        }
    }
    println!("\nDone");
    Ok(())
}