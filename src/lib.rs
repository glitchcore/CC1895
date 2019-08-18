extern crate web_sys;

use wasm_bindgen::prelude::*;

#[allow(unused_macros)]
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

mod intro;
mod tuning;
mod music;
mod city;

struct Ctx {
    intro: intro::Intro,
    tuning: tuning::Tuning,
    music: music::Music,
    city: city::City,
}

fn process_sample(ctx: &mut Ctx, t: f32, fs: f32) -> (f32, f32) {
    if t < 60.0 {
        if t < 7.0 {
            ctx.intro.draw(t, fs)
        } else {
            ctx.tuning.draw(&mut ctx.music, t - 7.0, fs)
        }
    } else {
        ctx.city.draw(&mut ctx.music, t, fs)
    }
}

static mut CTX: Ctx = Ctx {
    intro: intro::Intro::new(),
    tuning: tuning::Tuning::new(),
    music: music::Music::new(),
    city: city::City::new(),
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
            let (x, y) = process_sample(&mut CTX, t/1.0, fs*1.0);

            BUFFER[i] = x;
            BUFFER[i + buffer_len/2] = y;
        }

        t += 1.0/fs;
    }

    return t;
}
