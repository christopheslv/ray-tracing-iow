use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, Neg, Index, IndexMut};
use std::marker::{Copy};
use super::utils;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3]  // We could just use x,y,z attributes, kept an array to match book's "API"
}  

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3{
            e : [x,y,z],
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }
    
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2];
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        return (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s);
    }

    #[allow(dead_code)]
    pub fn random_unit() -> Vec3 {
        let x = utils::random_one();
        let y = utils::random_one();
        let z = utils::random_one();

        Vec3{
            e : [x,y,z],
        }
    }

    pub fn random(min:f64, max:f64) -> Vec3 {
        let x = utils::random_float(min,max);
        let y = utils::random_float(min,max);
        let z = utils::random_float(min,max);

        Vec3{
            e : [x,y,z],
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }else {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    #[allow(dead_code)]
    pub fn random_in_hemisphere(normal:Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(in_unit_sphere, normal) > 0.0 { // In the same hemisphere as the normal
            return in_unit_sphere;
        }else {
            return -in_unit_sphere;
        }
    }
 }

impl Default for Vec3 {
    fn default () -> Vec3 {
        Vec3{
            e : [0.0,0.0,0.0],
        }
    }
}


// Primitive operations
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {  e: [self.e[0] + other.e[0],
                            self.e[1] + other.e[1],
                            self.e[2] + other.e[2] ]}
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Self {  e: [self.e[0] * t,
                            self.e[1] * t,
                            self.e[2] * t]}
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = Self {  e: [self.e[0] / t,
                            self.e[1] / t,
                            self.e[2] / t]}
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {  e: [-self.e[0],
                    -self.e[1],
                    -self.e[2]]}
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

// Utility functions
impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3},{:.3},{:.3})", self.e[0], self.e[1], self.e[2])
    }
} 

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {  e: [self.e[0] + other.e[0],
                    self.e[1] + other.e[1],
                    self.e[2] + other.e[2] ]}
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {  e: [self.e[0] - other.e[0],
                    self.e[1] - other.e[1],
                    self.e[2] - other.e[2] ]}
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {  e: [self.e[0] * other.e[0],
                    self.e[1] * other.e[1],
                    self.e[2] * other.e[2] ]}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self {  e: [self.e[0] * t,
                    self.e[1] * t,
                    self.e[2] * t ]}
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {  e: [self.e[0] / other.e[0],
                    self.e[1] / other.e[1],
                    self.e[2] / other.e[2] ]}
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        Self {  e: [self.e[0] / t,
                    self.e[1] / t,
                    self.e[2] / t ]}
    }
}

pub fn dot(u:Vec3, v:Vec3) -> f64 {
    u.e[0] * v.e[0] +
    u.e[1] * v.e[1] +
    u.e[2] * v.e[2]
    
}
#[allow(dead_code)]
pub fn cross(u:Vec3, v:Vec3) -> Vec3 {
    Vec3 {  e: [u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0]]
    }
}

pub fn unit_vector(v:Vec3) -> Vec3 {
    v / v.length()
}