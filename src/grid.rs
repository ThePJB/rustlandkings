use crate::rect::*;
use crate::vec2::*;

// return some kind of cursor with a getabove, etc

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Ground,
    Wall,
}

pub struct Grid {
    pub w: i32,
    pub h: i32,
    pub elem_w: f32,
    pub elem_h: f32,
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(w: i32, h: i32, elem_w: f32, elem_h: f32) -> Grid {
        Grid {
            w: w,
            h: h,
            elem_w: elem_w,
            elem_h: elem_h,
            tiles: vec![Tile::Wall; (w*h) as usize],
        }
    }

    pub fn set_2d(&mut self, x: i32, y: i32, t: Tile) {
        self.tiles[(x + y * self.w) as usize] = t;
    }

    pub fn get_2d(&self, x: i32, y: i32) -> Option<Tile> {
        if x >= self.w || y >= self.h || x < 0 || y < 0 {
            None
        } else {
            Some(self.tiles[(x + y * self.w) as usize])
        }
    }

    pub fn get_rect_2d(&self, x: i32, y: i32) -> Rect {
        Rect {
            x: self.elem_w * x as f32,
            y: self.elem_h * y as f32,
            w: self.elem_w,
            h: self.elem_h,
        }
    }

    pub fn get_rect_1d(&self, i: i32) -> Rect {
        let x = i % self.w;
        let y = i / self.w;
        self.get_rect_2d(x,y)
    }

    pub fn get_xy_of_position(&self, v: Vec2) -> (i32, i32) {
        let ix = (v.x / self.elem_w) as i32;
        let iy = (v.y / self.elem_h) as i32;
        (ix, iy)
        /*
        if ix >= self.w || iy >= self.h || ix < 0 || iy < 0 {
            None
        } else {
            Some((ix, iy))
        }
        */
    }

    pub fn get_position(&self, v: Vec2) -> Option<Tile> {
        let (ix, iy) = self.get_xy_of_position(v);
        self.get_2d(ix, iy)
    }

    // i still probably dont floor/ceil to the right size either

    // um infinte loop lol forgetting to increment anything?
    pub fn raycast(&self, ray_origin: Vec2, ray_destination: Vec2) -> Option<Vec2> {
        let round_up = |u: f32, side_length: f32| {
            (u/side_length).ceil() * side_length
        };
        let round_down = |u: f32, side_length: f32| {
            (u/side_length).floor() * side_length
        };
        let bound = |u, sign: i32, side_length| {
            if sign >= 0 {
                let ru = round_up(u, side_length);
                if ru == u { side_length } else {ru - u}
            } else {
                let ru = round_down(u, side_length);
                if ru == u { side_length } else {u - ru}
            }
        };

        let (mut grid_x, mut grid_y) = self.get_xy_of_position(ray_origin); // a bit unholy actually, but things shouldnt go oob // can ?
        let (grid_dest_x, grid_dest_y) = self.get_xy_of_position(ray_destination);

        println!("raycasting from ({}, {}) to ({}, {})", grid_x, grid_y, grid_dest_x, grid_dest_y);

        let delta_vec = ray_destination.sub(ray_origin);
        let ray_dir = delta_vec.normalize();

        // increment these
        let mut actual_march_x: f32 = ray_origin.x;
        let mut actual_march_y: f32 = ray_origin.y;

        let sign_x = if delta_vec.x > 0.0 { 1 } else { -1 };
        let sign_y = if delta_vec.y > 0.0 { 1 } else { -1 };

        // cycle through these
        let side_length = self.elem_w; // should just be elems
        let mut next_tile_in_x: f32 = bound(actual_march_x, sign_x, side_length);
        let mut next_tile_in_y: f32 = bound(actual_march_y, sign_y, side_length);

        let mut n = 0;
        loop {
            if n > 9999 { 
                panic!("raycast infinite loop");
                println!("bailing");
                return None; 
            }
            n += 1;
            println!("raycast loop ({:.2},{:.2}), sign ({:?},{:?}) grid ({}, {})", actual_march_x, actual_march_y, sign_x, sign_y, grid_x, grid_y);
            // might be a bit inefficient, checking same thing repeatedly, dont care its more readable rn
            // check to terminate (wall strike)
            println!("check ({}, {})", grid_x, grid_y);
            if self.get_2d(grid_x, grid_y).unwrap() == Tile::Wall {
                return Some(Vec2::new(actual_march_x, actual_march_y));
            }

            if grid_x == grid_dest_x && grid_y == grid_dest_y {
                return None;
            }


            let x_distance = bound(actual_march_x, sign_x, side_length);
            let y_distance = bound(actual_march_y, sign_y, side_length);

            let x_want = (x_distance / ray_dir.x).abs();
            let y_want = (y_distance / ray_dir.y).abs();
            
            println!("distance ({:.2} {:.2})", x_distance, y_distance);
            println!("want ({:.2}, {:.2})", x_want, y_want);

            let (x_to_march, y_to_march) = // this msut be wrong
                if x_want <= y_want {
                    println!("move in x direction");
                    let x_to_march = x_distance;
                    let y_to_march = ray_dir.div_scalar(ray_dir.x).mul_scalar(x_distance).y;
                    (x_to_march, y_to_march)
                } else {
                    println!("move in y direction");
                    let y_to_march = y_distance;
                    let x_to_march = ray_dir.div_scalar(ray_dir.y).mul_scalar(y_distance).x;
                    (x_to_march.abs(), y_to_march.abs())
                };

            println!("xtm, ytm: ({}, {})", x_to_march, y_to_march);

            // march the ray
            actual_march_x += x_to_march * sign_x as f32;
            actual_march_y += y_to_march * sign_y as f32;

            // calculate grid update
            next_tile_in_x -= x_to_march;
            if next_tile_in_x <= 0.0 {
                next_tile_in_x += side_length;
                grid_x += sign_x;
            }
            next_tile_in_y -= y_to_march;
            if next_tile_in_y <= 0.0 {
                next_tile_in_y += side_length;
                grid_y += sign_y;
            }
            println!("next tile in: ({:?}, {:?})", next_tile_in_x, next_tile_in_y);

        }
    }
}

