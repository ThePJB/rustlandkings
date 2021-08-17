use crate::rect::*;
use crate::vec2::*;

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

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub aabb: Rect,
    pub colour: Color,
    pub velocity: Vec2,
    pub draw_order: DrawOrder,
    pub force: EntityForce,
    pub collision_group: CollisionGroup,
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
        }
    }

    pub fn new_platform(player_x: f32, which: PlatformHeight) -> Entity {
        Entity {
            aabb: match which {
                PlatformHeight::Top => Rect::new(player_x + 0.5, 0.4, 0.1, 0.05),
                PlatformHeight::Middle => Rect::new(player_x + 0.4, 0.6, 0.1, 0.05),
                PlatformHeight::Bottom => Rect::new(player_x + 0.3, 0.8, 0.1, 0.05),
            },
            colour: match which {
                PlatformHeight::Top => Color::RGB(255,0,0),
                PlatformHeight::Middle => Color::RGB(0, 255, 0),
                PlatformHeight::Bottom => Color::RGB(0, 0, 255),
            },
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Front,
            force: EntityForce::Neutral,
            collision_group: CollisionGroup::Static,
            health: 5.0,
            last_hit: 0.0,
        }
    }

    pub fn new_bullet(from: Vec2, to: Vec2) -> Entity {
        let bullet_s = 0.02;
        let bullet_speed = 3.0;
        Entity { 
            aabb: Rect::new_centered(from.x, from.y, bullet_s, bullet_s), 
            colour: Color::RGB(255, 255, 0), 
            velocity: to.sub(from).normalize().mul_scalar(bullet_speed),
            draw_order: DrawOrder::Front, 
            force: EntityForce::Player,
            collision_group: CollisionGroup::Bullet,
            health: 1.0,
            last_hit: 0.0,
        }
    }
}