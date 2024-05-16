extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::math::Vec2d;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
// use rand::prelude::*;

const X_MAX: u32 = 500;
const Y_MAX: u32 = 500;

pub struct App {
    gl: GlGraphics,
    bodies: Vec<Ball>,
}

struct Ball {
    position: Vec2d,
    velocity: Vec2d,
    radius: f64,
}

impl Ball {
    fn render_coordinates(&self) -> [f64; 2] {
        [
            self.position[0] - self.radius,
            self.position[1] - self.radius,
        ]
    }
}

/*
    render: Draw a visualization of the simulation
    update: Update the parameters of the simulation
*/
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, g| {
            clear(GREEN, g);

            for b in self.bodies.iter() {
                let square = rectangle::square(0.0, 0.0, b.radius * 2.0);
                let transform = c.transform.trans_pos(b.render_coordinates());
                ellipse(RED, square, transform, g);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let surface = [Into::<f64>::into(X_MAX), Into::<f64>::into(Y_MAX)];
        for b in self.bodies.iter_mut() {
            b.position[0] += args.dt * b.velocity[0];
            b.position[1] += args.dt * b.velocity[1];
            for (i, item) in surface.iter().enumerate() {
                if b.position[i] >= item - b.radius || b.position[i] <= b.radius {
                    b.velocity[i] = -b.velocity[i]
                }
            }
        }
    }
}

fn main() {
    // let opengl = OpenGL::V2_1;
    let opengl = OpenGL::V3_2;

    // Create Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [X_MAX, Y_MAX])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // let mut rng = rand::thread_rng();
    // Create new game and run
    let mut app = App {
        gl: GlGraphics::new(opengl),
        bodies: vec![Ball {
            position: [100.0, 100.0],
            // velocity: [rng.gen::<f64>() * 10.0, 0.0],
            velocity: [60.0, 50.0],
            radius: 10.0,
        }],
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
