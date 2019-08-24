use crate::primitive::{Primitive, Point, Line, rotate, scale, shift};

use std::f32;

pub struct Chip {
    pub rotate: f32,
    pub shift: (f32, f32),
    pub scale: (f32, f32)
}

impl Chip {
    pub const fn new() -> Self {
            Chip {            
            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }
}

const WIDTH: f32 = 0.5;
const HEIGHT: f32 = 0.5;
const CENTER_X: f32 = 0.5;
const CENTER_Y: f32 = 0.5;

const PIN_COUNT: usize = 8;
const PADDING: f32 = 0.05;
const PIN_LENGTH: f32 = 0.08;

const BODY: [Line;4] = [
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

const SPACING_X: f32 = (WIDTH - PADDING) / PIN_COUNT as f32;
const SPACING_Y: f32 = (HEIGHT - PADDING) / PIN_COUNT as f32;

impl Primitive for Chip {
    fn draw(&self, t: f32, fs: f32) -> (f32, f32) {
    	// let fs = fs * 100.0;

        let mut pins: [Line; PIN_COUNT * 4] = Default::default();

        // top
        for i in 0..PIN_COUNT {
            pins[i + PIN_COUNT * 0] = Line::new(
                Point{
                    x:CENTER_X - WIDTH/2.0 + PADDING + i as f32 * SPACING_X,
                    y:CENTER_Y + HEIGHT/2.0
                },
                Point{
                    x:CENTER_X - WIDTH/2.0 + PADDING + i as f32 * SPACING_X,
                    y:CENTER_Y + HEIGHT/2.0 + PIN_LENGTH
                }
            );
        }

        // left
        for i in 0..PIN_COUNT {
            pins[i + PIN_COUNT * 1] = Line::new(
                Point{
                    x:CENTER_X + WIDTH/2.0,
                    y:CENTER_Y - HEIGHT/2.0 + (PIN_COUNT - i) as f32 * SPACING_Y
                },
                Point{
                    x:CENTER_X + WIDTH/2.0 + PIN_LENGTH,
                    y:CENTER_Y - HEIGHT/2.0 + (PIN_COUNT - i) as f32 * SPACING_Y
                }
            );
        }

        // bottom
        for i in 0..PIN_COUNT {
            pins[i + PIN_COUNT * 2] = Line::new(
                Point{
                    x:CENTER_X - WIDTH/2.0 + (PIN_COUNT - i) as f32 * SPACING_X,
                    y:CENTER_Y - HEIGHT/2.0
                },
                Point{
                    x:CENTER_X - WIDTH/2.0 + (PIN_COUNT - i) as f32 * SPACING_X,
                    y:CENTER_Y - HEIGHT/2.0 - PIN_LENGTH
                }
            );
        }

        // right
        for i in 0..PIN_COUNT {
            pins[i + PIN_COUNT * 3] = Line::new(
                Point{
                    x:CENTER_X - WIDTH/2.0,
                    y:CENTER_Y - HEIGHT/2.0 + PADDING + i as f32 * SPACING_Y
                },
                Point{
                    x:CENTER_X - WIDTH/2.0 - PIN_LENGTH,
                    y:CENTER_Y - HEIGHT/2.0  + PADDING + i as f32 * SPACING_Y
                }
            );
        }

        let mut primitives = BODY.iter().chain(pins.iter());

        let primitives_len = primitives.size_hint().1.unwrap();

        let phase = (t * primitives_len as f32) % 1.0;

        let current_primitive = (t * primitives_len as f32) as usize;

    	let (x, y) = (primitives.nth(if current_primitive < primitives_len {
            current_primitive
        } else {
            2 * primitives_len - current_primitive - 1
        }).unwrap() as &Primitive)
            .draw(if current_primitive < primitives_len {phase} else {1.0 - phase}, fs);

        let (x,y) = shift((x,y), (-0.5, -0.5));
        let (x, y) = scale((x, y), self.scale);
        let (x, y) = rotate((x, y), self.rotate);
        let (x, y) = shift((x, y), (self.shift.0 + 0.5, self.shift.1 + 0.5));

        return (x, y);
    }

    fn get_size(&self) -> f32 {
        (BODY.len() + PIN_COUNT * 4) as f32
    }
}