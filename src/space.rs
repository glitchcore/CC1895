use crate::primitive::{Primitive, Point, Line, Ellipse};

use std::f32;
use crate::music::Music;

pub struct Space {
    current_primitive: usize,
    phase: f32,
    angle: f32,
}


/*
TODO
Рисовать центр корпуса четвертинками, и линии двойным штрихом
*/

impl Space {
    pub const fn new() -> Self {
        Space {
            current_primitive: 0,
            phase: 0.0,
            angle: 0.0,
        }
    }

    pub fn draw(&mut self, music: &mut Music, _t: f32, fs: f32) -> (f32, f32) {

        let body = Ellipse::new(Point{x:0.7, y:0.7}, 0.15, 0.15);

        let mut body_center = Ellipse::new(Point{x:0.7, y:0.7}, 0.1, 0.15);
        body_center.rotate = 0.5;

        let mut antennas_end = Ellipse::new(Point{x:0.2, y:0.3}, 0.2, 0.3);
        antennas_end.rotate = 0.5;

        let mut antennas: [Line; 4] = Default::default();

        let antennas_iter = [0.0, 0.25, 0.5, 0.75].iter().map(|angle| {
            let begin_point = body_center.draw(angle + self.angle, fs);
            let end_point = antennas_end.draw(angle + self.angle, fs);

            Line::new(Point{x:begin_point.0, y:begin_point.1}, Point{x:end_point.0, y:end_point.1})
        });

        for (i, antenna) in antennas_iter.enumerate() {
            antennas[i] = antenna;
        }

        let primitives = [
            &body as &Primitive,
            &body_center as &Primitive,
            &antennas[0] as &Primitive,
            &antennas[1] as &Primitive,
            &antennas[2] as &Primitive,
            &antennas[3] as &Primitive,
        ];

        let freq = music.get_freq(fs);

        let phase = self.phase % 1.0;

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

        self.angle += 0.00001;

        return (x, y);
    }
}