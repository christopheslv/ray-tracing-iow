use super::vec3::Point3 as Point3;
use super::vec3::Vec3 as Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3
}  

impl Ray {
    pub fn new(origin:Point3, direction:Vec3) -> Ray {
        Ray{
            orig : origin,
            dir : direction,
        }
    }
    
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    #[allow(dead_code)]
    pub fn at(&self, t:f64) -> Point3 {
        return self.orig + self.dir * t
    }
}

impl Default for Ray {
    fn default () -> Ray {
        Ray{
            orig : Point3::new(0.0,0.0,0.0),
            dir: Vec3::new(0.0,0.0,0.0),
        }
    }
}