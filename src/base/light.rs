use crate::base::ray::Triple;
use crate::base::color::RGBA;

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub color: RGBA,
    pub position: Triple,
}

#[derive(Debug, Clone, Copy)]
pub struct UniformLight {
    pub color: RGBA,
    pub direction: Triple,
}