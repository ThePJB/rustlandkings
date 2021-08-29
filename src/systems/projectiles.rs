use crate::{side_effect::SideEffect, simulation_state::*};
use crate::systems::collision::*;
use crate::entity::*;
use crate::vec2::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Gun {
    damage: f32,
    spread: f32,
    speed: f32,
    last_fired: f32,
    num_bullets: i32,
    pub automatic: bool,
    cooldown: f32,
    ammo: i32,
    
    burst: i32,
    burst_count: i32,
    burst_cooldown: f32,

    keep_shooting: bool,
}
/*
pistol
shotgun
machine gun
burst fire gun
super inaccurate smg thing, skorpion or whatever
burst shotgun like scattergun
sniper rifle

could have d2 modifiers lol
*/
impl Gun {
    pub fn new_pistol() -> Gun {
        Gun { 
            damage: 1.0, 
            spread: 0.1, 
            speed: 2.0, 
            num_bullets: 1, 
            automatic: false, 
            cooldown: 0.5, 
            burst: 1, 
            burst_count: 0,
            burst_cooldown: 0.0, 
            ammo: 24,
            last_fired: 0.0,

            keep_shooting: false,
        }
    }
    pub fn new_npc_gun() -> Gun {
        Gun { 
            damage: 1.0, 
            spread: 0.1, 
            speed: 0.8, 
            num_bullets: 1, 
            automatic: false, 
            cooldown: 0.5, 
            burst: 1, 
            burst_count: 0,
            burst_cooldown: 0.0, 
            ammo: 24,
            last_fired: 0.0,

            keep_shooting: false,
        }
    }
    pub fn new_sprayer_gun() -> Gun {
        Gun { 
            damage: 0.5, 
            spread: 0.2, 
            speed: 0.7, 
            num_bullets: 1, 
            automatic: true, 
            cooldown: 0.1, 
            burst: 7, 
            burst_count: 0,
            burst_cooldown: 2.0, 
            ammo: 48,
            last_fired: 0.0,

            keep_shooting: false,
        }
    }

    pub fn new_makina() -> Gun {
        Gun { 
            damage: 0.7, 
            spread: 0.15, 
            speed: 1.8, 
            num_bullets: 1, 
            automatic: true, 
            cooldown: 0.05, 
            ammo: 200, 
            last_fired: 0.0,

            burst: 1, 
            burst_count: 0,
            burst_cooldown: 0.0, 

            keep_shooting: false,
        }
    }

    pub fn new_burst_rifle() -> Gun {
        Gun { 
            damage: 1.0, 
            spread: 0.02, 
            speed: 1.8, 
            num_bullets: 1, 
            automatic: true, 
            cooldown: 0.04, 
            ammo: 100, 
            last_fired: 0.0,

            burst: 3, 
            burst_count: 0,
            burst_cooldown: 0.33, 

            keep_shooting: true,
        }
    }

    pub fn new_shotgun() -> Gun {
        Gun { 
            damage: 1.0, 
            spread: 0.3, 
            speed: 2.0, 
            num_bullets: 6, 
            automatic: true, 
            cooldown: 0.5, 
            ammo: 24, 
            last_fired: 0.0,

            burst: 1, 
            burst_count: 0,
            burst_cooldown: 1.0, 

            keep_shooting: false,
        }
    }

    pub fn new_scattergun() -> Gun {
        Gun { 
            damage: 1.0, 
            spread: 0.3, 
            speed: 2.0, 
            num_bullets: 4, 
            automatic: true, 
            cooldown: 0.2, 
            ammo: 36, 
            last_fired: 0.0,

            burst: 2, 
            burst_count: 0,
            burst_cooldown: 1.0, 

            keep_shooting: false,
        }
    }
    pub fn new_bigdog_gun() -> Gun {
        Gun { 
            damage: 0.5, 
            spread: 0.5, 
            speed: 0.6, 
            num_bullets: 5, 
            automatic: true, 
            cooldown: 0.4, 
            ammo: 36, 
            last_fired: 0.0,

            burst: 2, 
            burst_count: 0,
            burst_cooldown: 4.0, 

            keep_shooting: false,
        }
    }
}

// ok now we need a fn to handle shooting. I guess its in command handle
// maybe a fn that takes &mut state and makes the bullets and shit
// but &mut state would alias the gun?
// well we could do the dispatching ourself

// could be a mut ref to a gun and return a Vec of entities
/*
pub fn shoot_gun(state: &mut SimulationState, shooter: u32, target_pos: Vec2) {

}
*/

// i think I want to go crazy with enums mmmmmm y

pub fn shoot_gun(entity: &mut Entity, entity_id: u32, time: f32) -> Vec<Entity> {
    let mut new_entities = Vec::new();

    if entity.gun.ammo <= 0 { return new_entities; }
    if time - entity.gun.last_fired < entity.gun.cooldown { return new_entities; }
    if entity.gun.burst_count == 0 {
        if time - entity.gun.last_fired > entity.gun.burst_cooldown {
            entity.gun.burst_count = entity.gun.burst; // restock burst
        } else {
            // burst needs to recharge still
            entity.gun.keep_shooting = false;
            return new_entities;
        }
    }
    
    entity.gun.ammo -= 1;
    entity.gun.last_fired = time;
    entity.gun.burst_count -= 1;
    if entity.gun.burst_count > 0 {
        entity.gun.keep_shooting = true;
    }
    
    for _ in 0..entity.gun.num_bullets {
        let mut bullet = Entity::new_bullet(entity.aabb.center(), entity.look_direction, entity.force, entity_id);
        bullet.melee_damage = entity.gun.damage;
        bullet.velocity = bullet.velocity.normalize().spread(entity.gun.spread).mul_scalar(entity.gun.speed);
        
        new_entities.push(bullet);
    }

    return new_entities;
}


pub fn handle_bullet_impacts(state: &SimulationState, collisions: &Vec<CollisionEvent>, effects: &mut Vec<SideEffect>) {
    for col in collisions.iter() {
        if let Some(subject) = state.entities.get(&col.subject) {
            if subject.variety == EntityType::Bullet {
                match col.object {
                    CollisionObject::Entity(id) => {
                        if let Some(object) = state.entities.get(&id) {
                            effects.push(SideEffect::Damage(subject.melee_damage, id));
                            effects.push(SideEffect::Damage(999.0, col.subject));
                            match object.variety {
                                EntityType::Retaliator => {
                                    effects.push(SideEffect::SpawnCircBullets(8, 1.0, EntityForce::Neutral, object.aabb.center(), id));
                                }
                                _ => {},
                            }
                        } else {
                            // probably shouldn't happen
                            panic!("panic time");
                        }
                    },
                    CollisionObject::Terrain(_x, _y) => {
                        effects.push(SideEffect::Damage(999.0, col.subject));
                    },
                }
            }
        }
    }
}