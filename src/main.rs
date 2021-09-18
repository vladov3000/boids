use std::convert::TryInto;
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::ops::Rem;
use std::time::{Duration, Instant};

use rand::Rng;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

use boids::boids::{Boid, BoidController};
use boids::rule::Rule;
use boids::rule::default_dir_fns::{alignment, separation, cohesion};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG);
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let window = video_subsystem.window("boids", 600, 400)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_logical_size(600, 400);
    let texture_creator = canvas.texture_creator();
    let boid_texture = texture_creator.load_texture("assets/boid_minimal.png").unwrap();

    let target_fps = 30;
    let time_per_frame = Duration::from_millis(1000 / target_fps);
    let mut timer = Instant::now();

    let mut rng = rand::thread_rng();
    let mut boids = BoidController {
        boids: Vec::new(),
        boid_texture: &boid_texture,
        rules: vec![
            Rule { dir_fn: separation, weight: 1. },
            Rule { dir_fn: alignment, weight: 1. },
            Rule { dir_fn: cohesion, weight: 1. },
        ],
        boid_speed: 10.,
        boid_turn_resistance: 5.,
        nearby_range: 32,
    };

    'running: loop {
        let boids_mut_ref = &mut boids;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    boids_mut_ref.boids.push(Boid {
                        rect: Rect::from_center((x, y), 16, 32),
                        angle: rng.gen_range(0.0..TAU),
                    });
                }
                _ => {}
            }
        }

        while timer.elapsed() > time_per_frame {
            // fps independent logic goes here

            boids_mut_ref.update(canvas.logical_size());

            timer += time_per_frame;
        }
        timer += timer.elapsed();

        canvas.clear();

        boids.render(&mut canvas);

        canvas.present();
        std::thread::sleep(
            time_per_frame.checked_sub(timer.elapsed())
                .unwrap_or(Duration::new(0, 0))
        );
    }
}
