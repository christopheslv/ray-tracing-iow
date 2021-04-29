use super::vec3::Point3 as Point3;
use super::vec3::Vec3 as Vec3;
use super::ray::Ray as Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}  

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let orig = Point3::default();
        let horig = Vec3::new(viewport_width, 0.0, 0.0);
        let vert = Vec3::new(0.0, viewport_height, 0.0);
        let llc = orig - horig/2.0 - vert/2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera{
           origin: orig, horizontal: horig, vertical: vert, lower_left_corner: llc,
        }
    }

    pub fn get_ray(&self, u:f64, v:f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
    }
}