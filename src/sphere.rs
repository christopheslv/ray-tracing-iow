use super::vec3::Point3 as Point3;
use super::vec3;
use super::ray::Ray as Ray;
use super::material::Material as Material;

use super::hittable::HitRecord as HitRecord;
use super::hittable::Hittable as Hittable;

use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}  

impl Sphere {
    pub fn new(cent:Point3, r:f64, m:Rc<dyn Material>) -> Sphere {
        Sphere{
            center : cent,
            radius : r,
            mat : m,
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, r:Ray, t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 {
            return false;
        }
   
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;   
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
             
        return true;
    }
}