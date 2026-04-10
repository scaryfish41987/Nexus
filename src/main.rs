mod algorithms;
mod data;
mod config;
mod registry;
mod metadata;
mod app;
mod render;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640,500)
        .title("Algoritmos y Estructuras de Datos")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello World!",12,12,20, Color::BLACK);
    }
}

