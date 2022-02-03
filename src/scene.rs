use crate::lib::color;
use crate::lib::light;
use crate::lib::ray::Ray;
use crate::lib::ray::Triple;
use crate::lib::tracable;

//#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<dyn tracable::Drawable>>,
    skybox: Box<dyn Fn(&Ray) -> color::RGBA>,
    point_lights: Vec<light::PointLight>,
    directional_lights: Vec<light::UniformLight>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: vec![],
            skybox: Box::new(|_| (0.0, 0.0, 0.0, 0.0)),
            point_lights: vec![],
            directional_lights: vec![],
        }
    }

    pub fn add_object(&mut self, object: Box<dyn tracable::Drawable>) {
        self.objects.push(object)
    }

    pub fn add_point_light(&mut self, light: light::PointLight) {
        self.point_lights.push(light)
    }

    pub fn add_directional_light(&mut self, light: light::UniformLight) {
        self.directional_lights.push(light)
    }

    pub fn set_skybox(&mut self, skybox: Box<dyn Fn(&Ray) -> color::RGBA>) {
        self.skybox = skybox;
    }

    pub fn cast_ray(&self, ray: &Ray) -> Option<(f32, &Box<dyn tracable::Drawable>)> {
        self.objects.iter().fold(None, |candidate, o| {
            let mut min_t = f32::INFINITY;
            for t in o.intersect(ray) {
                if t >= 0.0 && t < min_t {
                    min_t = t;
                }
            }
            if min_t == f32::INFINITY {
                return candidate;
            } else {
                match candidate {
                    None => Some((min_t, o)),
                    Some((prev_t, _)) => {
                        if min_t < prev_t {
                            Some((min_t, o))
                        } else {
                            candidate
                        }
                    }
                }
            }
        })
    }

    pub fn get_color(&self, ray: &Ray) -> color::RGBA {
        match self.cast_ray(ray) {
            None => (self.skybox)(ray),
            Some((distance, obj)) => {
                //println!("{:?}@{}", obj, distance);
                let point = ray.origin.vec_add(&ray.direction.scale(distance));
                let material_color = obj.material_color(ray, &point);
                let normal = obj.normal(&point);

                self.get_diffuse(&point, &normal, material_color)
            }
        }
    }

    fn get_diffuse(&self, point: &Triple, normal: &Triple, (mr,mg,mb,_): color::RGBA) -> color::RGBA {
        // ambient
        let (r,g,b,a) = (mr * 0.1, mg * 0.1, mb * 0.1, 1.0);
        
        // directional lights
        let (r,g,b,a) = self.directional_lights.iter().filter(|l| {
            match self.cast_ray(&Ray {
                origin: point.vec_sub(&l.direction.scale(0.01)),
                direction: l.direction.scale(-1.0)
            }) {
                None => true,
                Some(_) => false,
            }
        }).fold((r,g,b,a), |(r,g,b,a),l| {
            let diffuse = (0.0f32).max(normal.dot_prod(&l.direction.scale(-1.0)));
            let (lr,lg, lb, _) = l.color;
            let (dr, dg, db) = (diffuse * lr * mr, diffuse * lg * mg, diffuse * lb * mb);

            (r+dr,g+dg,b+db,a)
        });
        // positional lights
        let (r,g,b,a) = self.point_lights.iter().filter(|l| {
            let direction = l.position.vec_sub(point).unit_vector();
            match self.cast_ray(&Ray {
                origin: point.vec_add(&direction.scale(0.01)),
                direction: direction
            }) {
                None => true,
                Some(_) => false,
            }
        }).fold((r,g,b,a), |(r,g,b,a),l| {
            let direction = l.position.vec_sub(point).unit_vector();
            let diffuse = (0.0f32).max(normal.dot_prod(&direction));
            let (lr,lg, lb, _) = l.color;
            let (dr, dg, db) = (diffuse * lr * mr, diffuse * lg * mg, diffuse * lb * mb);

            (r+dr,g+dg,b+db,a)
        });

        (r,g,b,a)
    }
}
