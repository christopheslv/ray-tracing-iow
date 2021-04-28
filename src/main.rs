use std::io;
use std::io::Write; 
mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;

use vec3::Point3 as Point3;
use vec3::Vec3 as Vec3;
use vec3::Color as Color;
use ray::Ray as Ray;
use sphere::Sphere as Sphere;
use hittable::HitRecord as HitRecord;
use hittable::Hittable;

fn ray_color(r:Ray, s:Sphere) -> Color {
    let mut hitrec = HitRecord::default();

    let did_hit = s.hit(r, 0.0, 200.0, &mut hitrec);
    if did_hit {
        return Color::new(hitrec.normal.x()+1.0, hitrec.normal.y()+1.0, hitrec.normal.z()+1.0) * 0.5;
    }

    let unit_direction:Vec3 = vec3::unit_vector(r.direction());
    let t:f64 = 0.5*(unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0)*(1.0-t) + Color::new(0.5, 0.7, 1.0)*t
}

fn generate_image() {
   // Image
   let aspect_ratio = 16.0 / 9.0;
   let img_width = 800;
   let img_height = (img_width as f64 / aspect_ratio) as u32;

   // World
   let sphere = Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5);
   
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
           let pixel_color = ray_color(r, sphere);
           color::write_color(pixel_color);
       }
   }

   writeln!(io::stderr(),"\nDone.").unwrap();  
}

fn main() {
    generate_image(); 
}
