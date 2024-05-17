extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::math as graph_math;
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

#[derive(Clone)]
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
    fn handle_boundary_colision(&mut self, boundries: Vec2d) {
        for (i, item) in boundries.iter().enumerate() {
            if self.position[i] >= item - self.radius || self.position[i] <= self.radius {
                self.velocity[i] = -self.velocity[i]
            }
        }
    }
    fn handle_ball_colisions(&mut self, inital_ball_props: &Vec<Ball>, self_index: usize) {
        // TODO: That's right, the logic for handling these is more complex than
        // for the boundaries. The borrow checker seems designed to disallow
        // the way I implemented this in python
        for (i, other) in inital_ball_props.iter().enumerate() {
            if i == self_index {
                break;
            }
            let diff = graph_math::sub(self.position, other.position);
            let diff_len = graph_math::square_len(diff).sqrt();
            let center_seperation_len = self.radius + other.radius;
            if diff_len == 0.0 || diff_len > center_seperation_len {
                break;
            }
            let scale = 1.0 / diff_len;
            let normalized_direction = diff.map(|d| d * scale);
            let correction_scaler = (center_seperation_len - diff_len) / 2.0;
            self.position = graph_math::scale(
                graph_math::add(self.position, normalized_direction),
                correction_scaler,
            )
        }
    }
}

/*
    render: Draw a visualization of the simulation
    update: Update the parameters of the simulation
*/
impl App {
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

        let init_ball_states = self.bodies.clone();

        for (i, b) in self.bodies.iter_mut().enumerate() {
            b.position[0] += args.dt * b.velocity[0];
            b.position[1] += args.dt * b.velocity[1];
            b.handle_boundary_colision(surface);
            b.handle_ball_colisions(&init_ball_states, i);
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
