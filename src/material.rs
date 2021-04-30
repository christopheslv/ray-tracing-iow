use super::vec3::Color as Color;
use super::vec3::Vec3 as Vec3;
use super::vec3;
use super::ray::Ray as Ray;
use super::hittable::HitRecord as HitRecord;
use super::utils;

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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new(a:Color, f:f64) -> Metal {
        Metal{
            albedo : a,
            fuzz: f.min(1.0),
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

// Dielectric Material

pub struct Dielectric {
    pub ir:f64, // Index of Refraction
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(index_of_refraction:f64) -> Dielectric {
        Dielectric{
            ir : index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in:Ray, rec:HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        *attenuation = Color::new( 1.0, 1.0, 1.0);
        let mut refraction_ratio = self.ir;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        }

        let unit_direction = vec3::unit_vector( r_in.direction() );
        let cos_theta = vec3::dot( -unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction:Vec3;

        if cannot_refract || utils::reflectance(cos_theta, refraction_ratio) > utils::random_one() {
            direction = vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = vec3::refract(unit_direction, rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.p, direction);

        true
    }
}