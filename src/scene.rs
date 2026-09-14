use crate::base::color;
use crate::base::light;
use crate::base::ray::Ray;
use crate::base::ray::Triple;
use crate::base::tracable;

//#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<dyn tracable::Drawable>>,
    skybox: Box<dyn Fn(&Ray) -> color::RGBA>,
    ambient_light: color::RGBA,
    point_lights: Vec<light::PointLight>,
    directional_lights: Vec<light::UniformLight>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: vec![],
            skybox: Box::new(|_| (0.0, 0.0, 0.0, 0.0).into()),
            ambient_light: (0.1, 0.1, 0.1, 1.0).into(),
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

    pub fn set_ambient_light(&mut self, light: color::RGBA) {
        self.ambient_light = light;
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
                let point = ray.origin + ray.direction.scale(distance);
                let material_color = obj.material_color(ray, &point);
                let normal = obj.normal(&point);

                self.get_diffuse(&point, &normal, material_color)
            }
        }
    }

    fn get_diffuse(
        &self,
        point: &Triple,
        normal: &Triple,
        mat: color::RGBA,
    ) -> color::RGBA {
        // ambient
        let base_diffuse = self.ambient_light * mat;
        // directional lights
        let directional = self
            .directional_lights
            .iter()
            .filter(|l| {
                match self.cast_ray(&Ray {
                    origin: point.vec_sub(&l.direction.scale(0.01)),
                    direction: l.direction.scale(-1.0),
                }) {
                    None => true,
                    Some(_) => false,
                }
            })
            .fold(base_diffuse, |c, l| {
                let diffuse = (0.0f32).max(normal.dot_prod(&l.direction.scale(-1.0)));
                let mut light_contribution = l.color * mat * diffuse;
                light_contribution.a = 0.0;

                c + light_contribution
            });
        // positional lights
        let positional = self
            .point_lights
            .iter()
            .fold(directional, |c, l| {
                let delta = l.position.vec_sub(point);
                let distance_squared = delta.dot_prod(&delta);
                let direction = delta.unit_vector();
                let origin = point + &direction.scale(0.01) ;
                let visible = match self.cast_ray(&Ray { origin, direction }) {
                    None => true,
                    Some((t, _)) => distance_squared < (t + 0.01) * (t + 0.01),
                };
                if visible {
                    let diffuse = (0.0f32).max(normal.dot_prod(&direction));
                    let diffuse = diffuse / (distance_squared.max(1.0));
                    let mut light_contribution = l.color * mat * diffuse;
                    light_contribution.a = 0.0;

                    c + light_contribution
                } else {
                    c
                }
            });

        positional
    }
}
