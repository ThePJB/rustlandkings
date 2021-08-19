use crate::{side_effect::SideEffect, simulation_state::*};
use crate::systems::collision::*;
use crate::entity::*;

pub fn handle_bullet_impacts(state: &SimulationState, collisions: &Vec<CollisionEvent>, effects: &mut Vec<SideEffect>) {
    for col in collisions.iter() {
        if let Some(subject) = state.entities.get(&col.subject) {
            if subject.variety == EntityType::Bullet {
                match col.object {
                    CollisionObject::Entity(id) => {
                        if let Some(object) = state.entities.get(&id) {
                            effects.push(SideEffect::Damage(1.0, id));
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