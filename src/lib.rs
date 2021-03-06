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
mod space;
mod rocket;
mod chip;
mod chip_world;

struct Ctx {
    intro: intro::Intro,
    tuning: tuning::Tuning,
    music: music::Music,
    city: city::City,
    space: space::Space,
    rocket: rocket::Rocket,
    chip_world: chip_world::ChipWorld,
}

fn process_sample(ctx: &mut Ctx, t: f32, fs: f32) -> (f32, f32) {    
    match (t * 1000.0) as i32 {
        0...7000 => ctx.intro.draw(&mut ctx.music, t, fs),
        7000...14000 => ctx.tuning.draw(&mut ctx.music, t - 7.0, fs),
        14000...60000 => ctx.city.draw(&mut ctx.music, t - 14.0, fs),
        _ => (0.0, 0.0)
    }

    /*

    if false {
        ctx.space.draw(&mut ctx.music, t, fs);
    }

    if false {
        ctx.rocket.draw(&mut ctx.music, t, fs);
    }

    if false {
        (0.0, 0.0);
    }

    if true {
        ctx.chip_world.draw(&mut ctx.music, t, fs)
    } else {(0.0, 0.0)}
    */
}

static mut CTX: Ctx = Ctx {
    intro: intro::Intro::new(),
    tuning: tuning::Tuning::new(),
    music: music::Music::new(),
    city: city::City::new(),
    space: space::Space::new(),
    rocket: rocket::Rocket::new(),
    chip_world: chip_world::ChipWorld::new(),
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
