use crate::primitive::{Primitive, Point, Line, Ellipse, scale, rotate, shift};

use core::f32;
use crate::math::Math;
use crate::music::Music;

pub struct Rocket {
    pub angle: f32,
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

            angle: 0.0,
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
            Line::new(Point{x:0.460, y:0.600}, Point{x:0.540, y:0.600}),
        ];

        let bottom_console = Ellipse::new(Point{x: 0.5, y: 0.290}, 0.180/2.0, 0.0);
        let top_console = Ellipse::new(Point{x: 0.5, y: 0.560}, 0.08/2.0, 0.0);

        let mut inner_lines: [Line; 3] = Default::default();

        let inner_lines_iter = [0.0, 0.3333, 0.6666].iter().map(|angle| {
            let begin_point = top_console.draw(angle + self.angle, fs);
            let end_point = bottom_console.draw(angle + self.angle, fs);

            Line::new(Point{x:begin_point.0, y:begin_point.1}, Point{x:end_point.0, y:end_point.1})
        });

        for (i, inner_line) in inner_lines_iter.enumerate() {
            inner_lines[i] = inner_line;
        }

        let mut primitives = inner_lines.iter().chain(body_lines.iter());

        let phase = t % 1.0;

        let current_primitive = t as usize;

        let (x, y) = (primitives.nth(current_primitive).unwrap() as &Primitive)
            .draw(phase, fs);

        let (x,y) = shift((x,y), (-0.5, -0.5));
        let (x, y) = scale((x, y), self.scale);
        let (x, y) = rotate((x, y), self.rotate);
        let (x, y) = shift((x, y), (self.shift.0 + 0.5, self.shift.1 + 0.5));

        (x, y)
    }

    fn get_size(&self) -> f32 {
        (3 + 8) as f32
    }
}