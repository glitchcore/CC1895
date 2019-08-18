use crate::primitive::{Primitive, Point, Line, Ellipse, scale, shift};

use std::f32;
use crate::music::Music;

pub struct City {
    current_primitive: usize,
    phase: f32,
    switch_time: f32,

    tower_lines: [Line;6],
    tower_top: Ellipse,

    tower_scale: f32,
}

impl City {
    pub const fn new() -> Self {
        City {
            current_primitive: 0,
            phase: 0.0,
            switch_time: 0.0,
            tower_lines: [
                Line::new(Point{x:0.3, y:0.1}, Point{x:0.5, y:0.9}),
                Line::new(Point{x:0.5, y:0.9}, Point{x:0.7, y:0.1}),
                Line::new(Point{x:0.7, y:0.1}, Point{x:0.35, y:0.29}),
                Line::new(Point{x:0.35, y:0.29}, Point{x:0.63, y:0.39}),
                Line::new(Point{x:0.63, y:0.39}, Point{x:0.41, y:0.52}),
                Line::new(Point{x:0.41, y:0.52}, Point{x:0.56, y:0.65}),
            ],
            tower_top: Ellipse::new(Point{x:0.5, y:0.9}, 0.05, 0.05),
            tower_scale: 1.0,
        }
    }

    pub fn draw(&mut self, music: &mut Music, t: f32, fs: f32) -> (f32, f32) {
        let freq = music.get_freq(fs);
        // let freq = 1000.0;

        self.phase += 1.0/fs * freq;

        if self.tower_scale > 0.2 {
            self.tower_scale -= 1.0/fs * 0.2;
        }

        let primitives = [
            &self.tower_lines[0] as &Primitive,
            &self.tower_top as &Primitive,
            &self.tower_lines[1] as &Primitive,
            &self.tower_lines[2] as &Primitive,
            &self.tower_lines[3] as &Primitive,
            &self.tower_lines[4] as &Primitive,
            &self.tower_lines[5] as &Primitive,
        ];

        let phase = self.phase % 1.0;

        let (x, y) = primitives[if self.current_primitive < primitives.len() {
            self.current_primitive
        } else {
            2 * primitives.len() - self.current_primitive - 1
        }]
            .draw(if self.current_primitive < primitives.len() {phase} else {1.0 - phase}, fs);
        let (x,y) = (x * 2.0 - 1.0, y * 2.0 - 1.0);

        if t - self.switch_time > 1.0/freq && phase < 1.0 {
            self.phase = 0.0;
            // log!("sw t: {}, ph: {}, ({}, {})", t, phase, x, y);

            self.switch_time = t;

            self.current_primitive += 1;
            if self.current_primitive >= 2 * primitives.len() {
                self.current_primitive = 0;
            }
        }

        let (x,y) = scale((x,y), (self.tower_scale, self.tower_scale));
        let (x,y) = shift((x,y), (-(1.0 - self.tower_scale) * 0.5, (1.0 - self.tower_scale) * 0.2));

        return (x, y);
    }
}