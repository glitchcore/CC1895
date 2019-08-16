use crate::primitive::{Primitive, Point, Line, Ellipse};

use std::f32;

pub struct Intro {
    current_primitive: usize,
    switch_time: f32,
    phase: f32,
    line_1_0: Line,
    line_1_1: Line,

    ell_8_0: Ellipse,
    ell_8_1: Ellipse,

    ell_9_0: Ellipse,
    ell_9_1: Ellipse,

    line_5_0: Line,
    line_5_1: Line,
    ell_5_2: Ellipse,
}

impl Intro {
    pub const fn new() -> Self {
        Intro {
            current_primitive: 0,
            switch_time: 0.0,
            phase: 0.0,
            line_1_0: Line::new(Point{x:0.14, y:0.35}, Point{x:0.05, y:0.39}),
            line_1_1: Line::new(Point{x:0.14, y:0.35}, Point{x:0.14, y:0.64}),

            ell_8_0: Ellipse::new(Point{x:0.36, y:0.42}, 0.08, 0.07),
            ell_8_1: Ellipse::new(Point{x:0.36, y:0.57}, 0.085, 0.08),

            ell_9_0: Ellipse::new(Point{x:0.6, y:0.43}, 0.085, 0.087),
            ell_9_1: Ellipse::new(Point{x:0.6, y:0.49}, 0.1, 0.15),

            line_5_0: Line::new(Point{x:0.77, y:0.35}, Point{x:0.92, y:0.35}),
            line_5_1: Line::new(Point{x:0.77, y:0.35}, Point{x:0.77, y:0.48}),
            ell_5_2: Ellipse::new(Point{x:0.82, y:0.56}, 0.1, 0.09),
            // ellipse: Ellipse::new(0.5, 0.3),
        }
    }

    pub fn draw(&mut self, t: f32, fs: f32) -> (f32, f32) {
        self.phase += 1.0/fs * (500.0 /*+ 50.0 * self.current_primitive as f32*/);

        let phase = self.phase % 1.0;

        self.ell_9_0.rotate = f32::consts::PI;
        self.ell_9_1.rotate = f32::consts::PI;

        self.ell_9_1.begin = 0.9;
        self.ell_9_1.end = 0.4;

        self.ell_5_2.begin = 0.0;
        self.ell_5_2.end = 0.65;
        self.ell_5_2.rotate = 0.5;

        self.ell_8_1.rotate = f32::consts::PI;

        let primitives = [
            &self.line_1_0 as &Primitive,
            &self.line_1_1 as &Primitive,
            &self.ell_8_0 as &Primitive,
            &self.ell_8_1 as &Primitive,
            &self.ell_9_0 as &Primitive,
            &self.ell_9_1 as &Primitive,
            &self.line_5_0 as &Primitive,
            &self.line_5_1 as &Primitive,
            &self.ell_5_2 as &Primitive,
        ];

        let (x, y) = primitives[if self.current_primitive < primitives.len() {
            self.current_primitive
        } else {
            2 * primitives.len() - self.current_primitive - 1
        }]
            .draw(
                (phase * 3.1415926).sin(),
                fs
            );
        let (x,y) = (x * 2.0 - 1.0, 1.0 - y * 2.0);

        if t - self.switch_time > 1.0/500.0 && phase < 1.0 {
            self.phase = 0.0;
            // log!("sw t: {}, ph: {}, ({}, {})", t, phase, x, y);

            self.switch_time = t;

            self.current_primitive += 1;
            if self.current_primitive >= 2 * primitives.len() {
                self.current_primitive = 0;
            }
        }

        return (x, y);
    }
}