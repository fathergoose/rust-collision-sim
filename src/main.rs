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
    mass: f64,
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
    fn handle_ball_colisions(&mut self, inital_ball_props: &[Ball], self_index: usize) {
        let damping = 1.0;
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
            let pos_n_dir_sum = graph_math::add(self.position, normalized_direction);
            self.position = [
                pos_n_dir_sum[0] * -correction_scaler,
                pos_n_dir_sum[1] * -correction_scaler,
            ];

            let self_init_v = graph_math::dot(self.velocity, normalized_direction);
            let other_init_v = graph_math::dot(other.velocity, normalized_direction);

            let m1 = self.mass;
            let m2 = other.mass;
            let combined_mass = m1 + m2;

            let self_end_v = (m1 * self_init_v + m2 * other_init_v
                - m2 * (self_init_v - other_init_v) * damping)
                / combined_mass;

            // INFO: Pretty sure this and the line begining let other_diff_v...
            // won't be utilized the way they were in my Python implementation
            // but I'm going to keep them here untill I'm clear on the best way
            // to optimize this approach

            // let other_end_v = (m1 * self_init_v + m2 * other_init_v
            //     - m1 * (other_init_v - self_init_v) * damping)
            //     / combined_mass;
            // let other_diff_v = other_end_v - other_init_v;

            let self_diff_v = self_end_v - self_init_v;
            let sum_v_and_normal_direction = graph_math::add(self.velocity, normalized_direction);
            self.velocity = [
                sum_v_and_normal_direction[0] * self_diff_v,
                sum_v_and_normal_direction[1] * self_diff_v,
            ];
        }
    }
}

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

fn radius_to_volume_in_l3(radius: f64) -> f64 {
    (4.0 / 3.0) * PI * radius.powf(3.0)
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
    let radius = 50.0;
    let mut app = App {
        gl: GlGraphics::new(opengl),
        bodies: vec![
            Ball {
                position: [140.0, 200.0],
                // velocity: [rng.gen::<f64>() * 10.0, 0.0],
                velocity: [120.0, 10.0],
                radius,
                mass: radius_to_volume_in_l3(radius),
            },
            Ball {
                position: [100.0, 100.0],
                // velocity: [rng.gen::<f64>() * 10.0, 0.0],
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
