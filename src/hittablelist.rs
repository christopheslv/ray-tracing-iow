use super::ray::Ray as Ray;
use super::hittable::HitRecord as HitRecord;
use super::hittable::Hittable as Hittable;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}  

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default () -> HittableList {
        HittableList{
            objects : Vec::new(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r:Ray, t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}