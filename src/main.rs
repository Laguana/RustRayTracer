mod base;
mod objects;
mod scene;
mod shapes;

use base::light;
use base::camera::Camera;
use base::ray::Triple;
use base::quaternion::rotate;
use shapes::plane::Plane;
use shapes::plane::PlaneSegment;
use shapes::sphere::Sphere;

extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Instant;

fn main() {

    let (width, height): (u32, u32) = (400, 400);

    let mut camera = Camera::new(Triple {
        x: 0.0,
        y: 0.0,
        z: -20.0,
    }, Triple {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    }, Triple {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    },
        std::f32::consts::PI / 4.0,
        width, height);

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ray tracer", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

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

    scene.set_ambient_light((0.1,0.1,0.1,1.0).into());

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

    scene.add_object(Box::new(objects::normal_sphere::NormalSphere::new(
        Sphere::new(
            Triple {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            0.5,
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
                (0.0, 0.0, 0.0, 1.0).into()
            }
        }),
    )));

    scene.add_object(Box::new(objects::colored_plane::ColoredPlane::new(
        PlaneSegment::new(
            Plane {
                normal: Triple {
                    x: 0.0,
                    y: 0.1,
                    z: -1.0,
                }.unit_vector(),
                reference: Triple {
                    x: -2.5,
                    y: -2.5,
                    z: 4.0,
                },
            },
            Triple {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Triple {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            5.0,
            5.0,
        ),
        Box::new(|u, v| {
            (10.0*u, 10.0*v, 0.0, 1.0).into()
        }),
    )));

    let mut texture = texture_creator.create_texture_streaming(None, width, height).unwrap();

    let mut now = Instant::now();
    'running: loop {
        texture.with_lock(None, |buf, pitch|  {
            for y_idx in 0..height {
                let row_offset = y_idx as usize * pitch ;
                for x_idx in 0..width {
                    let offset = row_offset + x_idx as usize * 4;

                    let r = camera.pixel_ray(x_idx, y_idx);

                    let color: sdl3::pixels::Color = scene.get_color(&r).into();

                    // HACK: empirically this works right now, really it should
                    // be done based on the pixel format.
                    buf[offset] = color.b;
                    buf[offset+1] = color.g;
                    buf[offset+2] = color.r;
                    buf[offset+3] = color.a;
                }
            }
        }).unwrap();

        canvas.copy(&texture,  None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    camera.origin = camera.origin + camera.right * 0.1;
                },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                    camera.origin = camera.origin - camera.right * 0.1;
                },
                Event::KeyDown { keycode: Some(Keycode::W), ..} => {
                    camera.origin = camera.origin - camera.up * 0.1;
                },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    camera.origin = camera.origin + camera.up * 0.1
                },
                Event::KeyDown { keycode: Some(Keycode::Z), ..} => {
                    camera.origin = camera.origin + camera.direction * 0.1;
                },
                Event::KeyDown { keycode: Some(Keycode::X), ..} => {
                    camera.origin = camera.origin - camera.direction * 0.1;
                },
                Event::KeyDown { keycode: Some(Keycode::Q), ..} => {
                    camera.direction = rotate(camera.direction, camera.up, -0.01);
                    camera.right = camera.direction.cross_prod(camera.up).unit_vector();

                },
                Event::KeyDown { keycode: Some(Keycode::E), ..} => {
                    camera.direction = rotate(camera.direction, camera.up, 0.01);
                    camera.right = camera.direction.cross_prod(camera.up).unit_vector();
                },
                
                _ => {}
            }
        }

        println!("{}", now.elapsed().as_millis());
        now = Instant::now();
    }

}
