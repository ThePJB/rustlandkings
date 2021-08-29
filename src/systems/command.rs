use crate::vec2::*;
use crate::simulation_state::*;
use crate::systems::projectiles::*;
use crate::entity::*;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum Command {
    Walk(u32, Vec2), // walker, direction (or stop by doing 0.0)
    Look(u32, Vec2), // looker, dir
    Shoot(u32), // shooter
}

pub fn apply_command(state: &mut SimulationState, command: Command) {
    match command {
        Command::Walk(walker_id, direction) => {
            if let Some(mut walker) = state.entities.get_mut(&walker_id) {
                walker.velocity = direction.mul_scalar(walker.speed);
            }
        },
        Command::Look(id, dir) => {
            if let Some(mut looker) = state.entities.get_mut(&id) {
                looker.look_direction = dir;
            }
        }
        Command::Shoot(shooter_id) => {
            if let Some(mut shooter) = state.entities.get_mut(&shooter_id) {
                let bullets = shoot_gun(&mut shooter, shooter_id, state.time as f32);
                for bullet in bullets {
                    state.entities.insert(rand::thread_rng().gen(), bullet);
                }
            }
        },
    }
}

// keep em separate from effects and shit for now...
// though they do seem similar
// as also do collisions and movements to some extent