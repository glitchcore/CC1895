use crate::primitive::{Primitive, Point, Line, Ellipse, shift, rotate};

use std::f32;

use crate::music::Music;

pub struct Tuning {
    current_primitive: usize,
    switch_time: f32,
    phase: f32,
    angle: f32,

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

            line: Line::new(Point{x:0.2, y:0.0}, Point{x:0.1, y:0.0}),
            circle: Ellipse::new(Point{x:0.5, y:0.5}, 0.2, 0.2),
        }
    }

    pub fn draw(&mut self, music: &mut Music,t: f32, fs: f32) -> (f32, f32) {
        
        // 10.0 + if t < 3.0 {t * 2.0} else {if t < 6.0 {t * 300.0 - 900.0} else {1800.0 - 900.0}};

        let freq = music.get_freq(fs);
        self.phase += 1.0/fs * freq; /*+ 50.0 * self.current_primitive as f32*/;

        let phase = self.phase % 1.0;

        if music.bass_idx == 0 {
            self.angle = (music.freq_idx / 2) as f32 * 3.8;
        }

        self.angle += 0.0003 * (music.bass_idx % 5) as f32;

        self.circle.rotate = f32::consts::PI * 1.5 + self.angle;


        

        let kick_line = Line::new(Point{x:0.2, y:0.5}, Point{x:0.8, y:0.5});

        let (x, y) = if music.freq_idx % 4 != 0 || music.bass_idx < 6 {
            let primitives_len = 2;

            let primitive_idx = if self.current_primitive < primitives_len {
                self.current_primitive
            } else {
                2 * primitives_len - self.current_primitive - 1
            };

            if t - self.switch_time > 1.0/freq && phase < 1.0 {
                self.phase = 0.0;
                // log!("sw t: {}, ph: {}, ({}, {})", t, phase, x, y);

                self.switch_time = t;

                self.current_primitive += 1;
                if self.current_primitive >= 2 * primitives_len {
                    self.current_primitive = 0;
                }
            }

            match primitive_idx {
                0 => {
                    let (x,y) = self.line.draw((phase * 3.1415926).sin(), fs);
                    let (x,y) = rotate((x,y), self.angle);
                    let (x,y) = shift((x,y), (0.5, 0.5));
                    
                    (x,y)

                },
                1 => {
                    self.circle.draw((phase * 3.1415926).sin(), fs)
                },
                _ => (0.0, 0.0)
            }
        } else {
            let (x,_) = kick_line.draw(music.kick_phase.sin() - 0.5, fs);
            (x * 2.0, (self.angle + x * music.kick_freq * 0.01).sin() * 0.2 + 0.5)
        };
         

        let (x,y) = (x * 2.0 - 1.0, 1.0 - y * 2.0);

        

        return (x, y);
    }
}