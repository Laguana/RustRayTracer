use crate::lib::color::RGBA;
use crate::lib::tracable::Drawable;
use crate::lib::tracable::Renderable;
use crate::lib::tracable::Tracable;
use crate::shapes::plane::PlaneSegment;

pub struct ColoredPlane {
    geometry: PlaneSegment,
    uv_mapped_color: Box<dyn Fn(f32, f32) -> RGBA>,
}

impl ColoredPlane {
    pub fn new(plane: PlaneSegment, uv_mapped_color: Box<dyn Fn(f32, f32) -> RGBA>) -> ColoredPlane {
        ColoredPlane{geometry: plane, uv_mapped_color}
    }
}

impl Tracable for ColoredPlane {
    fn intersect(&self, r: &crate::lib::ray::Ray) -> std::vec::Vec<f32> {
        self.geometry.intersect(r)
    }
}

impl Renderable for ColoredPlane {
    fn color(&self, _: &crate::lib::ray::Ray, p: &crate::lib::ray::Triple) -> (u8, u8, u8, u8) {
        let (u, v) = self.geometry.uv_coords(p);
        (self.uv_mapped_color)(u, v)
    }
}

impl Drawable for ColoredPlane {}