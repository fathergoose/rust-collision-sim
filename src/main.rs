extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::f64::consts::PI;

use glutin_window::GlutinWindow as Window;
use graphics::math as graph_math;
use graphics::math::Vec2d;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
mod app;

const X_MAX: u32 = 500;
const Y_MAX: u32 = 500;



impl app::App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
        const FG: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        self.gl.draw(args.viewport(), |c, g| {
            clear(BG, g);

            for b in self.bodies.iter() {
                let square = rectangle::square(0.0, 0.0, b.radius * 2.0);
                let transform = c.transform.trans_pos(b.render_coordinates());
                ellipse(FG, square, transform, g);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let surface = [Into::<f64>::into(X_MAX), Into::<f64>::into(Y_MAX)];

        let init_ball_states = self.bodies;
        let mut result_ball_states: [Ball; 2] = [Ball {
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            radius: 0.0,
            mass: 0.0,
        }; 2];

        for (i, b) in self.bodies.iter().enumerate() {
            // TODO: Tick returns the new position
            let position = tick(b, args);
            let new_ball
            b.handle_boundary_colision(surface);
            for (j, ob) in init_ball_states.iter().skip(i + 1).enumerate() {
                match b.handle_ball_colisions(ob) {
                    Some((ball, other_ball)) => {
                        result_ball_states[i] = ball;
                        result_ball_states[j + 1] = other_ball
                    }
                    None => {
                        result_ball_states[i] = *b;
                        result_ball_states[j + 1] = *ob;
                    }
                }
            }
        }
    }
}

fn tick(b: &Ball, args: &UpdateArgs) -> Vec2d<f64> {
    [
        b.position[0] + args.dt * b.velocity[0],
        b.position[1] + args.dt * b.velocity[1],
    ]
}

fn radius_to_volume_in_l3(radius: f64) -> f64 {
    (4.0 / 3.0) * PI * radius.powf(3.0)
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("bouncing-balls", [X_MAX, Y_MAX])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let radius = 50.0;
    let mut app = App {
        gl: GlGraphics::new(opengl),
        bodies: [
            Ball {
                position: [140.0, 200.0],
                velocity: [120.0, 10.0],
                radius,
                mass: radius_to_volume_in_l3(radius),
            },
            Ball {
                position: [100.0, 100.0],
                velocity: [60.0, 50.0],
                radius,
                mass: radius_to_volume_in_l3(radius),
            },
        ],
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
