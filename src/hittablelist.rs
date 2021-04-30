use std::rc::Rc;

use super::ray::Ray as Ray;
use super::hittable::HitRecord as HitRecord;
use super::hittable::Hittable as Hittable;
use super::material::Material as Material;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    nomat: Rc<dyn Material>,
}  

impl HittableList {
    pub fn new() -> HittableList {
        HittableList{
            objects : Vec::new(),
            nomat: Rc::new(super::material::NoMaterial::new()),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn empty_hit_record(&mut self) -> HitRecord{
        HitRecord::new(self.nomat.clone())
    }
}


impl Hittable for HittableList {
    fn hit(&self, r:Ray, t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {

            let mut temp_rec = HitRecord::new(self.nomat.clone());
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}