use crate::base::ray::Ray;
use crate::base::ray::Triple;
use crate::base::tracable::Drawable;
use crate::base::tracable::Renderable;
use crate::base::tracable::Tracable;
use crate::base::tracable::Material;

use crate::shapes::plane::PlaneSegment;

pub struct ColoredPlane {
    geometry: PlaneSegment,
    uv_mapped_material: Box<dyn Fn(f32, f32) -> Material>,
}

impl ColoredPlane {
    pub fn new(
        plane: PlaneSegment,
        uv_mapped_material: Box<dyn Fn(f32, f32) -> Material>,
    ) -> ColoredPlane {
        ColoredPlane {
            geometry: plane,
            uv_mapped_material,
        }
    }
}

impl Tracable for ColoredPlane {
    fn intersect(&self, r: &Ray) -> std::vec::Vec<f32> {
        self.geometry.intersect(r)
    }
}

impl Renderable for ColoredPlane {
    fn material(
        &self,
        p: &Triple,
    ) -> Material {
        let (u, v) = self.geometry.uv_coords(p);
        let Material { color, reflectivity, normal } = (self.uv_mapped_material)(u/self.geometry.u_width, v/self.geometry.v_height);
        Material { color, reflectivity, normal: (self.geometry.plane.normal + normal).unit_vector() }
    }
}

impl Drawable for ColoredPlane {}
