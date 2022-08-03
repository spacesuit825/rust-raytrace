use crate::point::Point;
use crate::vector::Vector3;
use crate::render::{Ray, Intersectable};
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use std::ops::{Mul, Add};

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Colour {
    pub fn clamp(&self) -> Colour {
        Colour {
            red: self.red.min(1.0),
            green: self.green.min(1.0),
            blue: self.blue.min(1.0),
        }
    }
}

impl Add for Colour {
    type Output = Colour;
    fn add(self, other: Colour) -> Colour {
        Colour {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

impl<'a> Mul for &'a Colour {
    type Output = Colour;

    fn mul(self, other: &'a Colour) -> Colour {
        Colour {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, other: f32) -> Colour {
        Colour {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}

impl Mul<Colour> for f32 {
    type Output = Colour;
    fn mul(self, other: Colour) -> Colour {
        other * self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub colour: Colour,
    pub albedo: f32,
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub colour: Colour,
    pub intensity: f32,
}

pub struct SphericalLight {
    pub position: Point,
    pub colour: Colour,
    pub intensity: f32,
}


pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn colour(&self) -> Colour {
        match *self {
            Light::Directional(ref d) => d.colour,
            Light::Spherical(ref s) => s.colour,
        }
    }
    pub fn direction_from(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Light::Directional(ref d) => -d.direction,
            Light::Spherical(ref s) => (s.position - *hit_point).normalize(),
        }
    }
    pub fn intensity(&self, hit_point: &Point) -> f32 {
        match *self {
            Light::Directional(ref d) => d.intensity,
            Light::Spherical(ref s) => {
                let r2 = (s.position - *hit_point).norm() as f32;
                s.intensity / (4.0 * ::std::f32::consts::PI * r2)
            }
        }
    }
    pub fn distance(&self, hit_point: &Point) -> f64 {
        match *self {
            Light::Directional(_) => ::std::f64::INFINITY,
            Light::Spherical(ref s) => (s.position - *hit_point).length(),
        }
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub light: Vec<Light>,
    pub shadow_bias: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub colour: Colour,
    pub albedo: f32,
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn colour(&self) -> &Colour {
        match *self {
            Element::Sphere(ref s) => &s.colour,
            Element::Plane(ref p) => &p.colour,
        }
    }

    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.albedo,
            Element::Plane(ref p) => p.albedo,
        }
    }
}


pub struct Intersection<'a> {
    pub distance: f64,
    pub elements: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        
        Intersection {
            distance: distance,
            elements: element,
        }
    }
}


impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}

