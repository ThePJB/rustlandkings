use crate::simulation_state::*;
use crate::systems::collision::*;
use crate::side_effect::*;

pub fn handle_melee_damage(state: &SimulationState, collisions: &Vec<CollisionEvent>, effects: &mut Vec<SideEffect>) {
    for col in collisions.iter() {
        if let Some(subject) = state.entities.get(&col.subject) {
            if subject.melee_damage != 0.0 {
                match col.object {
                    CollisionObject::Entity(id) => {
                        if let Some(object) = state.entities.get(&id) {
                            effects.push(SideEffect::Damage(subject.melee_damage * state.dt as f32, id));

                        }
                    },
                    _ => {},
                }
            }
        }
    }
}