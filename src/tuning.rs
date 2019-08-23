use crate::primitive::{Primitive, Point, Line, Ellipse, shift, rotate};

use std::f32;

use crate::music::Music;

pub struct Tuning {
    current_primitive: usize,
    switch_time: f32,
    phase: f32,
    angle: f32,
    angle_idx: usize,

    freq_idx: usize,
    freq_switch_time: f32,

    p_infade: f32,

    line: Line,
    circle: Ellipse,
}

impl Tuning {
    pub const fn new() -> Self {
        Tuning {
            current_primitive: 0,
            switch_time: 0.0,
            phase: 0.0,
            angle: 0.0,
            angle_idx: 0,

            p_infade: 0.0,

            freq_idx: 0,
            freq_switch_time: 0.0,

            line: Line::new(Point{x:0.3, y:0.0}, Point{x:0.2, y:0.0}),
            circle: Ellipse::new(Point{x:0.5, y:0.5}, 0.3, 0.3),
        }
    }

    pub fn draw(&mut self, music: &mut Music, t: f32, fs: f32) -> (f32, f32) {
        let freq = if FREQS[self.freq_idx].0 == 0.0 {300.0} else {50.0};  // music.get_freq(fs)
        self.phase += 1.0/fs * freq;

        if self.phase > 1.0 {
            self.phase = 0.0;
        }

        let phase = self.phase;

        if t > 10.0 {
            if self.p_infade > 0.0 {
                self.p_infade -= 1.0/fs * 1.0;
            }
        } else {
            if self.p_infade < 1.0 {
                self.p_infade += 1.0/fs * 1.0;
            }
        }

        self.line.scale = (self.p_infade, 1.0);

        const ANGLES: [(f32, f32); 14] = [
            (3.0, 3.0),
            (2.0, -1.0),
            (0.0, -5.0),
            (-2.0, -2.0),
            (0.0, 4.1),
            (0.5, 1.0),
            (2.1, 4.5),
            (2.5, 0.8),
            (4.0, 4.1),
            (2.8, -4.1),
            (4.1, 4.1),
            (5.9, 5.4),
            (3.1, -2.1),
            (0.0, -4.1),
        ];

        // (freq, dur)
        const FREQS: [(f32, f32); 8] = [
            (0.0, 1.2),
            (40.0, 0.1),
            (0.0, 1.7),
            (12.0, 0.2),
            (0.0, 0.8),
            (15.0, 0.13),
            (0.0, 1.3),
            (14.0, 0.14),
        ];

        self.angle += 1.0/fs * ANGLES[self.angle_idx].1;

        let need_change = if ANGLES[self.angle_idx].1 > 0.0 {
            self.angle > ANGLES[self.angle_idx].0
        } else {
            self.angle < ANGLES[self.angle_idx].0
        };

        if need_change {
            self.angle_idx += 1;
            if self.angle_idx >= ANGLES.len() {
                self.angle_idx = 0;
            }
        }
        
        if t - self.freq_switch_time > FREQS[self.freq_idx].1 {
                self.freq_idx += 1;
                self.freq_switch_time = t;

                if self.freq_idx >= FREQS.len() {
                    self.freq_idx = 0;
                }
        }

        if self.p_infade < 0.6 {
            self.freq_idx = 0;
        }

        self.circle.rotate = f32::consts::PI * 1.5 + self.angle;

        let (x, y) = if FREQS[self.freq_idx].0 == 0.0 {
            let primitives_len = 2;

            let primitive_idx = if self.current_primitive < primitives_len {
                self.current_primitive
            } else {
                2 * primitives_len - self.current_primitive - 1
            };

            if self.phase == 0.0 {
                self.current_primitive += 1;
                if self.current_primitive >= 2 * primitives_len {
                    self.current_primitive = 0;
                }
            }

            match primitive_idx {
                0 => {
                    let (x,y) = self.line.draw((phase * f32::consts::PI).sin(), fs);
                    let (x,y) = rotate((x,y), self.angle);
                    let (x,y) = shift((x,y), (0.5, 0.5));
                    
                    (x,y)

                },
                1 => {
                    self.circle.draw((phase * f32::consts::PI).sin(), fs)
                },
                _ => (0.0, 0.0)
            }
        } else {
            (
                (phase * 2.0 * f32::consts::PI).sin() * 0.5 + 0.5,
                (phase * (FREQS[self.freq_idx].0 * 2.0 + self.angle * 10.0) * f32::consts::PI).sin() * 0.2 + 0.5
            )
        };

        let (x,y) = (x * 2.0 - 1.0, 1.0 - y * 2.0);

        return (x, y);
    }
}