// probably should make grid super tight to being on edges and shit
// any algorithm with floating point is annoying as fuck lol
// but thats a bad attitude its just bytes

// need to keep track properly of paid and unpaid

// im also not accounting for grid size

#[test]
fn test_raycast() {
    {
        let mut g = Grid::new(10, 10, 1.0, 1.0);
        for i in 0..10 {
            for j in 0..10 {
                g.set_2d(i,j, Tile::Ground);
            }
        }
        g.set_2d(0, 5, Tile::Wall);
        assert_eq!(g.raycast(Vec2::new(0.5, 9.5), Vec2::new(0.5, 0.5)), Some(Vec2::new(0.5, 6.0)));
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(0.5, 9.0)), Some(Vec2::new(0.5, 5.0)));
    }
    {
        let mut g = Grid::new(10, 10, 1.0, 1.0);
        for i in 0..10 {
            for j in 0..10 {
                g.set_2d(i,j, Tile::Ground);
            }
        }
        g.set_2d(5, 0, Tile::Wall);
        assert_eq!(g.raycast(Vec2::new(9.5, 0.5), Vec2::new(0.5, 0.5)), Some(Vec2::new(6.0, 0.5)));
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(9.0, 0.5)), Some(Vec2::new(5.0, 0.5)));
    }
    {
        let mut g = Grid::new(10, 10, 1.0, 1.0);
        g.set_2d(0, 0, Tile::Ground);
        g.set_2d(1, 0, Tile::Ground);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.6)), None);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5)), None);
        g.set_2d(1, 0, Tile::Wall);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5)), Some(Vec2::new(1.0, 0.5)));
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(5.5, 0.5)), Some(Vec2::new(1.0, 0.5)));
    }
    {
        let mut g = Grid::new(10, 10, 1.0, 1.0);
        g.set_2d(0, 0, Tile::Ground);
        g.set_2d(0, 1, Tile::Ground);
        g.set_2d(1, 1, Tile::Ground);
        g.set_2d(2, 1, Tile::Ground);
        g.set_2d(3, 1, Tile::Ground);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.1), Vec2::new(2.0, 1.6)), Some(Vec2::new(1.0, 0.6)));
    }
    {
        // moar testing needed cause it no worky properly in game
        let mut g = Grid::new(10, 10, 1.0, 1.0);
        for i in 0..10 {
            for j in 0..10 {
                g.set_2d(i,j, Tile::Ground);
            }
        }
        println!("asssss");
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(3.5, 1.5)), None);
        println!("ass");
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(2.0, 2.0)), None);
        println!("ass2");
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(3.0, 3.0)), None);
        println!("ass3");
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(5.0, 5.0)), None);
        println!("ass4");
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(8.5, 8.5)), None);
        println!("ass5");
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(8.5, 0.5)), None);
        println!("ass6");
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(0.5, 8.5)), None);

    }
}

#[test]
fn test_grid() {
    let g = Grid::new(10, 10, 1.0, 1.0);
    assert_eq!(g.get_xy_of_position(Vec2::new(5.5, 6.5)), (5, 6));
}

