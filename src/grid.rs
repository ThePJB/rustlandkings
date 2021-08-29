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
                    (x_to_march.abs(), y_to_march.abs())
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

#[test]
fn test_raycast() {
    {
        /*
        ####
        #e##
        # ##
        # p#
        ####
        */
        let mut g = Grid::new(10, 10, 1.0, 1.0);
        g.set_2d(1, 1, Tile::Ground);
        g.set_2d(1, 2, Tile::Ground);
        g.set_2d(1, 3, Tile::Ground);
        g.set_2d(2, 3, Tile::Ground);
        assert_eq!(g.raycast(Vec2::new(1.5, 1.5), Vec2::new(2.6, 3.6)), Some(Vec2::new(2.0, 2.4545455))); // idk i saw this Noneing during gameplay
    }
    {
        /*
        ####
        ##p#
        #e #
        ####
        */
        let mut g = Grid::new(10, 10, 1.0, 1.0);
        g.set_2d(2, 2, Tile::Ground);
        g.set_2d(2, 1, Tile::Ground);
        g.set_2d(1, 2, Tile::Ground);
        assert_eq!(g.raycast(Vec2::new(1.5, 2.5), Vec2::new(2.9, 1.9)), None);
    }
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
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(3.5, 1.5)), None);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(2.0, 2.0)), None);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(3.0, 3.0)), None);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(5.0, 5.0)), None);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(8.5, 8.5)), None);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(8.5, 0.5)), None);
        assert_eq!(g.raycast(Vec2::new(0.5, 0.5), Vec2::new(0.5, 8.5)), None);

    }
}

#[test]
fn test_grid() {
    let g = Grid::new(10, 10, 1.0, 1.0);
    assert_eq!(g.get_xy_of_position(Vec2::new(5.5, 6.5)), (5, 6));
}