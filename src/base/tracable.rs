use crate::base::ray::Ray;
use crate::base::color::RGBA;
use crate::base::ray::Triple;

pub trait Tracable {
    // return the distances along the ray (including backwards) to intersections
    fn intersect(&self, ray: &Ray) -> Vec<f32>;
}

pub trait Renderable {
    fn material_color(&self, ray: &Ray, point: &Triple) -> RGBA;
    fn normal(&self, point: &Triple) -> Triple;
}

pub trait Drawable : Tracable + Renderable {

}