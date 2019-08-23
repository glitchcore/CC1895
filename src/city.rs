use crate::primitive::{Primitive, Point, Line, Ellipse, interp, scale, rotate, shift};

use std::f32;
use crate::music::Music;

struct Tower {
    tower_lines: [Line;6],

    pub rotate: f32,
    pub shift: (f32, f32),
    pub scale: (f32, f32)
}

impl Tower {
    pub const fn new() -> Self {
        Tower {
            tower_lines: [
                Line::new(Point{x:0.3, y:0.1}, Point{x:0.5, y:0.9}),
                Line::new(Point{x:0.5, y:0.9}, Point{x:0.7, y:0.1}),
                Line::new(Point{x:0.7, y:0.1}, Point{x:0.35, y:0.29}),
                Line::new(Point{x:0.35, y:0.29}, Point{x:0.63, y:0.39}),
                Line::new(Point{x:0.63, y:0.39}, Point{x:0.41, y:0.52}),
                Line::new(Point{x:0.41, y:0.52}, Point{x:0.56, y:0.65}),
            ],

            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }
}

impl Primitive for Tower {
    fn draw(&self, t: f32, fs: f32) -> (f32, f32) {

        let primitives_len = self.tower_lines.len();

        let phase = (t * primitives_len as f32) % 1.0;

        let current_primitive = (t * primitives_len as f32) as usize;

        let (x, y) = self.tower_lines[current_primitive % primitives_len].draw(phase, fs);

        let (x,y) = shift((x,y), (-0.5, -0.5));
        let (x, y) = scale((x, y), self.scale);
        let (x, y) = rotate((x, y), self.rotate);
        let (x, y) = shift((x, y), (self.shift.0 + 0.5, self.shift.1 + 0.5));

        (x, y)
    }
}

pub struct City {
    current_primitive: usize,
    phase: f32,

    p_infade: f32,

    tower: Tower,

    horizon: Line,

    tower_scale: f32,
    signal_phase: f32,
}

impl City {
    pub const fn new() -> Self {
        City {
            current_primitive: 0,
            phase: 0.0,

            p_infade: 0.0,

            tower: Tower::new(),

            horizon: Line::new(Point{x:0.0, y:0.0}, Point{x:1.0, y:0.0}),
            
            tower_scale: 1.0,
            signal_phase: 0.0,
        }
    }

    pub fn draw(&mut self, music: &mut Music, t: f32, fs: f32) -> (f32, f32) {
        let freq = music.get_freq(fs);
        // let freq = 1000.0;

        if t > 5.0 {
            if self.tower_scale > 0.0 {
                self.tower_scale -= 1.0/fs * 0.5;
            }
        }
        
        if self.p_infade < 1.0 {
            self.p_infade += 1.0/fs * 0.5;
        }

        let _p_fade = 1.0 - self.p_infade;

        self.horizon.shift = (0.0, 0.3 * self.p_infade);

        /*
        if self.p_infade > 0.5 {
            self.horizon.shift = (0.0, 0.3 - (1.0 - self.tower_scale) * 0.2);
            self.horizon.scale = (1.0/self.tower_scale, 1.0/self.tower_scale);
        } else {
            self.horizon.shift = (0.0, 0.3 - (1.0 - self.tower_scale) * 0.2);
            self.horizon.scale = (1.0/self.tower_scale, 1.0/self.tower_scale);
        }
        */

        // self.tower_signal.scale = (music.kick_freq - 100.0, music.kick_freq/100.0);

        const TOP_END: (f32, f32) = (0.2, 0.15);

        let top_end = (
            interp(0.5 - self.p_infade * TOP_END.0, 0.2, self.tower_scale),
            interp(0.5 + TOP_END.1 * self.p_infade, 0.3, self.tower_scale)
        );

        let tower_top = Ellipse::new(
            Point{
                x: top_end.0,
                y: top_end.1
            },
            (0.3 - self.p_infade * (0.3 - 0.02)) * self.tower_scale,
            (0.3 - self.p_infade * (0.3 - 0.02)) * self.tower_scale
        );

        self.tower.shift = (
            interp(0.0 - TOP_END.0 * self.p_infade, -0.3, self.tower_scale),
            interp((0.5 - 0.9) * self.p_infade * 0.5 + TOP_END.1 * self.p_infade, -0.2, self.tower_scale)
        );

        self.tower.scale = (
            self.p_infade * 0.5 * self.tower_scale,
            self.p_infade * 0.5 * self.tower_scale
        );

        let primitives = [
            &self.tower as &Primitive,
            &tower_top as &Primitive,
            &self.horizon as &Primitive,
        ];

        self.signal_phase += 1.0/fs * 2.0;
        if self.signal_phase >= 1.0 {
            self.signal_phase = 0.0;
        }

        let phase = self.phase % 1.0;

        let (x, y) = if self.signal_phase < 0.2 {
            let tower_signal = Ellipse::new(
                Point{x:top_end.0, y:top_end.1},
                tower_top.a + self.signal_phase * 2.0,
                tower_top.b + self.signal_phase * 2.0
            );

            tower_signal.draw(phase, fs)
        } else {
            primitives[if self.current_primitive < primitives.len() {
                self.current_primitive
            } else {
                2 * primitives.len() - self.current_primitive - 1
            }]
                .draw(if self.current_primitive < primitives.len() {phase} else {1.0 - phase}, fs)
        };

        let (x,y) = (x * 2.0 - 1.0, y * 2.0 - 1.0);

        self.phase += 1.0/fs * freq;
        if self.phase >= 1.0 {
            self.phase = 0.0;

            self.current_primitive += 1;
            if self.current_primitive >= 2 * primitives.len() {
                self.current_primitive = 0;
            }
        }

        return (x, y);
    }
}