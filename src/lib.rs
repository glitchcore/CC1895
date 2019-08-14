mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, demo!");
}

use std::f32;


trait Primitive {
    fn draw(&mut self, t: f32) -> (f32, f32);
}

struct Point {
    x: f32,
    y: f32,
}

fn scale(point: (f32, f32), factor: (f32, f32)) -> (f32, f32) {
    (point.0 * factor.0, point.1 * factor.1)
}

fn shift(point: (f32, f32), vector: (f32, f32)) -> (f32, f32) {
    (point.0 + vector.0, point.1 + vector.1)
}

fn rotate(point: (f32, f32), angle: f32) -> (f32, f32) {
    (
        point.0 * angle.cos() - point.1 * angle.sin(),
        point.0 * angle.sin() + point.1 * angle.cos()
    )
}

struct Rect {
    width: f32,
    height: f32,

    rotate: f32,
    shift: (f32, f32),
    scale: (f32, f32)
}

impl Rect {
    pub const fn new(width: f32, height: f32) -> Self {
        return Rect {
            width,
            height,

            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0),
        };
    }
}

impl Primitive for Rect {
    fn draw(&mut self, t: f32) -> (f32, f32) {

        let p = t * 4.0;

        let x = -self.width / 2.0;
        let y = -self.height / 2.0;
        
        let (point_x, point_y) = match p {
            d if d >= 0.0 && d < 1.0 => (x + p * self.width, y),
            d if d >= 1.0 && d < 2.0 => (x + self.width, y + (p - 1.0) * self.height),
            d if d >= 2.0 && d < 3.0 => (x + (3.0 - p) * self.width, y + self.height),
            d if d >= 3.0 && d < 4.0 => (x,  (4.0 - p) * self.height + y),
            _ => (0.0, 0.0)
        };

        let (point_x, point_y) = scale((point_x, point_y), self.scale);
        let (point_x, point_y) = rotate((point_x, point_y), self.rotate);
        let (point_x, point_y) = shift((point_x, point_y), self.shift);


        return (point_x, point_y);
    }
}

struct Line {
    begin: Point,
    end: Point,

    rotate: f32,
    shift: (f32, f32),
    scale: (f32, f32)
}

impl Line {
    pub const fn new(begin: Point, end: Point) -> Self {
        Line {
            begin,
            end,
            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }
}

impl Primitive for Line {
    fn draw(&mut self, t: f32) -> (f32, f32) {
        let (point_x, point_y) = (
            self.begin.x + (self.end.x - self.begin.x) * t,
            self.begin.y + (self.end.y - self.begin.y) * t,
        );

        let (point_x, point_y) = scale((point_x, point_y), self.scale);
        let (point_x, point_y) = rotate((point_x, point_y), self.rotate);
        let (point_x, point_y) = shift((point_x, point_y), self.shift);

        return (point_x, point_y);
    }
}

struct Ctx {
    current_primitive: usize,
    test_line: [Line; 3],
}

fn process_sample(ctx: &mut Ctx, t: f32, fs: f32) -> (f32, f32) {

    ctx.test_line[0].rotate -= 0.00012;
    ctx.test_line[2].rotate -= 0.00001;
    ctx.test_line[1].rotate -= 0.000001;

    let (x, y) = ctx.test_line[ctx.current_primitive]
        .draw(
            ((t * (110.0 + 55.0 * ctx.current_primitive as f32)) % 1.0 * 3.1415).sin()
        );

    ctx.current_primitive = ((t * 30.0) % 3.0) as usize;

    return (x, y);
}

static mut CTX: Ctx = Ctx { 
    current_primitive: 0,
    test_line: [
        Line::new(Point{x:0.0, y:0.0}, Point{x:0.2, y:0.2}),
        Line::new(Point{x:0.0, y:0.0}, Point{x:0.4, y:0.0}),
        Line::new(Point{x:0.0, y:0.0}, Point{x:0.1, y:0.0}),
    ],

};

static mut BUFFER: [f32;8192] = [0.0;8192];

#[wasm_bindgen]
pub fn get_buffer() -> *const f32 {
    unsafe {
        BUFFER.as_ptr()
    }
}

#[wasm_bindgen]
pub fn get_buffer_len() -> usize {
    unsafe {
        BUFFER.len()
    }
}

#[wasm_bindgen]
pub fn request_frame(init_t: f32, fs: f32) -> f32 {
    let mut t = init_t;

    let buffer_len = get_buffer_len();
    for i in 0..buffer_len/2 {
        unsafe {
            let (x, y) = process_sample(&mut CTX, t, fs);

            BUFFER[i] = x;
            BUFFER[i + buffer_len/2] = y;
        }

        t += 1.0/fs;
    }

    return t;
}

