use std::io;
use std::io::Write; 

mod vec3;
mod color;
mod camera;
mod ray;
mod hittable;
mod sphere;
mod hittablelist;
mod utils;

use vec3::Point3 as Point3;
use vec3::Vec3 as Vec3;
use vec3::Color as Color;
use ray::Ray as Ray;
use camera::Camera as Camera;
use sphere::Sphere as Sphere;
use hittable::HitRecord as HitRecord;
use hittable::Hittable;
use hittablelist::HittableList as HittableList;

fn ray_color(r:Ray, world:&impl Hittable, depth:u32) -> Color {
    let mut rec = HitRecord::default();

     // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::default();
    }
     
    if world.hit(r, 0.001, utils::INFINITY, &mut rec) {
        let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
        return ray_color( Ray::new(rec.p, target-rec.p), world, depth-1) * 0.5;
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
   let samples_per_pixel = 20;
   let max_depth = 50;

   // World
   let mut world = HittableList::default();
   world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
   world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));
   
   // Camera
   let camera = Camera::new();

   // Render
   writeln!(io::stdout(),"P3\n{} {}\n255", img_width, img_height).unwrap(); 

   for j in (0..=(img_height-1)).rev() {
       write!(io::stderr(),  "\rScanlines remaining: {} ", j).unwrap(); 

       for i in 0..img_width {
           let mut pixel_color = Color::default();

            for _s in 0..samples_per_pixel {
                let u = (i as f64 + utils::random_one()) / (img_width-1) as f64;
                let v = (j as f64 + utils::random_one()) / (img_height-1) as f64;
                let r = camera.get_ray(u,v);

                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            } 
           color::write_color(pixel_color, samples_per_pixel);
       }
   }

   writeln!(io::stderr(),"\nDone.").unwrap();  
}

fn main() {
    generate_image();      
}
