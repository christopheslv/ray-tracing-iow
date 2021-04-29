use std::io;
use std::io::Write; 
use super::vec3;
use super::utils;

pub fn write_color(pixel_color:vec3::Color, samples_per_pixel:u32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // Write the translated [0,255] value of each color component.
    let ir = (255.999 * utils::clamp(r, 0.0, 0.999)) as u32;
    let ig = (255.999 * utils::clamp(g, 0.0, 0.999)) as u32;
    let ib = (255.999 * utils::clamp(b, 0.0, 0.999)) as u32;

    writeln!(io::stdout(),"{} {} {}", ir, ig, ib).unwrap();
}