use crate::lib::light;
use crate::lib::ray;
use crate::lib::tracable;
use crate::lib::color;

//#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<dyn tracable::Drawable>>,
    skybox: Box<dyn Fn(&ray::Ray) -> color::RGBA>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: vec![], skybox: Box::new(|_| (0,0,0,0))}
    }

    pub fn add_object(&mut self, object: Box<dyn tracable::Drawable>) {
        self.objects.push(object)
    }

    pub fn set_skybox(&mut self, skybox: Box<dyn Fn(&ray::Ray) -> color::RGBA>) {
        self.skybox = skybox;
    }

    pub fn cast_ray(&self, ray: &ray::Ray) -> Option<(f32, &Box<dyn tracable::Drawable>)> {
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

    pub fn get_color(&self, ray: &ray::Ray) -> color::RGBA {
        match self.cast_ray(ray) {
            None => (self.skybox)(ray),
            Some((distance, obj)) => {
                //println!("{:?}@{}", obj, distance);
                let point = ray.origin.vec_add(&ray.direction.scale(distance));
                obj.color(ray, &point)
            }
        }
    }
}
