use crate::primitive::{Primitive, Point, Line, Ellipse};

use std::f32;
use crate::music::Music;

pub struct Rocket {
    current_primitive: usize,
    phase: f32,
}

impl Rocket {
    pub const fn new() -> Self {
        Rocket {
            current_primitive: 0,
            phase: 0.0,
        }
    }

    pub fn draw(&mut self, music: &mut Music, _t: f32, fs: f32) -> (f32, f32) {

        let body_lines = [
            Line::new(Point{x:0.500, y:0.930}, Point{x:0.520, y:0.830}),
            Line::new(Point{x:0.520, y:0.830}, Point{x:0.520, y:0.520}),
            Line::new(Point{x:0.520, y:0.520}, Point{x:0.550, y:0.290}),
            Line::new(Point{x:0.550, y:0.290}, Point{x:0.450, y:0.290}),
            Line::new(Point{x:0.450, y:0.290}, Point{x:0.480, y:0.520}),
            Line::new(Point{x:0.480, y:0.520}, Point{x:0.480, y:0.830}),
            Line::new(Point{x:0.480, y:0.830}, Point{x:0.500, y:0.930}),
        ];

        let freq = music.get_freq(fs);

        let phase = self.phase % 1.0;

        let (x, y) = body_lines[if self.current_primitive < body_lines.len() {
            self.current_primitive
        } else {
            2 * body_lines.len() - self.current_primitive - 1
        }]
            .draw(if self.current_primitive < body_lines.len() {phase} else {1.0 - phase}, fs);
        let (x,y) = (x * 2.0 - 1.0, y * 2.0 - 1.0);

        self.phase += 1.0/fs * freq;
        if self.phase >= 1.0 {
            self.phase = 0.0;

            self.current_primitive += 1;
            if self.current_primitive >= 2 * body_lines.len() {
                self.current_primitive = 0;
            }
        }

        return (x, y);
    }
}