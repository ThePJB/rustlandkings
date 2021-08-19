use crate::entity::*;
use crate::simulation_state::*;
use crate::vec2::*;
use std::f32::consts::PI;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum SideEffect {
    None,
    Damage(f32, u32),                               // damage subject
    SpawnCircBullets(i32, f32, EntityForce, Vec2, u32),  // n, damage, force, pos, source

}

impl SimulationState {
    pub fn resolve_side_effect(&mut self, effect: SideEffect) {
        match effect {
            SideEffect::Damage(amount, subject) => {
                if let Some(mut entity) = self.entities.get_mut(&subject) {
                    entity.health -= amount;
                } else {
                    // maybe trace this if theres a suspicion its wrong, it shouldnt really fail
                    panic!("shouldnt happen");
                }
            },
            SideEffect::SpawnCircBullets(n_bullets, _damage, force, pos, source) => {
                for i in 1..n_bullets {
                    let i_frac = i as f32 / n_bullets as f32;
                    let dirn_vec = Vec2::new((i_frac * 2.0*PI).sin(),(i_frac * 2.0*PI).cos());
                    
                    self.entities.insert(rand::thread_rng().gen(), Entity::new_bullet(pos, pos.add(dirn_vec), force, source));
                }
            },
            SideEffect::None => {},
        }
    }
}