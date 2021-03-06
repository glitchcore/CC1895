use crate::primitive::{Primitive, Point, Line, Ellipse};

use std::f32;
use crate::music::Music;

pub struct Intro {
    current_primitive: usize,
    phase: f32,
    p_fade: f32,
    line_1_0: Line,
    line_1_1: Line,
    line_5_0: Line,
    line_5_1: Line,
}

impl Intro {
    pub const fn new() -> Self {
        Intro {
            current_primitive: 0,
            phase: 0.0,
            p_fade: 1.0,
            line_1_0: Line::new(Point{x:0.14, y:0.35}, Point{x:0.05, y:0.39}),
            line_1_1: Line::new(Point{x:0.14, y:0.35}, Point{x:0.14, y:0.64}),
            line_5_0: Line::new(Point{x:0.77, y:0.35}, Point{x:0.92, y:0.35}),
            line_5_1: Line::new(Point{x:0.77, y:0.35}, Point{x:0.77, y:0.48}),
        }
    }

    pub fn draw(&mut self, music: &mut Music, t: f32, fs: f32) -> (f32, f32) {
        let freq = if t < 3.0 {
            10.0 + t * 2.0}
        else {
            music.get_freq(fs)
        };

        if t > 6.0 {
            if self.p_fade > 0.0 {
                self.p_fade -= 1.0/fs * 3.0;
            }
        }

        self.phase += 1.0/fs * (freq /*+ 50.0 * self.current_primitive as f32*/);

        let phase = self.phase;

        const END_D: f32 = 0.3;

        let ell_8_0 = Ellipse::new(Point{
            x: 0.5 - self.p_fade * (0.5 - 0.36), y: 0.5 - self.p_fade * (0.5 - 0.42)
        }, END_D - self.p_fade * (END_D - 0.08), END_D - self.p_fade * (END_D - 0.07));

        let mut ell_8_1 = Ellipse::new(Point{
            x: 0.5 - self.p_fade * (0.5 - 0.36), y: 0.5 - self.p_fade * (0.5 -0.57)
        }, END_D - self.p_fade * (END_D - 0.085), END_D - self.p_fade * (END_D - 0.08));

        ell_8_1.rotate = f32::consts::PI;

        let mut ell_9_0 = Ellipse::new(Point{
            x: 0.5 - self.p_fade * (0.5 -0.6), y: 0.5 - self.p_fade * (0.5 - 0.43)
        }, END_D - self.p_fade * (END_D - 0.085), END_D - self.p_fade * (END_D - 0.087));
        let mut ell_9_1 = Ellipse::new(Point{
            x: 0.5 - self.p_fade * (0.5 - 0.6), y: 0.5 - self.p_fade * (0.5 - 0.49)
        }, END_D - self.p_fade * (END_D - 0.1), END_D - self.p_fade * (END_D - 0.15));

        ell_9_0.rotate = f32::consts::PI;

        ell_9_1.rotate = f32::consts::PI;
        ell_9_1.begin = 0.9;
        ell_9_1.end = 0.4;

        let mut ell_5_2 = Ellipse::new(Point{
            x: 0.5 - self.p_fade * (0.5 - 0.82), y: 0.5 - self.p_fade * (0.5 - 0.56)
        }, END_D - self.p_fade * (END_D - 0.1), END_D - self.p_fade * (END_D - 0.09));
        ell_5_2.begin = 0.0;
        ell_5_2.end = 0.65;
        ell_5_2.rotate = 0.5;

        self.line_1_0.scale = (self.p_fade,self.p_fade);
        self.line_1_1.scale = (self.p_fade,self.p_fade);
        self.line_5_0.scale = (self.p_fade,self.p_fade);
        self.line_5_1.scale = (self.p_fade,self.p_fade);

        let shift = 0.5 * (1.0 - self.p_fade);
        let shift = (shift,shift);

        self.line_1_0.shift = shift;
        self.line_1_1.shift = shift;
        self.line_5_0.shift = shift;
        self.line_5_1.shift = shift;


        let primitives = [
            &self.line_1_0 as &Primitive,
            &self.line_1_1 as &Primitive,
            &ell_8_0 as &Primitive,
            &ell_8_1 as &Primitive,
            &ell_9_0 as &Primitive,
            &ell_9_1 as &Primitive,
            &self.line_5_0 as &Primitive,
            &self.line_5_1 as &Primitive,
            &ell_5_2 as &Primitive,
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

        if self.phase > 1.0 {
            self.phase = 0.0;

            self.current_primitive += 1;
            if self.current_primitive >= 2 * primitives.len() {
                self.current_primitive = 0;
            }
        }

        return (x, y);
    }
}