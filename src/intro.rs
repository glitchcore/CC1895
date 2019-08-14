use crate::primitive::{Primitive, Point, Line, Ellipse};

pub struct Intro {
    pub current_primitive: usize,
    pub switch_time: f32,
    pub phase: f32,
    pub test_line: [Line; 3],
    pub ellipse: Ellipse,
}

impl Intro {
	pub fn draw(&mut self, t: f32, fs: f32) -> (f32, f32) {
		self.test_line[0].rotate += 0.0001;
	    self.test_line[2].rotate += 0.00001;
	    self.test_line[1].rotate += 0.00006;

	    self.phase += 1.0/fs * (50.0/* + 110.0 * self.current_primitive as f32*/);

	    let phase = self.phase % 1.0;

	    let primitives = [
	    	&self.test_line[0] as &Primitive,
	    	&self.test_line[1] as &Primitive,
	    	&self.test_line[2] as &Primitive,
	    	&self.ellipse as &Primitive,
	    ];

	    let (x, y) = primitives[self.current_primitive]
	        .draw(
	            (phase * 3.1415926).sin(),
	            fs
	        );

	    if t - self.switch_time > 1.0 && phase < 0.002 {
	        self.phase = 0.0;
	        // log!("sw t: {}, ph: {}, ({}, {})", t, phase, x, y);

	        self.switch_time = t;

	        self.current_primitive += 1;
	        if self.current_primitive > (primitives.len() - 1) {
	            self.current_primitive = 0;
	        }
	    }

	    return (x, y);
	}
}