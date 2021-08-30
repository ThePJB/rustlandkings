use crate::rect::*;
use crate::vec2::*;
use crate::game::*;
use crate::side_effect::*;
use crate::systems::projectiles::*;

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
    Enemy,
    Swarmer,
    Sprayer,
    Bigdog,
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

    pub speed: f32,

    pub melee_damage: f32,
    pub look_direction: Vec2,
    pub gun: Gun,
}

impl Entity {
    pub fn new_player(x: f32, y: f32) -> Entity {
        Entity {
            aabb: Rect::new_centered(x, y, 0.05, 0.05),
            colour: Color::RGB(255, 255, 255),
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Front,
            force: EntityForce::Player,
            collision_group: CollisionGroup::Other,
            health: 5.0,
            last_hit: 0.0,
            variety: EntityType::Player,
            source: 0,
            melee_damage: 0.0,
            look_direction: Vec2::new(1.0, 0.0),
            speed: 0.6,
            gun: Gun::new_burst_rifle(),
        }
    }

    pub fn new_enemy(x: f32, y: f32) -> Entity {
        Entity {
            aabb: Rect::new_centered(x, y, 0.08, 0.08),
            colour: Color::RGB(255, 0, 0),
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Front,
            force: EntityForce::Enemy,
            collision_group: CollisionGroup::Other,
            health: 5.0,
            last_hit: 0.0,
            variety: EntityType::Enemy,
            source: 0,
            melee_damage: 0.0,
            look_direction: Vec2::new(1.0, 0.0),
            gun: Gun::new_npc_gun(),
            speed: 0.6,
        }
    }
    pub fn new_sprayer(x: f32, y: f32) -> Entity {
        Entity {
            aabb: Rect::new_centered(x, y, 0.07, 0.07),
            colour: Color::RGB(0, 0, 255),
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Front,
            force: EntityForce::Enemy,
            collision_group: CollisionGroup::Other,
            health: 5.0,
            last_hit: 0.0,
            variety: EntityType::Sprayer,
            source: 0,
            melee_damage: 0.0,
            look_direction: Vec2::new(1.0, 0.0),
            gun: Gun::new_sprayer_gun(),
            speed: 0.3,
        }
    }
    pub fn new_bigdog(x: f32, y: f32) -> Entity {
        Entity {
            aabb: Rect::new_centered(x, y, 0.12, 0.12),
            colour: Color::RGB(0, 0, 128),
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Front,
            force: EntityForce::Enemy,
            collision_group: CollisionGroup::Other,
            health: 10.0,
            last_hit: 0.0,
            variety: EntityType::Bigdog,
            source: 0,
            melee_damage: 0.0,
            look_direction: Vec2::new(1.0, 0.0),
            gun: Gun::new_bigdog_gun(),
            speed: 0.1,
        }
    }
    
    pub fn new_swarmer(x: f32, y: f32) -> Entity {
        let mut enemy = Entity::new_enemy(x, y);
        enemy.variety = EntityType::Swarmer;
        enemy.colour = Color::RGB(128, 0, 0);
        enemy.aabb = Rect::new_centered(x, y, 0.05, 0.05);
        enemy.melee_damage = 2.0;
        enemy.speed = 0.6;
        enemy.health = 3.0;
        return enemy;
    }
    
    pub fn new_crate(x: f32, y: f32) -> Entity {
        Entity { 
            force: EntityForce::Neutral,
            collision_group: CollisionGroup::Static,
            variety: EntityType::Crate,
            aabb: Rect::new_centered(x, y, 0.1, 0.1),
            colour: Color::RGB(64, 64, 0),
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Back, health: 4.0, 
            last_hit: 0.0,
            source: 0,
            melee_damage: 0.0,
            look_direction: Vec2::new(1.0, 0.0),
            gun: Gun::new_pistol(),
            speed: 0.6,
        }
    }

    pub fn new_retalliator(x: f32, y: f32) -> Entity {
        Entity { 
            force: EntityForce::Neutral,
            collision_group: CollisionGroup::Static,
            variety: EntityType::Retaliator,
            aabb: Rect::new_centered(x, y, 0.15, 0.15),
            colour: Color::RGB(32, 32, 32),
            velocity: Vec2::zero(),
            draw_order: DrawOrder::Back, health: 10.0, 
            last_hit: 0.0,
            source: 0,
            melee_damage: 0.0,
            look_direction: Vec2::new(1.0, 0.0),
            gun: Gun::new_pistol(),
            speed: 0.6,
        }
    }

    pub fn new_bullet(from: Vec2, dir: Vec2, force: EntityForce, source: u32) -> Entity {
        let bullet_s = 0.02;
        let bullet_speed = 0.7;

        Entity { 
            aabb: Rect::new_centered(from.x, from.y, bullet_s, bullet_s), 
            colour: Color::RGB(255, 255, 0), 
            velocity: dir.mul_scalar(bullet_speed),
            draw_order: DrawOrder::Front, 
            force: force,
            collision_group: CollisionGroup::Bullet,
            health: 1.0,
            last_hit: 0.0,
            variety: EntityType::Bullet,
            source: source,
            melee_damage: 1.0,
            look_direction: Vec2::new(1.0, 0.0),
            gun: Gun::new_pistol(),
            speed: 0.6,
        }
    }
}