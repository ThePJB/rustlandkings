use crate::rect::*;
use crate::vec2::*;
use crate::game::*;
use crate::side_effect::*;

use sdl2::controller::GameController;
use sdl2::pixels::Color;


pub enum PlatformHeight {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawOrder {
    Front,
    Back,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityForce {
    Player,
    Neutral,
    Enemy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollisionGroup {
    Bullet,
    Static,
    Other,    
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Bullet,
    Crate,
    Retaliator,
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub force: EntityForce,
    pub collision_group: CollisionGroup,
    pub variety: EntityType,
    pub source: u32,
    
    pub aabb: Rect,
    pub colour: Color,
    pub velocity: Vec2,
    pub draw_order: DrawOrder,
    pub health: f32,
    pub last_hit: f32, // for iframes etc
}

impl Entity {
    pub fn new_player(x: f32, y: f32) -> Entity {
        Entity {
            aabb: Rect::new(x, y, 0.05, 0.05),
            colour: Color::RGB(255, 255, 255),
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Front,
            force: EntityForce::Player,
            collision_group: CollisionGroup::Other,
            health: 5.0,
            last_hit: 0.0,
            variety: EntityType::Player,
            source: 0,
        }
    }

    pub fn new_crate(x: f32, y: f32) -> Entity {
        Entity { 
            force: EntityForce::Neutral,
            collision_group: CollisionGroup::Static,
            variety: EntityType::Crate,
            aabb: Rect::new_centered(x, y, 0.15, 0.15),
            colour: Color::RGB(64, 64, 0),
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Back, health: 4.0, 
            last_hit: 0.0,
            source: 0,
        }
    }

    pub fn new_retalliator(x: f32, y: f32) -> Entity {
        Entity { 
            force: EntityForce::Neutral,
            collision_group: CollisionGroup::Static,
            variety: EntityType::Retaliator,
            aabb: Rect::new_centered(x, y, 0.2, 0.2),
            colour: Color::RGB(32, 32, 32),
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Back, health: 10.0, 
            last_hit: 0.0,
            source: 0,
        }
    }

    pub fn new_bullet(from: Vec2, to: Vec2, force: EntityForce, source: u32) -> Entity {
        let bullet_s = 0.02;
        let bullet_speed = 2.0;

        Entity { 
            aabb: Rect::new_centered(from.x, from.y, bullet_s, bullet_s), 
            colour: Color::RGB(255, 255, 0), 
            velocity: to.sub(from).normalize().mul_scalar(bullet_speed),
            draw_order: DrawOrder::Front, 
            force: force,
            collision_group: CollisionGroup::Bullet,
            health: 1.0,
            last_hit: 0.0,
            variety: EntityType::Bullet,
            source: source,
        }
    }
}