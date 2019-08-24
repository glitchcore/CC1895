// use crate::primitive::{/*Primitive, Point, Line, Ellipse,*/ rotate, scale, shift};
use crate::chip::Chip;
use crate::primitive::Primitive;

use std::f32;
use crate::music::Music;

pub struct ChipWorld {
    current_primitive: usize,
    phase: f32,
    chips: [Chip;5],

    p_infage: f32,
    chip_len: usize,
    switch_time: f32,
}

impl ChipWorld {
    pub const fn new() -> Self {
        ChipWorld {
            current_primitive: 0,
            phase: 0.0,
            p_infage: 10.0,
            chip_len: 1,
            switch_time: 0.0,
            chips: [Chip::new(), Chip::new(), Chip::new(), Chip::new(), Chip::new()],
        }
    }

    pub fn draw(&mut self, _music: &mut Music, t: f32, fs: f32) -> (f32, f32) {
    	// let fs = fs * 100.0;

        if self.p_infage > 1.0 {
            self.p_infage -= 1.0/fs * 5.0;
        }

    	let freq = 100.0; // music.get_freq(fs);

        self.chips[0].shift = (-0.3,-0.3);
        self.chips[0].scale = (0.5 * self.p_infage, 0.5 * self.p_infage);

        self.chips[1].shift = (0.1,0.3);
        self.chips[1].scale = (0.5 * self.p_infage, 0.5 * self.p_infage);

        self.chips[2].shift = (0.3,0.1);
        self.chips[2].scale = (0.5 * self.p_infage, 0.5 * self.p_infage);

        self.chips[3].shift = (0.3,0.3);
        self.chips[3].scale = (0.5 * self.p_infage, 0.5 * self.p_infage);

        self.chips[4].shift = (0.6,0.7);
        self.chips[4].scale = (0.5 * self.p_infage, 0.5 * self.p_infage);

        let (x,y) = self.chips[self.current_primitive].draw(self.phase, fs);

    	let (x,y) = (x * 2.0 - 1.0, y * 2.0 - 1.0);

        if t - self.switch_time > 2.0 {
            self.switch_time = t;

            if self.chip_len < self.chips.len() {
                self.chip_len += 1;
            }
        }

        self.phase += 1.0/fs * freq;
        if self.phase >= 1.0 {
            self.phase = 0.0;

            self.current_primitive += 1;
            if self.current_primitive >= self.chip_len {
                self.current_primitive = 0;
            }
        }

        return (x, y);
    }
}