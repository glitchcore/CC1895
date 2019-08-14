use crate::primitive::{Primitive, Point, Line};

pub struct Intro {
    pub current_primitive: usize,
    pub switch_time: f32,
    pub phase: f32,
    pub test_line: [Line; 3],
}

impl Primitive for Intro {
	fn draw(&mut self, t: f32, fs: f32) -> (f32, f32) {
		self.test_line[0].rotate += 0.0001;
	    self.test_line[2].rotate += 0.00001;
	    self.test_line[1].rotate += 0.00006;

	    self.phase += 1.0/fs * (220.0 + 55.0 * self.current_primitive as f32);

	    let phase = self.phase % 1.0;

	    let (x, y) = self.test_line[self.current_primitive]
	        .draw(
	            (phase * 3.1415926).sin(),
	            fs
	        );

	    if t - self.switch_time > 0.02 && phase < 0.01 {
	        self.phase = 0.0;
	        // log!("sw t: {}, ph: {}, ({}, {})", t, phase, x, y);

	        self.switch_time = t;

	        self.current_primitive += 1;
	        if self.current_primitive > (self.test_line.len() - 1) {
	            self.current_primitive = 0;
	        }
	    }

	    return (x, y);
	}
}