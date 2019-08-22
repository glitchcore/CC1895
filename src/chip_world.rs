// use crate::primitive::{/*Primitive, Point, Line, Ellipse,*/ rotate, scale, shift};
use crate::chip::Chip;
use crate::primitive::Primitive;

use std::f32;
use crate::music::Music;

pub struct ChipWorld {
    current_primitive: usize,
    phase: f32,
    chips: [Chip;2]
}

impl ChipWorld {
    pub const fn new() -> Self {
        ChipWorld {
            current_primitive: 0,
            phase: 0.0,
            chips: [Chip::new(), Chip::new()],
        }
    }

    pub fn draw(&mut self, _music: &mut Music, _t: f32, fs: f32) -> (f32, f32) {
    	// let fs = fs * 100.0;

    	let freq = 100.0; // music.get_freq(fs);

        self.chips[0].shift = (-0.3,-0.3);
        self.chips[0].scale = (0.5, 0.5);

        self.chips[1].shift = (0.3,0.3);
        self.chips[1].scale = (0.5, 0.5);

        let (x,y) = self.chips[self.current_primitive].draw(self.phase, fs);

    	let (x,y) = (x * 2.0 - 1.0, y * 2.0 - 1.0);

        self.phase += 1.0/fs * freq;
        if self.phase >= 1.0 {
            self.phase = 0.0;

            self.current_primitive += 1;
            if self.current_primitive >= self.chips.len() {
                self.current_primitive = 0;
            }
        }

        return (x, y);
    }
}