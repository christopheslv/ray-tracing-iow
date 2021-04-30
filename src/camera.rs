use super::utils;
use super::vec3::Point3 as Point3;
use super::vec3::Vec3 as Vec3;
use super::vec3;
use super::ray::Ray as Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}  

impl Camera {
    pub fn new(lookfrom:Vec3, lookat:Vec3, vup:Vec3, vfov:f64, aspect_ratio:f64) -> Camera {
        let theta = utils::degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(vup, w));
        let v = vec3::cross(w, u);

        let orig = lookfrom;
        let horiz = u * viewport_width;
        let vert = v * viewport_height;
        let llc = orig - horiz / 2.0 - vert / 2.0 -w;

        Camera{
           origin: orig, 
           horizontal: horiz, 
           vertical: vert, 
           lower_left_corner: llc,
        }
    }

    pub fn get_ray(&self, s:f64, t:f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin)
    }
}