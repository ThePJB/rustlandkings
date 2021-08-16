use crate::rect::*;
use crate::collision::*;
use crate::entity::*;
use crate::screen_transform::*;
use crate::grid::*;

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseState;
use std::collections::HashMap;
use rand::Rng;

static movement_speed: f32 = 0.5;


pub struct GameState {
    entities: HashMap<u32, Entity>,
    player_id: u32,
    frame_collisions: Vec<CollisionEvent>,
    frame_movements: Vec<(u32, f32, f32)>,
    time: f64,
    pause: bool,
    dead: bool,

    transform: ScreenTransform,
    aim_pos: (f32, f32),

    terrain: Grid,

    selected_rect: (i32, i32),
}

impl GameState {
    pub fn new(transform: ScreenTransform) -> GameState {
        let mut state = GameState {
            entities: HashMap::new(),
            frame_collisions: Vec::new(),
            frame_movements: Vec::new(),
            player_id: 0,
            time: 0.0,
            pause: false,
            dead: false,

            transform: transform,
            aim_pos: (0.5, 0.5),
            terrain: GameState::generate_level(),

            selected_rect: (0, 0),
        };

        state.player_id = state.add_entity(Entity::new_player(transform.aspect_ratio()/2.0, 0.4));
        state.add_entity(Entity::new_platform(transform.aspect_ratio()/2.0 - 0.4, PlatformHeight::Middle));
    
        return state;
    }

    fn generate_level() -> Grid {
        let w = 10;
        let h = 10;
        let elem_s = 0.2;

        let mut g = Grid::new(w, h, elem_s, elem_s);
        for i in 0..h {
            g.set_2d(i, 0, Tile::Wall);
            g.set_2d(0, i, Tile::Wall);
            g.set_2d(i, h-1, Tile::Wall);
            g.set_2d(w-1, i, Tile::Wall);
        }
        return g;
    }

    pub fn draw_terrain(&self, canvas: &mut Canvas<Window>) {
        for (i, t) in self.terrain.tiles.iter().enumerate() {
            canvas.set_draw_color(Color::RGB(255,255,255));
            let r = self.terrain.get_rect_1d(i as i32);
            let screenspace_rect = self.transform.project_rect(r);
            let px_rect = self.transform.sdl_rect(screenspace_rect);
            
            canvas.fill_rect(px_rect).unwrap();
            
            let inner_rect = screenspace_rect.dilate(-0.004);
            canvas.set_draw_color(match t {
                &Tile::Ground => {Color::RGB(200, 200, 100)},
                &Tile::Wall => {Color::RGB(50, 50, 100)},
            });
            if self.selected_rect.0 == i as i32 % self.terrain.w && self.selected_rect.1 == i as i32 / self.terrain.h {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
            }
            canvas.fill_rect(self.transform.sdl_rect(inner_rect)).unwrap();
        }
    }

    pub fn draw_entities(&self, canvas: &mut Canvas<Window>, w: u32, h: u32) {
        let mut draw_entity = |entity: &Entity, a: f32| {
            canvas.set_draw_color(entity.colour);
            let screenspace_rect = self.transform.project_rect(entity.aabb);
            let px_rect = sdl2::rect::Rect::new(    
                (screenspace_rect.x / a * w as f32) as i32,
                (screenspace_rect.y * h as f32) as i32,
             (screenspace_rect.w / a * w as f32) as u32,
            (screenspace_rect.h * h as f32) as u32,
            );
            canvas.fill_rect(px_rect).unwrap();
        };

        self.entities.iter().filter(|(_, entity)| entity.draw_order == DrawOrder::Back).for_each(|(_, entity)| draw_entity(entity, w as f32/h as f32));
        self.entities.iter().filter(|(_, entity)| entity.draw_order == DrawOrder::Front).for_each(|(_, entity)| draw_entity(entity, w as f32/h as f32));
    }

    pub fn add_entity(&mut self, entity: Entity) -> u32 {
        let key = rand::thread_rng().gen();
        self.entities.insert(key, entity);
        return key;
    }

    pub fn update_held_keys(&mut self, keys: &KeyboardState) {
        if keys.is_scancode_pressed(Scancode::A) {
            self.entities.get_mut(&self.player_id).unwrap().vx = -movement_speed;
        } else if keys.is_scancode_pressed(Scancode::D) {
            self.entities.get_mut(&self.player_id).unwrap().vx = movement_speed;
        } else {
            self.entities.get_mut(&self.player_id).unwrap().vx = 0.0;
        }
        if keys.is_scancode_pressed(Scancode::W) {
            self.entities.get_mut(&self.player_id).unwrap().vy = -movement_speed;
        } else if keys.is_scancode_pressed(Scancode::S) {
            self.entities.get_mut(&self.player_id).unwrap().vy = movement_speed;
        } else {
            self.entities.get_mut(&self.player_id).unwrap().vy = 0.0;
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.pause {
            return;
        }

        self.time += dt;

        self.frame_collisions.clear();
        self.frame_movements.clear();

        simulate_entity_entity_collisions(&self.entities, &mut self.frame_collisions, dt as f32);
        simulate_entity_terrain_collisions(&self.entities, &self.terrain, &mut self.frame_collisions, dt as f32);
        for col in self.frame_collisions.iter() {
            if col.subject == self.player_id {
                match col.object {
                    CollisionObject::Entity(key) => {},
                    CollisionObject::Terrain(x, y) => {println!("{:.2}: player collision with tile ({}, {}) which is of type {:?}", self.time, x, y, self.terrain.get_2d(x, y).unwrap());},
                }
            }
        }
        compute_movement(&self.entities, &self.frame_collisions, &mut self.frame_movements, dt as f32);
        GameState::apply_movement(&mut self.entities, &self.frame_movements);
    }

    pub fn apply_movement(entities: &mut HashMap<u32, Entity>, movements: &Vec<(u32, f32, f32)>) {
        for (entity_id, dx, dy) in movements {
            let e = entities.get_mut(entity_id).unwrap();
            e.aabb.x += dx;
            e.aabb.y += dy;
        }
    }

    pub fn update_camera(&mut self, mouse: &MouseState) {
        self.aim_pos = self.transform.pick_world(mouse.x() as u32, mouse.y() as u32);
        let player_ent = self.entities.get(&self.player_id).unwrap();
        self.transform.translate_center((self.aim_pos.0 + player_ent.aabb.x)/2.0, (self.aim_pos.1 + player_ent.aabb.y)/2.0);
    }

    pub fn handle_input(&mut self, e: Event) {
        match e {
            Event::MouseButtonDown{x, y, ..} => {
                let (wx, wy) = self.transform.pick_world(x as u32, y as u32);
                if let Some(pos) = self.terrain.get_xy_of_position(wx, wy) {
                    self.selected_rect = pos;
                }
            }
            Event::KeyDown{keycode: Some(Keycode::P), ..} => { if !self.dead {self.pause = !self.pause}}
            _ => {}
        }
    }
}