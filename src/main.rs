use std::io;
use std::io::Write; 
mod vec3;

#[allow(dead_code)]
fn write_img() {
    // Image
    let img_width = 256;
    let img_height = 256;

    // Render
    writeln!(io::stdout(),"P3\n{} {}\n255", img_width, img_height).unwrap(); 

    let mut j = img_height-1;
    
    while j >= 0 {
        let mut i = 0;
        write!(io::stderr(),  "\rScanlines remaining: {} ", j).unwrap(); 

        while i < img_width{
            let r = i as f64 / (img_width-1) as f64;
            let g = j as f64 / (img_height-1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            writeln!(io::stdout(),"{} {} {}", ir, ig, ib).unwrap();
            i += 1;
        }

        j -= 1;
    }

    writeln!(io::stderr(),"\nDone.").unwrap();  
}

fn main() {
    let unit = vec3::Vec3::new(1.0, 1.0, 1.0);
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
