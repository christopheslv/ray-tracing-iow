use super::ray::Ray as Ray;
use super::vec3::Point3 as Point3;
use super::vec3::Vec3 as Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}  

impl Default for HitRecord {
    fn default () -> HitRecord {
        HitRecord{ p:Point3::default(), normal:Vec3::default(), t:0.0 }
    }
}

pub trait Hittable {
    fn hit(&self, r:Ray, t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool;
}