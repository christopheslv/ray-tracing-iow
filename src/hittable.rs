use super::ray::Ray as Ray;
use super::vec3::Point3 as Point3;
use super::vec3::Vec3 as Vec3;
use super::vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}  

impl HitRecord {
    pub fn set_face_normal(&mut self, r:Ray, outward_normal:Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

impl Default for HitRecord {
    fn default () -> HitRecord {
        HitRecord{ p:Point3::default(), normal:Vec3::default(), t:0.0 , front_face:false}
    }
}

pub trait Hittable {
    fn hit(&self, r:Ray, t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool;
}