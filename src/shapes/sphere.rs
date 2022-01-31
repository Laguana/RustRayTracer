use crate::lib::ray::Triple;
use crate::lib::tracable::Tracable;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Triple,
    radius: f32,
}

impl Tracable for Sphere {
    fn intersect(&self, ray: &crate::lib::ray::Ray) -> std::vec::Vec<f32> {
        // solving for |(P-C)| = r
        // (P-C).(P-C) = r^2
        // P = ray.origin + t*ray.direction
        // r^2 = dot(ray.origin + t*ray.direction - C, ray.origin + t*ray.direction -C)
        // r^2 = dot(ray.origin-C + t*ray.direction, ray.origin-C + t*ray.direction)
        // r^2 = t^2 * dot(ray.direction, ray.direction) + 2*t*dot(ray.origin-C, ray.direction) + dot(ray.origin-C, ray.origin-C)
        // 0   = t^2 + 2*t*dot(offset, ray.direction) + dot(offset, offset) - r^2
        // solutions are  t = (-b +- sqrt(b*b - 4*a*c))/2a
        let offset = ray.origin.vec_sub(&self.center);
        let c = offset.dot_prod(&offset) - self.radius * self.radius;
        let b = 2.0 * offset.dot_prod(&ray.direction);
        let descriminant = b*b - 4.0 * c;
        if descriminant < 0.0 {
            // There is no intersection
            return vec![]
        }

        if descriminant == 0.0 {
            // There is a single intersection
            return vec![-b/2.0];
        }

        let sqrt = (descriminant).sqrt();

        return vec![(-b + sqrt)/2.0, (-b - sqrt)/2.0]

    }
}
