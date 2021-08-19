use crate::systems::command::*;
use crate::simulation_state::*;
use crate::entity::*;

pub fn compute_ai_commands(state: &SimulationState, commands: &mut Vec<Command>) {
    for (enemy_id, enemy) in state.entities.iter().filter(|(_, e)| e.variety == EntityType::Enemy) {

        // look for a target to shoot
        for (target_id, target) in state.entities.iter().filter(|(_, e)| e.variety == EntityType::Player) {
            let distance = enemy.aabb.center().sub(target.aabb.center()).magnitude();
            if distance < 0.5 {
                commands.push(Command::Shoot(*enemy_id, target.aabb.center()));
            }
        }
    }
}