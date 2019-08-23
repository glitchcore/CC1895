use crate::primitive::{Primitive, Point, Line, scale, rotate, shift};

use std::f32;

pub struct Rocket {
    pub rotate: f32,
    pub shift: (f32, f32),
    pub scale: (f32, f32)
}

impl Rocket {
    pub const fn new() -> Self {
        Rocket {
            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }
}

impl Primitive for Rocket {
    fn draw(&self, t: f32, fs: f32) -> (f32, f32) {
        // let fs = fs * 100.0;

        let body_lines = [
            Line::new(Point{x:0.500, y:0.930}, Point{x:0.540, y:0.830}),
            Line::new(Point{x:0.540, y:0.830}, Point{x:0.540, y:0.560}),
            Line::new(Point{x:0.540, y:0.560}, Point{x:0.590, y:0.290}),
            Line::new(Point{x:0.590, y:0.290}, Point{x:0.410, y:0.290}),
            Line::new(Point{x:0.410, y:0.290}, Point{x:0.460, y:0.560}),
            Line::new(Point{x:0.460, y:0.560}, Point{x:0.460, y:0.830}),
            Line::new(Point{x:0.460, y:0.830}, Point{x:0.540, y:0.830}),
            Line::new(Point{x:0.460, y:0.830}, Point{x:0.500, y:0.930}),
        ];

        let primitives_len = body_lines.len();

        let phase = (t * primitives_len as f32) % 1.0;

        let current_primitive = (t * primitives_len  as f32) as usize;

        let (x, y) = body_lines[if current_primitive < primitives_len {
            current_primitive
        } else {
            (2 * primitives_len - current_primitive - 1) % primitives_len
        }]
            .draw(if current_primitive < primitives_len {phase} else {1.0 - phase}, fs);

        let (x,y) = shift((x,y), (-0.5, -0.5));
        let (x, y) = scale((x, y), self.scale);
        let (x, y) = rotate((x, y), self.rotate);
        let (x, y) = shift((x, y), (self.shift.0 + 0.5, self.shift.1 + 0.5));

        (x, y)
    }
}