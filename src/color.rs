use std::io;
use std::io::Write; 
use super::vec3;

pub fn write_color(pixel_color:vec3::Color) {
    let ir = (255.999 * pixel_color.x()) as u32;
    let ig = (255.999 * pixel_color.y()) as u32;
    let ib = (255.999 * pixel_color.z()) as u32;

    writeln!(io::stdout(),"{} {} {}", ir, ig, ib).unwrap();
}