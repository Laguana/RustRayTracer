use crate::base::ray::Ray;
use crate::base::color::RGBA;
use crate::base::ray::Triple;

pub trait Tracable {
    // return the distances along the ray (including backwards) to intersections
    fn intersect(&self, ray: &Ray) -> Vec<f32>;
}

pub struct Material {
    pub color: RGBA,
    pub reflectivity: f32,
    pub normal: Triple,
    pub refractive_index: f32,
}

pub trait Renderable {
    fn material(&self, point: &Triple) -> Material;
}

pub trait Drawable : Tracable + Renderable {

}