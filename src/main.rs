mod game;
mod rect;
mod systems;
mod entity;
mod screen_transform;
mod grid;
mod vec2;
mod side_effect;
mod simulation_state;

use crate::game::*;
use screen_transform::ScreenTransform;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::time::{Duration, SystemTime};

fn main() {
    let xres = 1280;
    let yres = 720;
    let a = xres as f32 / yres as f32;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rustland kings", xres, yres)
        .position_centered()
        .build()
        .expect("failed making window");

    let mut canvas = window.into_canvas().build()
        .expect("couldnt make canvas");

    let mut event_pump = sdl_context.event_pump().unwrap();

    let gravity = 3.5;
    let cam_vx = 0.4;

    let mut game = Game::new(ScreenTransform::new(xres, yres));
    let mut dt = 1.0f64 / 60f64;

    'running: loop {
        let loop_start = SystemTime::now();

        game.clear_arenas();
        if !game.handle_input(&mut event_pump) {
            break 'running;
        }
        
        canvas.set_draw_color(Color::RGB(200, 200, 255));
        canvas.clear();
        
        game.update(dt);
        game.draw(&mut canvas);

        canvas.present();

        let loop_end = SystemTime::now();
        let delta = loop_end.duration_since(loop_start).unwrap().as_secs_f64();
        let frame_cap = 1.0 / 60.0;
        if delta < frame_cap {
            std::thread::sleep(Duration::from_secs_f64(frame_cap - delta));
            dt = frame_cap;
        } else {
            dt = delta;
        }
        //println!("{} fps", 1.0/dt);
    }
}