/*

// --------------- picking

float intbound(float s, float ds) {
    if (ds < 0) {
        return intbound(-s, -ds);
    } else {
        if (ds > 0) {
            s = s - floorf(s);
        } else {
            s = s - ceilf(s);
        }
        return (1-s)/ds;
    }
}

pick_info pick_block(chunk_manager *world, vec3s pos, vec3s facing, float max_distance) {
    debugf("facing %.2f, %.2f, %.2f\n", facing.x, facing.y, facing.z);
    debugf("at %.2f, %.2f, %.2f\n", pos.x, pos.y, pos.z);

    pick_info ret = {0};
    ret.success = true;

    ret.coords = vec3s_to_vec3l(pos);

    int sx = signum(facing.x);
    int sy = signum(facing.y);
    int sz = signum(facing.z);

    float tMaxX = intbound(pos.x, facing.x);
    float tMaxY = intbound(pos.y, facing.y);
    float tMaxZ = intbound(pos.z, facing.z);

    debugf("initial tmx: %f, tmy: %.2f, tmz: %f\n", tMaxX, tMaxY, tMaxZ);
    //debugf("sx %d sy %d sz %d\n", sx, sy, sz);

    float accX = 0;
    float accY = 0;
    float accZ = 0;

    float tDeltaX = (float)sx / facing.x;
    float tDeltaY = (float)sy / facing.y;
    float tDeltaZ = (float)sz / facing.z;

    float max_squared = max_distance*max_distance;

    int n = 0;
    while (accX*accX + accY*accY + accZ*accZ <= max_squared) {
        n++;
        block_tag t = world_get_block(world, spread(ret.coords)).value;
        debugf("x: %d y: %d z: %d, t: %d\n", ret.coords.x, ret.coords.y, ret.coords.z, t);
        debugf("x dist: %.3f, y dist: %.3f, z dist: %.3f\n", accX, accY, accZ);
        //printf("tmx: %.2f, tmy: %.2f, tmz: %.2f\n", tMaxX, tMaxY, tMaxZ);
        if (t != BLOCK_AIR) {
            debugf("found block\n");
            ret.success=true;
            return ret;
        }

        if (tMaxX < tMaxY) {
            if (tMaxX < tMaxZ) {
                // X min
                ret.coords.x += sx;
                accX = tMaxX;
                tMaxX += tDeltaX;
                ret.normal_dir = sx < 0 ? DIR_PX : DIR_MX;
            } else {
                // Z min
                ret.coords.z += sz;
                accZ = tMaxZ;
                tMaxZ += tDeltaZ;
                ret.normal_dir = sz < 0 ? DIR_PZ : DIR_MZ;
            }
         } else {
            if (tMaxY < tMaxZ) {
                // Y min
                ret.coords.y += sy;
                accY = tMaxY;
                tMaxY += tDeltaY;
                ret.normal_dir = sy < 0 ? DIR_PY : DIR_MY;
            } else {
                // Z min (again)
                ret.coords.z += sz;
                accZ = tMaxZ;
                tMaxZ += tDeltaZ;
                ret.normal_dir = sz < 0 ? DIR_PZ : DIR_MZ;

            }
        }

    }
    ret.success = false;
    debugf("didnt find anything after n iters %d\n", n);
    //printf("bailed with accx: %.2f, accy: %.2f, accz: %.2f\n", accX, accY, accZ);

    return ret;
    
} */


/*
    // returns optionally the point of intersection
    // so check a clear line of site via looking for none
    pub fn raycast2(&self, p1: Vec2, p2: Vec2) -> Option<Vec2> {
        let mut current_pos = p1;

        let this_square_result = self.get_xy_of_position(current_pos);
        let dest_square_result = self.get_xy_of_position(p2);

        if this_square_result.is_none() || dest_square_result.is_none() { return None; }

        let (mut curr_x, mut curr_y) = this_square_result.unwrap();
        let (dest_x, dest_y) = dest_square_result.unwrap();

        let v = p2.sub(p1);
        let ray_dir = v.normalize();
        
        loop { // careful of infinite loop
            if self.get_2d(curr_x, curr_y).unwrap() == Tile::Wall {
                println!("wall at {:?}", (curr_x, curr_y));
                return Some(current_pos);
            }
            if dest_x == curr_x && dest_y == curr_y {
                return None; // we casted all the way without hitting a wall
            }
            // increment in either direction by whichever is lowest distance remaining
            let x_distance = if v.x > 0.0 {
                current_pos.x.ceil() - current_pos.x
            } else {
                current_pos.x - current_pos.x.floor()
            };

            let dx = x_distance / v.normalize().x;

            let y_distance = if v.y > 0.0 {
                current_pos.y.ceil() - current_pos.y
            } else {
                current_pos.y - current_pos.y.floor()
            };

            let dy = y_distance / v.normalize().y;


            // so theres HOW FAR IT HAS TO GO / 
            // and HOW MUCH IT IS GOING THAT WAY

            printlunwrapn!("dx {:?} dy {:?}", dx, dy);
            // probably needs to also take into account direction of v
            if dx < dy {
                println!("doing dx");
                current_pos.x += x_distance;
                current_pos.y += ray_dir.div_scalar(ray_dir.x).mul_scalar(x_distance).y;
                curr_x += 1;
            } else {
                println!("doing dy");
                current_pos.y += y_distance;
                current_pos.x += ray_dir.div_scalar(ray_dir.y).mul_scalar(y_distance).x;

                curr_y += 1;
            }
        }        
    }

*/
