use crate::grid::*;
use crate::entity::*;
use std::collections::HashMap;
use rand::Rng;


pub struct SimulationState {
    pub time: f64,
    pub terrain: Grid,
    pub entities: HashMap<u32, Entity>,
}

fn generate_level() -> Grid {
    let w = 10;
    let h = 10;
    let elem_s = 0.2;

    let mut g = Grid::new(w, h, elem_s, elem_s);
    for i in 0..h {
        g.set_2d(i, 0, Tile::Wall);
        g.set_2d(0, i, Tile::Wall);
        g.set_2d(i, h-1, Tile::Wall);
        g.set_2d(w-1, i, Tile::Wall);
    }
    return g;
}

impl SimulationState {
    pub fn new() -> SimulationState {
        let mut state = SimulationState {
            time: 0.0,
            terrain: generate_level(),
            entities: HashMap::new(),
        };

        state.entities.insert(rand::thread_rng().gen(), Entity::new_player(0.4, 0.4));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_enemy(0.8, 1.1));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_enemy(1.5, 0.5));

        state.entities.insert(rand::thread_rng().gen(), Entity::new_crate(0.8, 0.4));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_crate(0.8, 0.9));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_crate(1.0, 0.2));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_crate(1.0, 0.4));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_crate(1.0, 0.9));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_crate(1.2, 0.6));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_crate(1.2, 0.7));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_crate(1.6, 1.0));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_crate(1.6, 1.6));

        state.entities.insert(rand::thread_rng().gen(), Entity::new_retalliator(0.3, 1.6));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_retalliator(1.0, 1.0));
        state.entities.insert(rand::thread_rng().gen(), Entity::new_retalliator(0.9, 1.5));



        return state;
    }
}