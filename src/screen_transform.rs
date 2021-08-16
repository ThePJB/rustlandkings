use crate::rect::*;

#[derive(Clone, Copy, Debug)]
pub struct ScreenTransform {
    offset: (f32, f32),
    px: (u32, u32),
}

impl ScreenTransform {
    pub fn new(x: u32, y: u32) -> ScreenTransform {
        ScreenTransform{px: (x, y), offset: (0.0, 0.0)}
    }

    pub fn resize(&mut self, x: u32, y: u32) {
        self.px = (x, y);
        // maybe do stuff such that it stays centered
    }

    pub fn translate_center(&mut self, x: f32, y: f32) {
        self.offset = (x - self.aspect_ratio()/2.0, y - 0.5);
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.px.0 as f32 / self.px.1 as f32
    }

    pub fn pick_world(&self, x: u32, y: u32) -> (f32, f32) {
        let (screen_x, screen_y) = self.pick_screen(x, y);
        (screen_x + self.offset.0, screen_y + self.offset.1)
    }

    pub fn pick_screen(&self, x: u32, y: u32) -> (f32, f32) {
        (self.aspect_ratio() * x as f32 / self.px.0 as f32, y as f32 / self.px.1 as f32)
    }

    pub fn project_point(&self, p: (f32, f32)) -> (f32, f32) {
        (p.0 - self.offset.0, p.1 - self.offset.1)
    }

    pub fn project_rect(&self, r: Rect) -> Rect {
        Rect {
            x: r.x - self.offset.0,
            y: r.y - self.offset.1,
            w: r.w,
            h: r.h,
        }
    }

    pub fn sdl_rect(&self, r: Rect) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            (r.x / self.aspect_ratio() * self.px.0 as f32) as i32,
            (r.y * self.px.1 as f32) as i32,
         (r.w / self.aspect_ratio() * self.px.0 as f32) as u32,
        (r.h * self.px.1 as f32) as u32,
        )
    }
}

