use crate::primitive::{Primitive, Point, Line, Ellipse};

use std::f32;
use crate::music::Music;

pub struct Chip {
    current_primitive: usize,
    phase: f32,
}

impl Chip {
    pub const fn new() -> Self {
        Chip {
            current_primitive: 0,
            phase: 0.0,
        }
    }

    pub fn draw(&mut self, music: &mut Music, _t: f32, fs: f32) -> (f32, f32) {
    	const WIDTH: f32 = 0.5;
    	const HEIGHT: f32 = 0.5;
    	const CENTER_X: f32 = 0.5;
    	const CENTER_Y: f32 = 0.5;

    	const PIN_COUNT: usize = 8;
    	const PADDING: f32 = 0.05;
    	const PIN_LENGTH: f32 = 0.1;




    	let freq = music.get_freq(fs);

    	let phase = self.phase % 1.0;

    	let primitives = [
    		Line::new(
    			Point{x:CENTER_X - WIDTH/2.0, y:CENTER_Y + HEIGHT/2.0},
    			Point{x:CENTER_X + WIDTH/2.0, y:CENTER_Y + HEIGHT/2.0}
    		),
    		Line::new(
    			Point{x:CENTER_X + WIDTH/2.0, y:CENTER_Y + HEIGHT/2.0},
    			Point{x:CENTER_X + WIDTH/2.0, y:CENTER_Y - HEIGHT/2.0}
    		),
    		Line::new(
    			Point{x:CENTER_X + WIDTH/2.0, y:CENTER_Y - HEIGHT/2.0},
    			Point{x:CENTER_X - WIDTH/2.0, y:CENTER_Y - HEIGHT/2.0}
    		),
    		Line::new(
    			Point{x:CENTER_X - WIDTH/2.0, y:CENTER_Y - HEIGHT/2.0},
    			Point{x:CENTER_X - WIDTH/2.0, y:CENTER_Y + HEIGHT/2.0}
    		),
    	];

    	let (x, y) = primitives[if self.current_primitive < primitives.len() {
            self.current_primitive
        } else {
            2 * primitives.len() - self.current_primitive - 1
        }]
            .draw(if self.current_primitive < primitives.len() {phase} else {1.0 - phase}, fs);
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