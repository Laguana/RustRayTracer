use crate::lib::tracable::Drawable;
use crate::lib::tracable::Renderable;
use crate::lib::tracable::Tracable;
use crate::shapes::sphere::Sphere;

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
    fn intersect(&self, r: &crate::lib::ray::Ray) -> std::vec::Vec<f32> {
        self.sphere.intersect(r)
    }
}

impl Renderable for NormalSphere {
    fn color(&self, _: &crate::lib::ray::Ray, p: &crate::lib::ray::Triple) -> (u8, u8, u8, u8) {
        let normal = self.sphere.normal(p);
        (
            128+(normal.x * 127.0).trunc() as u8,
            128+(normal.y * 127.0).trunc() as u8,
            128+(normal.z * 127.0).trunc() as u8,
            1,
        )
    }
}

impl Drawable for NormalSphere {}
