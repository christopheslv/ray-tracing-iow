use std::io;
use std::io::Write; 
mod vec3;
mod color;
mod ray;

#[allow(dead_code)]
fn test_vec() {
    let orig = vec3::Point3::new(0.0, 0.0, 0.0);
    let direction = vec3::Point3::new(1.0, 1.0, 0.0);
    let ray = ray::Ray::new(orig, direction);

    println!("v1 cross v2 {}", ray.at(10.0));
}

fn render() {
   // Image
   let img_width = 512;
   let img_height = 512;

   // Render
   writeln!(io::stdout(),"P3\n{} {}\n255", img_width, img_height).unwrap(); 

   for j in (0..=(img_height-1)).rev() {
       write!(io::stderr(),  "\rScanlines remaining: {} ", j).unwrap(); 

       for i in 0..img_width {
           let r = i as f64 / (img_width-1) as f64;
           let g = j as f64 / (img_height-1) as f64;
           let b = 0.25;

           let pixel_color = vec3::Color::new(r,g,b);
           color::write_color(pixel_color);
       }
   }

   writeln!(io::stderr(),"\nDone.").unwrap();  
}

fn main() {
    render(); 
}
