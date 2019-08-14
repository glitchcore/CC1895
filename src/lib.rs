extern crate web_sys;

use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::f32;

mod primitive;
use primitive::{Primitive, Point, Line};

struct Ctx {
    current_primitive: usize,
    switch_time: f32,
    phase: f32,
    test_line: [primitive::Line; 3],
}

fn process_sample(ctx: &mut Ctx, t: f32, fs: f32) -> (f32, f32) {

    ctx.test_line[0].rotate += 0.0001;
    ctx.test_line[2].rotate += 0.00001;
    ctx.test_line[1].rotate += 0.00006;

    ctx.phase += 1.0/fs * (220.0 + 55.0 * ctx.current_primitive as f32);

    let phase = ctx.phase % 1.0;

    let (x, y) = ctx.test_line[ctx.current_primitive]
        .draw(
            (phase * 3.1415926).sin()
        );

    if t - ctx.switch_time > 0.02 && phase < 0.01 {
        ctx.phase = 0.0;
        // log!("sw t: {}, ph: {}, ({}, {})", t, phase, x, y);

        ctx.switch_time = t;

        ctx.current_primitive += 1;
        if ctx.current_primitive > 2 {
            ctx.current_primitive = 0;
        }
    }

    return (x, y);
}

static mut CTX: Ctx = Ctx { 
    current_primitive: 0,
    switch_time: 0.0,
    phase: 0.0,
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
