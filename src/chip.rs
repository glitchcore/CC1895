use crate::primitive::{Primitive, Point, Line, Ellipse, rotate, scale, shift};

use std::f32;

pub struct Chip {
    current_primitive: usize,
    phase: f32,

    pub rotate: f32,
    pub shift: (f32, f32),
    pub scale: (f32, f32)
}

impl Chip {
    pub const fn new() -> Self {
        Chip {
            current_primitive: 0,
            phase: 0.0,
            
            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }

    pub fn draw(&mut self, freq: f32, _t: f32, fs: f32) -> (f32, f32) {
    	// let fs = fs * 100.0;

    	const WIDTH: f32 = 0.5;
    	const HEIGHT: f32 = 0.5;
    	const CENTER_X: f32 = 0.5;
    	const CENTER_Y: f32 = 0.5;

    	const PIN_COUNT: usize = 8;
    	const PADDING: f32 = 0.05;
    	const PIN_LENGTH: f32 = 0.08;

    	let phase = self.phase % 1.0;

    	let body = [
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

        let mut pins: [Line; PIN_COUNT * 4] = Default::default();

        let spacing_x = (WIDTH - PADDING) / PIN_COUNT as f32;
        let spacing_y= (HEIGHT - PADDING) / PIN_COUNT as f32;

        // top
        for i in 0..PIN_COUNT {
            pins[i + PIN_COUNT * 0] = Line::new(
                Point{
                    x:CENTER_X - WIDTH/2.0 + PADDING + i as f32 * spacing_x,
                    y:CENTER_Y + HEIGHT/2.0
                },
                Point{
                    x:CENTER_X - WIDTH/2.0 + PADDING + i as f32 * spacing_x,
                    y:CENTER_Y + HEIGHT/2.0 + PIN_LENGTH
                }
            );
        }

        // left
        for i in 0..PIN_COUNT {
            pins[i + PIN_COUNT * 1] = Line::new(
                Point{
                    x:CENTER_X + WIDTH/2.0,
                    y:CENTER_Y - HEIGHT/2.0 + (PIN_COUNT - i) as f32 * spacing_y
                },
                Point{
                    x:CENTER_X + WIDTH/2.0 + PIN_LENGTH,
                    y:CENTER_Y - HEIGHT/2.0 + (PIN_COUNT - i) as f32 * spacing_y
                }
            );
        }

        // bottom
        for i in 0..PIN_COUNT {
            pins[i + PIN_COUNT * 2] = Line::new(
                Point{
                    x:CENTER_X - WIDTH/2.0 + (PIN_COUNT - i) as f32 * spacing_x,
                    y:CENTER_Y - HEIGHT/2.0
                },
                Point{
                    x:CENTER_X - WIDTH/2.0 + (PIN_COUNT - i) as f32 * spacing_x,
                    y:CENTER_Y - HEIGHT/2.0 - PIN_LENGTH
                }
            );
        }

        // right
        for i in 0..PIN_COUNT {
            pins[i + PIN_COUNT * 3] = Line::new(
                Point{
                    x:CENTER_X - WIDTH/2.0,
                    y:CENTER_Y - HEIGHT/2.0 + PADDING + i as f32 * spacing_y
                },
                Point{
                    x:CENTER_X - WIDTH/2.0 - PIN_LENGTH,
                    y:CENTER_Y - HEIGHT/2.0  + PADDING + i as f32 * spacing_y
                }
            );
        }

        

        let mut primitives = body.iter().chain(pins.iter());

        let primitives_len = primitives.size_hint().1.unwrap();

    	let (x, y) = (primitives.nth(if self.current_primitive < primitives_len {
            self.current_primitive
        } else {
            2 * primitives_len - self.current_primitive - 1
        }).unwrap() as &Primitive)
            .draw(if self.current_primitive < primitives_len {phase} else {1.0 - phase}, fs);

        self.phase += 1.0/fs * freq;
        if self.phase >= 1.0 {
            self.phase = 0.0;

            self.current_primitive += 1;
            if self.current_primitive >= 2 * primitives_len {
                self.current_primitive = 0;
            }
        }

        let (x,y) = shift((x,y), (-0.5, -0.5));
        let (x, y) = scale((x, y), self.scale);
        let (x, y) = rotate((x, y), self.rotate);
        let (x, y) = shift((x, y), (self.shift.0 + 0.5, self.shift.1 + 0.5));

        return (x, y);
    }
}