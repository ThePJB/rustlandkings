use crate::rect::*;
use crate::entity::*;
use crate::screen_transform::*;
use crate::grid::*;
use crate::vec2::*;
use crate::side_effect::*;
use crate::simulation_state::*;
use crate::systems::command::*;
use crate::systems::collision::*;
use crate::systems::projectiles::*;
use crate::systems::ai::*;

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseState;
use std::collections::HashMap;
use rand::Rng;

static movement_speed: f32 = 0.5;

pub struct Game {
    pause: bool,
    
    frame_commands: Vec<Command>,
    frame_collisions: Vec<CollisionEvent>,
    frame_movements: Vec<(u32, f32, f32)>,
    frame_side_effects: Vec<SideEffect>,

    player_id: u32,
    aim_pos: Vec2,
    transform: ScreenTransform,

    state: SimulationState,
}

impl Game {
    pub fn new(transform: ScreenTransform) -> Game {
        let mut game = Game { 
            pause: false, 
            frame_commands: Vec::new(),
            frame_collisions: Vec::new(), 
            frame_movements: Vec::new(), 
            frame_side_effects: Vec::new(), 
            player_id: 0,
            aim_pos: Vec2::zero(),
            transform: transform, 
            state: SimulationState::new()
        };

        game.initialize();

        return game;
    }

    pub fn initialize(&mut self) {
        for (entity_id, entity) in self.state.entities.iter() {
            if entity.variety == EntityType::Player {
                self.player_id = *entity_id;
            }
        }
    }

    pub fn clear_arenas(&mut self) {
        self.frame_commands.clear();
        self.frame_collisions.clear();
        self.frame_movements.clear();
        self.frame_side_effects.clear();
    }

    pub fn handle_input(&mut self, event_pump: &mut EventPump) -> bool {
        
        // Handle keys held
        // player walking
        let keys = event_pump.keyboard_state();
        let mut walk_dir = Vec2::zero();
        if keys.is_scancode_pressed(Scancode::A) {
            walk_dir.x = -1.0;
        } else if keys.is_scancode_pressed(Scancode::D) {
            walk_dir.x = 1.0;
        }
        if keys.is_scancode_pressed(Scancode::W) {
            walk_dir.y = -1.0;
        } else if keys.is_scancode_pressed(Scancode::S) {
            walk_dir.y = 1.0;
        }
        if walk_dir != Vec2::zero() {
            walk_dir = walk_dir.normalize();
        }
        self.frame_commands.push(Command::Walk(self.player_id, walk_dir));
        
        // Handle mouse position
        let mouse = event_pump.mouse_state();

        self.aim_pos = self.transform.pick_world(mouse.x() as u32, mouse.y() as u32);
        if let Some(player_ent) = self.state.entities.get(&self.player_id) {
            self.transform.translate_center(self.aim_pos.add(player_ent.aabb.center()).div_scalar(2.0));
        }

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    return false;
                }
                Event::MouseButtonDown{x, y, ..} => {
                    let world_click = self.transform.pick_world(x as u32, y as u32);
                    self.frame_commands.push(Command::Shoot(self.player_id, world_click));
                },
                Event::KeyDown{keycode: Some(Keycode::P), ..} => {
                    self.pause = !self.pause;
                },
                Event::KeyDown{keycode: Some(Keycode::R), ..} => {
                    println!("===== reset =====");
                    self.state = SimulationState::new();
                    self.initialize();
                },
                _ => {},
            }
        }

        return true;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        
        // draw terrain
        for (i, t) in self.state.terrain.tiles.iter().enumerate() {
            canvas.set_draw_color(Color::RGB(255,255,255));
            let r = self.state.terrain.get_rect_1d(i as i32);
            let screenspace_rect = self.transform.project_rect(r);
            let px_rect = self.transform.sdl_rect(screenspace_rect);
            
            canvas.fill_rect(px_rect).unwrap();
            
            let inner_rect = screenspace_rect.dilate(-0.004);
            canvas.set_draw_color(match t {
                &Tile::Ground => {Color::RGB(200, 200, 100)},
                &Tile::Wall => {Color::RGB(50, 50, 100)},
            });

            canvas.fill_rect(self.transform.sdl_rect(inner_rect)).unwrap();
        }

        // draw entities
        let mut draw_entity = |entity: &Entity, a: f32| {
            canvas.set_draw_color(entity.colour);
            let screenspace_rect = self.transform.project_rect(entity.aabb);
            let px_rect = self.transform.sdl_rect(screenspace_rect);

            canvas.fill_rect(px_rect).unwrap();
        };

        self.state.entities.iter().filter(|(_, entity)| entity.draw_order == DrawOrder::Back).for_each(|(_, entity)| draw_entity(entity, self.transform.px.0 as f32/self.transform.px.1 as f32));
        self.state.entities.iter().filter(|(_, entity)| entity.draw_order == DrawOrder::Front).for_each(|(_, entity)| draw_entity(entity, self.transform.px.0 as f32/self.transform.px.1 as f32));
    }

    pub fn update(&mut self, dt: f64) {
        if self.pause { return; }

        compute_ai_commands(&self.state, &mut self.frame_commands);

        for command in self.frame_commands.iter() {
            apply_command(&mut self.state, *command);
        }

        self.state.time += dt;

        simulate_entity_entity_collisions(&self.state.entities, &mut self.frame_collisions, dt as f32);
        simulate_entity_terrain_collisions(&self.state.entities, &self.state.terrain, &mut self.frame_collisions, dt as f32);
        for col in self.frame_collisions.iter() {
            if col.subject == self.player_id {
                match col.object {
                    CollisionObject::Entity(key) => {},
                    CollisionObject::Terrain(x, y) => {println!("{:.2}: player collision with tile ({}, {}) which is of type {:?}", self.state.time, x, y, self.state.terrain.get_2d(x, y).unwrap());},
                }
            }
        }
        compute_movement(&self.state.entities, &self.frame_collisions, &mut self.frame_movements, dt as f32);

        // apply movements: a bit oldschool and maybe silly
        for (entity_id, dx, dy) in self.frame_movements.iter() {
            let e = self.state.entities.get_mut(&entity_id).unwrap();
            e.aabb.x += dx;
            e.aabb.y += dy;
        }

        handle_bullet_impacts(&self.state, &self.frame_collisions, &mut self.frame_side_effects);

        for effect in self.frame_side_effects.iter() {
            self.state.resolve_side_effect(*effect);
        }

        self.state.entities.retain(|_, e| e.health > 0.0);
    }
}