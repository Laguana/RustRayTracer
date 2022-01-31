use crate::lib::ray::Triple;
use crate::lib::tracable::Tracable;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Triple,
    pub reference: Triple,
}

impl Tracable for Plane {
    fn intersect(&self, ray: &crate::lib::ray::Ray) -> std::vec::Vec<f32> {
        let denom = self.normal.dot_prod(&ray.direction);
        // denom represents how far along the plane normal the ray travels per unit
        // distance along the ray direction.
        if denom >= -1e-10 {
            // If denom == 0 then strictly speaking if the ray origin lies on the plane then
            // it intersects everywhere, but I'll just go ahead and make it not intersect
            // If denom > 0 then the ray is pointing in the same direction as the plane normal,
            // and so I'm going to go ahead and say that planes are 1 sided and thus no intersection
            // finally, if it's pretty close to 0 then we'll avoid blowing things up and fudge it a little
            return vec![];
        }
        let normal_distance = self.reference.vec_sub(&ray.origin).dot_prod(&self.normal);
        // normal_distance represents how far the plane is away from the ray origin
        // in terms of the plane normal
        let ray_distance = normal_distance / denom;
        // ray_distance is "how far along the ray direction do you need to go
        // in order to travel normal_distance"

        return vec![ray_distance];
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlaneSegment {
    plane: Plane,
    u_vector: Triple,
    v_vector: Triple,
    u_width: f32,
    v_height: f32,
}

impl PlaneSegment {
    fn new(
        plane: Plane,
        u_vec: Triple,
        v_vec: Triple,
        u_width: f32,
        v_height: f32,
    ) -> PlaneSegment {
        // ensure u_vec and v_vec are unit vectors along the plane; i.e. perpendicular to the plane
        let u_norm_component = u_vec.dot_prod(&plane.normal);
        let v_norm_component = v_vec.dot_prod(&plane.normal);
        if u_norm_component.abs() < 1e-10 || v_norm_component.abs() < 1e-10 {
            panic!("Found a u/v vector pointing mostly along the normal!");
        }
        let u_vector = u_vec
            .vec_sub(&plane.normal.scale(u_norm_component))
            .unit_vector();
        let v_vector = v_vec
            .vec_sub(&plane.normal.scale(v_norm_component))
            .unit_vector();

        PlaneSegment {
            plane,
            u_vector,
            v_vector,
            u_width,
            v_height,
        }
    }

    fn uv_coords(&self, point: &Triple) -> (f32, f32) {
        let delta = point.vec_sub(&self.plane.reference);
        let u_component = delta.dot_prod(&self.u_vector);
        let v_component = delta.dot_prod(&self.v_vector);
        (u_component, v_component)
    }
}

impl Tracable for PlaneSegment {
    fn intersect(&self, ray: &crate::lib::ray::Ray) -> std::vec::Vec<f32> {
        let potential_intersect = self.plane.intersect(ray);
        match potential_intersect.get(0) {
            None => potential_intersect,
            Some(t) => {
                let point = ray.origin.vec_add(&ray.direction.scale(*t));
                let (u_component, v_component) = self.uv_coords(&point);
                if u_component > 0.0
                    && u_component < self.u_width
                    && v_component > 0.0
                    && v_component < self.v_height
                {
                    potential_intersect
                } else {
                    vec![]
                }
            }
        }
    }
}
