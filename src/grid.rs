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
            tiles: vec![Tile::Ground; (w*h) as usize],
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

    pub fn get_xy_of_position(&self, v: Vec2) -> Option<(i32, i32)> {
        let ix = (v.x / self.elem_w) as i32;
        let iy = (v.y / self.elem_h) as i32;
        if ix >= self.w || iy >= self.h || ix < 0 || iy < 0 {
            None
        } else {
            Some((ix, iy))
        }
    }

    pub fn get_position(&self, v: Vec2) -> Option<Tile> {
        // i bet theres a big brain monad way to do this
        if let Some((ix, iy)) = self.get_xy_of_position(v) {
            self.get_2d(ix, iy)
        } else {
            None
        }
    }
}

#[test]
fn test_grid() {
    let g = Grid::new(10, 10, 1.0, 1.0);
    assert_eq!(g.get_xy_of_position(Vec2::new(5.5, 6.5)), Some((5, 6)));
}