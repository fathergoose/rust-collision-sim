#![allow(warnings)]

// WARN: This should be turrned of ASAP. I've only added it to cool down my editor while I sketch
// out the rough outline of an application.

extern crate graphics;
use glutin_window::OpenGL;
use graphics::math::Vec2d;
use graphics::{Line, Polygon};
use opengl_graphics::GlGraphics;
use piston::{Window, WindowSettings};
/// A 2D vector.
//pub type Vector2<T> = [T; 2];

struct App {
    gl: GlGraphics,
}
// Handled by glutin window?
// struct Window {}
struct Simulation {
    bodies: Vec<Particle>,
}
struct View {}
struct Enclosure {
    walls: [[i64; 2]; 2],
}
struct Particle {
    // spheres for now
    radius: u64,
    mass: u64,
    position: Vec2d<i64>,
    velocity: Vec2d<i64>,
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create Glutin window.
    let mut window: Window = WindowSettings::new("bouncing-balls", [X_MAX, Y_MAX])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
}
