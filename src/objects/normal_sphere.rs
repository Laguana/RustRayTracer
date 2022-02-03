use crate::lib::ray::Ray;
use crate::lib::ray::Triple;
use crate::lib::tracable::Drawable;
use crate::lib::tracable::Renderable;
use crate::lib::tracable::Tracable;
use crate::shapes::sphere::Sphere;
use crate::lib::color::RGBA;

#[derive(Debug)]
pub struct NormalSphere {
    sphere: Sphere,
}

impl NormalSphere {
    pub fn new(sphere: Sphere) -> NormalSphere {
        NormalSphere { sphere }
    }
}

impl Tracable for NormalSphere {
    fn intersect(&self, r: &Ray) -> std::vec::Vec<f32> {
        self.sphere.intersect(r)
    }
}

impl Renderable for NormalSphere {
    fn material_color(&self, _: &Ray, p: &Triple) -> RGBA {
        let normal = self.sphere.normal(p);
        (
            (normal.x + 1.0) / 2.0,
            (normal.y + 1.0) / 2.0,
            (normal.z + 1.0) / 2.0,
            1.0,
        )
    }
    fn normal(&self, p: &Triple) -> Triple {
        self.sphere.normal(p)
    }
}

impl Drawable for NormalSphere {}
