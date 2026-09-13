mod lib;
mod objects;
mod scene;
mod shapes;

use lib::light;
use lib::ray::Ray;
use lib::ray::Triple;
use shapes::plane::Plane;
use shapes::plane::PlaneSegment;
use shapes::sphere::Sphere;

extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::rect::Point;
use std::time::Instant;

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

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ray tracer", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();


    let mut scene = scene::Scene::new();
    scene.set_skybox(Box::new(|r| {
        (
            (r.direction.x + 1.0) / 2.0,
            (r.direction.y + 1.0) / 2.0,
            (r.direction.z + 1.0) / 2.0,
            1.0,
        ).into()
    }));
    
    scene.add_directional_light(light::UniformLight {
        color: (0.0, 1.0, 0.0, 1.0).into(),
        direction: Triple {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
    });
    
    scene.add_point_light(light::PointLight {
        color: (1.0, 0.0, 0.0, 1.0).into(),
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
                (1.0, 1.0, 1.0, 1.0).into()
            } else {
                (0.0, 0.0, 0.0, 0.0).into()
            }
        }),
    )));

    let x_span = x_max - x_min;
    let y_span = y_max - y_min;

    let mut fps = 0;
    let mut now = Instant::now();
    'running: loop {
        for x_idx in 0..width {
            for y_idx in 0..height {

                let x = (x_idx as f32 / width as f32) * x_span + x_min;
                let y = (y_idx as f32 / height as f32) * y_span + y_min;
                let z = 0.0;
                let target = Triple { x, y, z };
                let direction = target.vec_sub(&ray_origin).unit_vector();
                let r = Ray {
                    origin: ray_origin,
                    direction,
                };
                //println!("{:?}", r);
                
                canvas.set_draw_color(scene.get_color(&r));
                canvas.draw_point(Point::new(x_idx as i32, y_idx as i32)).unwrap();
            }
        }

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        fps += 1;
        if now.elapsed().as_secs() >= 1 {
            now = Instant::now();
            println!("{}", fps);
            fps = 0;
        }
    }

}
