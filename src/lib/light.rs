use crate::lib::ray::Triple;
use crate::lib::color::RGBA;

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    color: RGBA,
    position: Triple,
}

#[derive(Debug, Clone, Copy)]
pub struct UniformLight {
    color: RGBA,
    direction: Triple,
}