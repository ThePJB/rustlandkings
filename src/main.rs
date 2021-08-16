mod game;
mod rect;
mod collision;
mod entity;
mod screen_transform;
mod grid;

use screen_transform::ScreenTransform;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use game::GameState;
use std::time::{Duration, SystemTime};

fn main() {
    let xres = 800;
    let yres = 600;
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

    let mut state = GameState::new(ScreenTransform::new(xres, yres));
    let mut dt = 1.0f64 / 60f64;

    'running: loop {
        let loop_start = SystemTime::now();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                }
                Event::KeyDown {keycode: Some(Keycode::R), ..} => {
                    println!("===== reset =====");
                    state = GameState::new(ScreenTransform::new(xres, yres));
                }
                _ => {state.handle_input(event)}
            }
        }
        
        canvas.set_draw_color(Color::RGB(200, 200, 255));
        canvas.clear();
        
        state.update_held_keys(&event_pump.keyboard_state());
        state.update(dt);
        state.update_camera(&event_pump.mouse_state());
        state.draw_terrain(&mut canvas);
        state.draw_entities(&mut canvas, xres, yres);

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
