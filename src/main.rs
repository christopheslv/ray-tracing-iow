use std::io;
use std::io::Write; 
use std::rc::Rc;

mod vec3;
mod color;
mod camera;
mod ray;
mod hittable;
mod sphere;
mod hittablelist;
mod utils;
mod material;

use vec3::Point3 as Point3;
use vec3::Vec3 as Vec3;
use vec3::Color as Color;
use ray::Ray as Ray;
use camera::Camera as Camera;
use sphere::Sphere as Sphere;
use hittable::Hittable;
use hittablelist::HittableList as HittableList;

fn ray_color(r:Ray, world:&mut HittableList, depth:u32) -> Color {
    let mut rec = world.empty_hit_record();

     // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::default();
    }
     
    if world.hit(r, 0.001, utils::INFINITY, &mut rec) {

        let mut scattered:Ray = Ray::default();
        let mut attenuation:Color = Color::default();

        let mat = rec.mat.clone();

        if mat.scatter(r, rec, &mut attenuation, &mut scattered) {
            return ray_color(scattered, world, depth-1) * attenuation;
        }

        return Color::default();
    }

    let unit_direction:Vec3 = vec3::unit_vector(r.direction());
    let t:f64 = 0.5*(unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0)*(1.0-t) + Color::new(0.5, 0.7, 1.0)*t
}

fn randrom_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(material::Lambertian::new(Color::new( 0.5, 0.5, 0.5)));
    world.add(Box::new( Sphere::new(Point3::new( 0.0, -1000.0, 0.0), 1000.0, ground_material.clone()) ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_one();
            let center = Point3::new(   a as f64 + 0.9*utils::random_one(),
                                        0.2,
                                        b as f64 + 0.9*utils::random_one());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material:Rc<dyn material::Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_unit() * Color::random_unit();
                    sphere_material = Rc::new(material::Lambertian::new(albedo));
                    world.add(Box::new( Sphere::new(center, 0.2, sphere_material.clone()) ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = utils::random_float(0.0, 0.5);
                    sphere_material = Rc::new(material::Metal::new(albedo, fuzz));
                    world.add(Box::new( Sphere::new(center, 0.2, sphere_material.clone()) ));

                } else {
                    // glass
                    sphere_material = Rc::new(material::Dielectric::new(1.5));
                    world.add(Box::new( Sphere::new(center, 0.2, sphere_material.clone()) ));
                }
            }                           

        }
    }

    let material1 = Rc::new(material::Dielectric::new(1.5));
    world.add(Box::new( Sphere::new(Point3::new( 0.0, 1.0, 0.0), 1.0, material1.clone()) ));

    let material2 = Rc::new(material::Lambertian::new(Color::new( 0.4, 0.2, 0.1)));
    world.add(Box::new( Sphere::new(Point3::new( -4.0, 1.0, 0.0), 1.0, material2.clone()) ));

    let material3 = Rc::new(material::Metal::new(Color::new( 0.7, 0.6, 0.5), 0.0));
    world.add(Box::new( Sphere::new(Point3::new( 4.0, 1.0, 0.0), 1.0, material3.clone()) ));

    world
}

fn main() {

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let img_width = 800;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 250;
    let max_depth = 50;

    // World
    let mut world = randrom_scene();
  

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new( 0.0, 0.0, 0.0);
    let vfov = Vec3::new( 0.0, 1.0, 0.0);
    let dist_to_focus =  10.0;
    let aperture = 0.1;
    let camera = Camera::new(lookfrom, lookat, vfov, 20.0, aspect_ratio, aperture, dist_to_focus);

    // Render
    writeln!(io::stdout(),"P3\n{} {}\n255", img_width, img_height).unwrap(); 

    for j in (0..=(img_height-1)).rev() {
        write!(io::stderr(),  "\rScanlines remaining: {}", j).unwrap(); 

        for i in 0..img_width {
            let mut pixel_color = Color::default();

                for _s in 0..samples_per_pixel {
                    let u = (i as f64 + utils::random_one()) / (img_width-1) as f64;
                    let v = (j as f64 + utils::random_one()) / (img_height-1) as f64;
                    let r = camera.get_ray(u,v);

                    pixel_color = pixel_color + ray_color(r, &mut world, max_depth);
                } 

            color::write_color(pixel_color, samples_per_pixel);
        }
    }

    writeln!(io::stderr(),"\nDone.").unwrap();  
}
