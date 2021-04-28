use std::io;
use std::io::Write; 
mod vec3;
mod color;
mod ray;

use vec3::Point3 as Point3;
use vec3::Vec3 as Vec3;
use vec3::Color as Color;
use ray::Ray as Ray;

fn hit_sphere(center:Point3, radius:f64, r:Ray) -> f64{
    let oc = r.origin() - center;
    let a = vec3::dot(r.direction(), r.direction());
    let b = vec3::dot(oc, r.direction()) * 2.0;
    let c = vec3::dot(oc,oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt() ) / (2.0*a);
    }
}

fn ray_color(r:Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r);
    if t > 0.0 {
        let normal = vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return Color::new(normal.x()+1.0, normal.y()+1.0, normal.z()+1.0) * 0.5;
    }
    let unit_direction:Vec3 = vec3::unit_vector(r.direction());
    let t:f64 = 0.5*(unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0)*(1.0-t) + Color::new(0.5, 0.7, 1.0)*t
}

fn generate_image() {
   // Image
   let aspect_ratio = 16.0 / 9.0;
   let img_width = 400;
   let img_height = (img_width as f64 / aspect_ratio) as u32;

   // Camera
   let viewport_height = 2.0;
   let viewport_width = aspect_ratio * viewport_height;
   let focal_length = 1.0;

   let origin = Point3::new(0.0, 0.0, 0.0);
   let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
   let vertical = Vec3::new(0.0, viewport_height, 0.0);

   let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

   // Render
   writeln!(io::stdout(),"P3\n{} {}\n255", img_width, img_height).unwrap(); 

   for j in (0..=(img_height-1)).rev() {
       write!(io::stderr(),  "\rScanlines remaining: {} ", j).unwrap(); 

       for i in 0..img_width {
           let u = i as f64 / (img_width-1) as f64;
           let v = j as f64 / (img_height-1) as f64;

           let r:Ray = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
           let pixel_color = ray_color(r);
           color::write_color(pixel_color);
       }
   }

   writeln!(io::stderr(),"\nDone.").unwrap();  
}

fn main() {
    generate_image(); 
}
