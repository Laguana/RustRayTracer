use crate::base::ray::Triple;
use crate::base::ray::Ray;
use crate::base::quaternion::rotate;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Triple,     // Origin of the camera
    pub direction: Triple,  // Direction the camera is pointing, should be unit vector
    pub up: Triple,         // This should be perpendicular to the direction, and defines the y-up direction in screen space
    pub right: Triple,      // The right vector, typically computed from direction and up
    pub fov: f32,           // field of view in radians
    pub width: u32,         // camera pixel resolution
    pub height: u32,        //x
}

// If we want a fov of a given angle phi, then
// if the direction points to the center, we want to go phi/2 to the left and to the right
// we can do this with the quaternion rotation

impl Camera {
    pub fn new(origin: Triple, direction: Triple, up: Triple, fov: f32, width: u32, height: u32) -> Camera {
        let right = direction.cross_prod(up);
        return Camera { origin, direction, up, right, fov, width, height }
    }

    pub fn pixel_ray(&self, x: u32, y: u32) -> Ray {
        assert!(x < self.width);
        assert!(y < self.height);
        let dx = x as f32 / self.width as f32 - 0.5;
        let dy = y as f32 / self.height as f32 - 0.5;
        let direction = rotate(rotate(self.direction, self.up, dx * self.fov / 2.0), self.right, dy * self.fov / 2.0).unit_vector();
        Ray { origin: self.origin, direction: direction }
    }
}