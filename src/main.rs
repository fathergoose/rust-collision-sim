#![allow(warnings)]

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::f64::consts::PI;

use glutin_window::GlutinWindow as Window;
use graphics::math as gmath;
use graphics::math::Vec2d;
use graphics::types::Radius;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const WINDOW_SIZE: [u32; 2] = [500, 500];
const BG: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
const FG: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

struct App {
    gl: GlGraphics,
    simulation: Simulation,
}
// Handled by glutin window?
// struct Window {}
struct Simulation {
    bodies: Vec<Particle>,
}
struct View {}
struct Enclosure {
    walls: [[f64; 2]; 2],
}
struct Particle {
    // spheres for now
    radius: f64,
    mass: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, glg| {
            graphics::clear(BG, glg);
        });
        // Render Particles
    }
    fn update(&mut self, args: &UpdateArgs) {
        for body in &self.simulation.bodies {
            print!("x: {}, y: {}", body.position[0], body.position[1])
        }
    }
}

fn radius_to_volume_in_l3(rad: f64) -> f64 {
    return rad.powf(3.0) * PI * 0.75;
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("bouncing-balls", WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let radius: f64 = 20.0;
    let mut app = App {
        gl: GlGraphics::new(opengl),
        simulation: Simulation {
            bodies: vec![Particle {
                position: [140.0, 200.0],
                velocity: [120.0, 10.0],
                radius,
                mass: radius_to_volume_in_l3(radius),
            }],
        },
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
