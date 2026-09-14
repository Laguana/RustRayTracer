use crate::base::color::RGBA;
use crate::base::ray::Ray;
use crate::base::ray::Triple;
use crate::base::tracable::Drawable;
use crate::base::tracable::Renderable;
use crate::base::tracable::Tracable;

use crate::shapes::plane::PlaneSegment;

pub struct ColoredPlane {
    geometry: PlaneSegment,
    uv_mapped_color: Box<dyn Fn(f32, f32) -> RGBA>,
}

impl ColoredPlane {
    pub fn new(
        plane: PlaneSegment,
        uv_mapped_color: Box<dyn Fn(f32, f32) -> RGBA>,
    ) -> ColoredPlane {
        ColoredPlane {
            geometry: plane,
            uv_mapped_color,
        }
    }
}

impl Tracable for ColoredPlane {
    fn intersect(&self, r: &Ray) -> std::vec::Vec<f32> {
        self.geometry.intersect(r)
    }
}

impl Renderable for ColoredPlane {
    fn material_color(
        &self,
        _: &Ray,
        p: &Triple,
    ) -> RGBA {
        let (u, v) = self.geometry.uv_coords(p);
        (self.uv_mapped_color)(u/self.geometry.u_width, v/self.geometry.v_height)
    }

    fn normal(&self, _: &Triple) -> Triple {
        self.geometry.plane.normal
    }
}

impl Drawable for ColoredPlane {}
