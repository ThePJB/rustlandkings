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

struct Walker {
    pos: (i32, i32),
    dir: i32,
    alive: bool,
}

pub fn generate_level_drunk() -> SimulationState {
    let side_length = 20;
    let elem_size = 0.2;
    let num_walkers = 40;
    let walk_iters = 10;
    let p_change_dir = 0.3;

    let mut g = Grid::new(side_length, side_length, elem_size, elem_size);
    let mut walkers = Vec::new();

    let dirs = [(1, 0), (-1,0), (0, -1), (0, 1)];
    
    for i in 0..num_walkers {
        walkers.push(Walker {
            pos: 
                (rand::thread_rng().gen_range(1..side_length-1),
                rand::thread_rng().gen_range(1..side_length-1)),
            dir: rand::thread_rng().gen_range(0..4),
            alive: true,
        });
    }

    // Y entities spawn in walls???

    for _ in 1..walk_iters {
        for w in walkers.iter_mut() {
            g.set_2d(w.pos.0, w.pos.1, Tile::Ground);
            if !w.alive {
                continue;
            }

            // maybe change direction
            if rand::thread_rng().gen_range(0.0..1.0) < p_change_dir {
                let mut idx = rand::thread_rng().gen_range(0..3);
                if idx >= w.dir {
                    idx += 1;
                }
                w.dir = idx;
            }

            // advance
            // kill instead of going off
            let dir = dirs[w.dir as usize];
            let candidate_pos = (w.pos.0 + dir.0, w.pos.1 + dir.1);
            if candidate_pos.0 <= 0 || candidate_pos.1 <= 0 || candidate_pos.0 >= g.w-1 || candidate_pos.1 >= g.h-1 {
                w.alive = false;
            } else {
                w.pos = candidate_pos;
            }
        }
    }

    /*
    this should not be necessary but it is, which implies that walkers are misbehaving with setting their position to be 'ground'
    but thats bizarre as its pretty unconditional
    its the final spot thats not getting converted for some reason
    */
    for w in walkers.iter() {
        g.set_2d(w.pos.0, w.pos.1, Tile::Ground);
    }

    let mut entities = HashMap::new();
    let player_pos = g.get_rect_2d(walkers[0].pos.0, walkers[0].pos.1).center();
    entities.insert(rand::thread_rng().gen(), Entity::new_player(player_pos.x, player_pos.y));

    for w in walkers[1..].iter() {
        let walker_pos = g.get_rect_2d(w.pos.0, w.pos.1).center();
                                                                        
        entities.insert(rand::thread_rng().gen(), match rand::thread_rng().gen_range(0..3) {
            0 => {Entity::new_enemy(walker_pos.x, walker_pos.y)}
            1 => {Entity::new_crate(walker_pos.x, walker_pos.y)}
            2 => {Entity::new_retalliator(walker_pos.x, walker_pos.y)}
            _ => {panic!("shouldnt happen")},
        });
    }

    return SimulationState {
        time: 0.0,
        entities: entities,
        terrain: g,
    };

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