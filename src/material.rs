use super::vec3::Color as Color;
use super::vec3::Vec3 as Vec3;
use super::vec3;
use super::ray::Ray as Ray;
use super::hittable::HitRecord as HitRecord;

pub trait Material{
    fn scatter(&self, r_in:Ray, rec:HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool;
}

// Default material needed to initialize empty record reference
pub struct NoMaterial {}

impl NoMaterial {
    pub fn new() -> NoMaterial { NoMaterial{} }
}


impl Material for NoMaterial { 
    fn scatter(&self, _r_in:Ray, _rec:HitRecord, _attenuation:&mut Color, _scattered:&mut Ray) -> bool {
        false
    }
}

// Lambertian Diffuse Material

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(a:Color) -> Lambertian {
        Lambertian{
            albedo : a,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in:Ray, rec:HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}


// Metal Material

pub struct Metal {
    pub albedo:Color,
    pub fuzz:f64,
}

impl Metal {
    pub fn new(a:Color, f:f64) -> Metal {
        let mut ff = f;
        if ff < 1.0 {
            ff = 1.0;
        }

        Metal{
            albedo : a,
            fuzz: ff,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in:Ray, rec:HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        let reflected = vec3::reflect( vec3::unit_vector(r_in.direction()) , rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere()*self.fuzz);
        *attenuation = self.albedo;
        
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}