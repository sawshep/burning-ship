extern crate image;
extern crate num_complex;

use image::ImageBuffer;
use num_complex::Complex;
use std::io::{stdin, stdout, Write};
//use clap::{Arg, App};

// https://en.wikipedia.org/wiki/Burning_Ship_fractal#Implementation
fn burning_ship(real: f32, imag: f32, threshold: f32, max_iter: u32) -> f32 {
    let c = Complex { re: real, im: imag };
    let mut z = Complex { re: 0.0, im: 0.0 };

    let mut i: u32 = 0;
    while i <= max_iter as u32 {
        let addend = Complex {
            re: (z.re as f32).abs(),
            im: (z.im as f32).abs(),
        };
        z = addend * addend + c;
        if z.norm() > (threshold as f32) {
            return (i as f32) / (max_iter as f32);
        }
        z = (z * z) + c;
        i += 1;
    }
    return 1.0;
}

/// Parses the user input of a `prompt`
/// to a type provided in the turbofish.
fn prompt_to_num<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{} ", prompt);
        stdout().flush().expect("Failed to flush stdout buffer");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        match input.trim().parse::<T>() {
            Ok(n) => return n,
            Err(_) => println!("Invalid input"),
        }
    }
}

fn main() {
    let pixel_res = prompt_to_num::<u32>("Image side length:");
    let coord_center_x = prompt_to_num::<f32>("X center of coordinate sector:");
    let coord_center_y = prompt_to_num::<f32>("Y center of coordinate sector:");
    let coord_res = prompt_to_num::<f32>("Side length of coordinate sector:");
    let max_iter = prompt_to_num::<u32>("Maximum iterations:");
    let threshold = prompt_to_num::<f32>("Infinity threshold:");

    println!("Initializing variables...");
    let coord_min_x = coord_center_x - coord_res / 2.0;
    let coord_max_x = coord_center_x + coord_res / 2.0;
    let coord_min_y = coord_center_y - coord_res / 2.0;
    let coord_max_y = coord_center_y + coord_res / 2.0;

    let x_scale = (coord_max_x - coord_min_x) / pixel_res as f32;
    // Due to the way a pixel coordinate system
    // differs from a traditional system,
    // `y_scale` must be negative for fractals
    // that are NOT symmetrical about
    // the real axis to render properly.
    // This does not include the Burning Ship set
    let y_scale = (coord_max_y - coord_min_y) / pixel_res as f32;

    // TODO: To save memory, first save a blank
    // image to secondary storage, and then create
    // individual buffers for every `pixel_y`.
    let mut image_buffer = ImageBuffer::new(pixel_res, pixel_res);

    println!("Running...");
    for (pixel_x, pixel_y, pixel) in image_buffer.enumerate_pixels_mut() {
        let x_real = coord_min_x + (pixel_x as f32) * x_scale;
        let y_imag = coord_min_y + (pixel_y as f32) * y_scale;

        let color_multiplier = burning_ship(x_real, y_imag, threshold, max_iter);
        let rgb_value = (255.0 * color_multiplier) as u8;
        *pixel = image::Rgb([rgb_value, rgb_value, rgb_value]);
    }
    println!("Saving image to fractal.png...");
    image_buffer.save("fractal.png").unwrap();
    println!("Done");
}
