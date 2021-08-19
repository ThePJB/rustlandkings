use crate::vec2::*;
use crate::simulation_state::*;
use crate::entity::*;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum Command {
    Walk(u32, Vec2), // walker, direction (or stop by doing 0.0)
    Shoot(u32, Vec2), // shooter, target
}

pub fn apply_command(state: &mut SimulationState, command: Command) {
    match command {
        Command::Walk(walker_id, direction) => {
            if let Some(mut walker) = state.entities.get_mut(&walker_id) {
                walker.velocity = direction.mul_scalar(0.5);
            }
        },
        Command::Shoot(shooter_id, target) => {
            if let Some(shooter) = state.entities.get(&shooter_id) {
                let start_pos = shooter.aabb.center();
                state.entities.insert(rand::thread_rng().gen(), Entity::new_bullet(start_pos, target, shooter.force, shooter_id));
            }
        },
    }
}

// keep em separate from effects and shit for now...
// though they do seem similar
// as also do collisions and movements to some extent