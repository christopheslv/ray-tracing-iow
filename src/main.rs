use std::io;
use std::io::Write; 
mod vec3;
mod color;

#[allow(dead_code)]
fn test_vec() {
    let mut v1 = vec3::Vec3::new(6.0, 5.15, 4.0);
    let v2 = vec3::Vec3::new(2.0, 2.0, 2.0);

    println!("v1{}", v1);
    v1 *= 2.0;
    println!("-v1{}", -v1);

    println!("v1 x {}",v1[0]);
    v1[0] = 35.40;
    println!("v1 x {}",v1[0]);

    println!("v1 {}",v1);
    println!("v2 {}",v2);
    println!("v1+v2 {}",v1+v2);
    println!("v1 {}",v1);

    println!("v1 cross v2 {}", vec3::cross(v1,v2));
}

fn main() {
    // Image
    let img_width = 256;
    let img_height = 256;

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
