#![allow(warnings)]

// WARN: This should be turrned of ASAP. I've only added it to cool down my editor while I sketch
// out the rough outline of an application.

extern crate graphics;
use graphics::math::Vec2d;
use graphics::{Line, Polygon};
/// A 2D vector.
//pub type Vector2<T> = [T; 2];
struct App {}
struct Window {}
struct Simulation {}
struct SimulationView {}
struct Enclosure {
    walls: [[i64; 2]; 2],
}
struct Particle {
    // particles are spheres for now
    radius: u64,
    mass: u64, // May i define this in terms of its radius?
    position: Vec2d<i64>,
    velocity: Vec2d<i64>,
}

fn main() {}
