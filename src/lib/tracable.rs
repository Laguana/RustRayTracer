use crate::lib::ray::Ray;
use crate::lib::color::RGBA;
use crate::lib::ray::Triple;

pub trait Tracable {
    // return the distances along the ray (including backwards) to intersections
    fn intersect(&self, ray: &Ray) -> Vec<f32>;
}

pub trait Renderable {
    fn color(&self, ray: &Ray, point: &Triple) -> RGBA;
}

pub trait Drawable : Tracable + Renderable {

}