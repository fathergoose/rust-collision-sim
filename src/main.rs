#![allow(warnings)]

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use core::f64;
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
    enclosure: Enclosure,
}
struct View {}
struct Enclosure {
    walls: Vec2d<f64>,
}
struct Particle {
    // spheres for now
    radius: f64,
    mass: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
}

impl Particle {
    fn render_coordinates(&self) -> [f64; 2] {
        [
            self.position[0] - self.radius,
            self.position[1] - self.radius,
        ]
    }
    fn tick(&mut self, dt: f64) {
        for (i, vel) in self.velocity.into_iter().enumerate() {
            self.position[i] += (vel * dt);
        }
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, glg| {
            graphics::clear(BG, glg);

            for p in self.simulation.bodies.iter() {
                let square = graphics::rectangle::square(0.0, 0.0, p.radius * 2.0);
                let transform =
                    graphics::Transformed::trans_pos(ctx.transform, p.render_coordinates());
                graphics::ellipse(FG, square, transform, glg);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let bodies = &mut self.simulation.bodies;
        let enclosure = &self.simulation.enclosure;

        for (i, particle) in bodies.iter_mut().enumerate() {
            particle.tick(args.dt);
            enclosure.handle_boundary_colision(particle);
        }
    }
}
impl Enclosure {
    fn handle_boundary_colision(&self, particle: &mut Particle) {
        for (i, item) in self.walls.iter().enumerate() {
            if self.walls[i] >= item - particle.radius || particle.position[i] <= particle.radius {
                particle.velocity[i] = -particle.velocity[i]
            }
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
            enclosure: Enclosure {
                walls: [1000.0, 1000.0],
            },
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
