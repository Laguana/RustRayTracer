use crate::base::ray::Ray;
use crate::base::ray::Triple;
use crate::base::tracable::Drawable;
use crate::base::tracable::Renderable;
use crate::base::tracable::Tracable;
use crate::base::tracable::Material;
use crate::shapes::sphere::Sphere;

#[derive(Debug)]
pub struct NormalSphere {
    sphere: Sphere,
    reflectivity: f32,
}

impl NormalSphere {
    pub fn new(sphere: Sphere, reflectivity: f32) -> NormalSphere {
        NormalSphere { sphere, reflectivity }
    }
}

impl Tracable for NormalSphere {
    fn intersect(&self, r: &Ray) -> std::vec::Vec<f32> {
        self.sphere.intersect(r)
    }
}

impl Renderable for NormalSphere {
    fn material(&self, p: &Triple) -> Material {
        let normal = self.sphere.normal(p);
        Material {
            color: (
                (normal.x + 1.0) / 2.0,
                (normal.y + 1.0) / 2.0,
                (normal.z + 1.0) / 2.0,
                1.0,
                ).into(),
            reflectivity: self.reflectivity,
            normal: self.sphere.normal(p)
        }

    }
}

impl Drawable for NormalSphere {}
