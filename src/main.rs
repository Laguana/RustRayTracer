mod lib;
mod objects;
mod scene;
mod shapes;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use lib::light;
use lib::ray::Ray;
use lib::ray::Triple;
use shapes::plane::Plane;
use shapes::plane::PlaneSegment;
use shapes::sphere::Sphere;

fn main() {
    println!("Hello, world!");

    let (width, height) = (400, 400);
    let (x_min, x_max) = (-1.5, 1.5);
    let (y_min, y_max) = (-1.5, 1.5);

    let ray_origin = Triple {
        x: 0.0,
        y: 0.0,
        z: -2.0,
    };

    let path = Path::new(r"out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    /*
    encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    */
    let mut writer = encoder.write_header().unwrap();

    let mut scene = scene::Scene::new();
    scene.set_skybox(Box::new(|r| {
        (
            (r.direction.x + 1.0) / 2.0,
            (r.direction.y + 1.0) / 2.0,
            (r.direction.z + 1.0) / 2.0,
            1.0,
        )
    }));
    
    scene.add_directional_light(light::UniformLight {
        color: (0.0, 1.0, 0.0, 1.0),
        direction: Triple {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
    });
    
    scene.add_point_light(light::PointLight {
        color: (1.0, 0.0, 0.0, 1.0),
        position: Triple {
            x: 0.75,
            y: -0.75,
            z: -0.75,
        },
    });
    scene.add_object(Box::new(objects::normal_sphere::NormalSphere::new(
        Sphere::new(
            Triple {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        ),
    )));

    scene.add_object(Box::new(objects::colored_plane::ColoredPlane::new(
        PlaneSegment::new(
            Plane {
                normal: Triple {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                reference: Triple {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                },
            },
            Triple {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Triple {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            2.0,
            2.0,
        ),
        Box::new(|u, v| {
            let x = (u * 10.0).trunc() as u8;
            let y = (v * 10.0).trunc() as u8;
            if (x + y) % 2 == 0 {
                (1.0, 1.0, 1.0, 1.0)
            } else {
                (0.0, 0.0, 0.0, 0.0)
            }
        }),
    )));

    let x_span = x_max - x_min;
    let y_span = y_max - y_min;

    let mut data: Vec<u8> = vec![0; (width * height * 3).try_into().unwrap()];
    for (i, chunk) in data.chunks_mut(3).enumerate() {
        let y_idx = 1.0 - (i / (width as usize)) as f32 / height as f32;
        let x_idx = (i % (width as usize)) as f32 / width as f32;
        let x = x_idx * x_span + x_min;
        let y = y_idx * y_span + y_min;
        let z = 0.0;
        let target = Triple { x, y, z };
        let direction = target.vec_sub(&ray_origin).unit_vector();
        let r = Ray {
            origin: ray_origin,
            direction,
        };
        //println!("{:?}", r);
        let (r, g, b, _) = scene.get_color(&r);
        chunk[0] = (r * 255.0).trunc() as u8;
        chunk[1] = (g * 255.0).trunc() as u8;
        chunk[2] = (b * 255.0).trunc() as u8;
    }

    writer.write_image_data(data.as_slice()).unwrap()
}
