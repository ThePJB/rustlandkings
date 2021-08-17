use crate::rect::*;
use crate::vec2::*;

#[derive(Clone, Copy, Debug)]
pub struct ScreenTransform {
    offset: Vec2,
    px: (u32, u32),
}

impl ScreenTransform {
    pub fn new(x: u32, y: u32) -> ScreenTransform {
        ScreenTransform{px: (x, y), offset: Vec2::zero()}
    }

    pub fn resize(&mut self, x: u32, y: u32) {
        self.px = (x, y);
        // maybe do stuff such that it stays centered
    }

    pub fn translate_center(&mut self, v: Vec2) {
        self.offset = Vec2::new(v.x - self.aspect_ratio()/2.0, v.y - 0.5);
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.px.0 as f32 / self.px.1 as f32
    }

    pub fn pick_world(&self, x: u32, y: u32) -> Vec2 {
        self.pick_screen(x, y).add(self.offset)
    }

    pub fn pick_screen(&self, x: u32, y: u32) -> Vec2 {
        Vec2::new(self.aspect_ratio() * x as f32 / self.px.0 as f32, y as f32 / self.px.1 as f32)
    }

    pub fn project_point(&self, p: Vec2) -> Vec2 {
        p.sub(self.offset)
    }

    pub fn project_rect(&self, r: Rect) -> Rect {
        Rect {
            x: r.x - self.offset.x,
            y: r.y - self.offset.y,
            w: r.w,
            h: r.h,
        }
    }

    pub fn sdl_rect(&self, r: Rect) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            (r.x * self.px.0 as f32 / self.aspect_ratio()) as i32,
            (r.y * self.px.1 as f32) as i32,
         (r.w * self.px.0 as f32 / self.aspect_ratio()) as u32,
        (r.h * self.px.1 as f32) as u32,
        )
    }
}

