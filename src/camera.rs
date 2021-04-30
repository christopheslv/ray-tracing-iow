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
    v: Vec3,
    u: Vec3,
    lens_radius:f64,
}  

impl Camera {
    pub fn new(lookfrom:Vec3, lookat:Vec3, vup:Vec3, vfov:f64, aspect_ratio:f64, aprture:f64, focus_dist:f64) -> Camera {
        let theta = utils::degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let ww = vec3::unit_vector(lookfrom - lookat);
        let uu = vec3::unit_vector(vec3::cross(vup, ww));
        let vv = vec3::cross(ww, uu);

        let orig = lookfrom;
        let horiz = uu * viewport_width * focus_dist;
        let vert = vv * viewport_height * focus_dist;
        let llc = orig - horiz / 2.0 - vert / 2.0 -ww*focus_dist;

        Camera{
           origin: orig, 
           horizontal: horiz, 
           vertical: vert, 
           lower_left_corner: llc,
           u: uu,
           v: vv,
           lens_radius: aprture / 2.0,
        }
    }

    pub fn get_ray(&self, s:f64, t:f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v* rd.y();
        Ray::new(self.origin + offset, 
            self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset)
    }
}