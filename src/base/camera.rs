use crate::base::ray::Triple;
use crate::base::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Triple,     // Origin of the camera
    pub direction: Triple,  // Direction the camera is pointing, magnitude is meaningful as origin+direction is where the 'focal plane' is
    pub up: Triple,         // This should be perpendicular to the direction, and defines the y-up direction in screen space. Magnitude is meaningful; pixels go from direction to direction+up
    pub right: Triple,      // This should be perpendicular to direction and up, and defines x-positive direction. Magnitude is meaningful; pixels go from diretion to direction+right
    pub width: u32,       // camera pixel resolution
    pub height: u32,      // 
}

impl Camera {
    pub fn new(origin: Triple, direction: Triple, up: Triple, right: Triple, width: u32, height: u32) -> Camera {
        return Camera { origin, direction, up, right, width, height }
    }

    pub fn pixel_ray(&self, x: u32, y: u32) -> Ray {
        assert!(x < self.width);
        assert!(y < self.height);
        let dx = x as f32 / self.width as f32;
        let dy = y as f32 / self.height as f32;
        Ray { origin: self.origin, direction: (self.direction + self.up * dy + self.right * dx).unit_vector() }
    }
}