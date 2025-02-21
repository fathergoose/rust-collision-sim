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

const X_MAX: u32 = 500;
const Y_MAX: u32 = 500;
const N_BODY: usize = 2;

pub struct App {
    gl: GlGraphics,
    bodies: [Ball; N_BODY],
}

#[derive(Clone, Copy)]
struct Ball {
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
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
    fn handle_boundary_colision(&self, boundries: Vec2d) -> Ball {
        for (i, item) in boundries.iter().enumerate() {
            if self.position[i] >= item - self.radius || self.position[i] <= self.radius {
                let mut new_ball = *self;
                new_ball.velocity[i] = -self.velocity[i];
                return new_ball;
            }
        }
        *self
    }
    fn handle_ball_colisions(&self, other: &Ball) -> Option<(Ball, Ball)> {
        let damping = 1.0;
        let diff = graph_math::sub(self.position, other.position);
        let diff_len = graph_math::square_len(diff).sqrt();
        let center_seperation_len = self.radius + other.radius;
        if diff_len == 0.0 || diff_len > center_seperation_len {
            return None;
        }
        let scale = 1.0 / diff_len;
        let normalized_direction = diff.map(|d| d * scale);
        let correction_scaler = (center_seperation_len - diff_len) / 2.0;
        let pos_n_dir_sum = graph_math::add(self.position, normalized_direction);

        let self_init_v = graph_math::dot(self.velocity, normalized_direction);
        let other_init_v = graph_math::dot(other.velocity, normalized_direction);

        let m1 = self.mass;
        let m2 = other.mass;
        let combined_mass = m1 + m2;

        let self_end_v = (m1 * self_init_v + m2 * other_init_v
            - m2 * (self_init_v - other_init_v) * damping)
            / combined_mass;
        let self_diff_v = self_end_v - self_init_v;
        let sum_v_and_normal_direction = graph_math::add(self.velocity, normalized_direction);

        let other_end_v = (m1 * self_init_v + m2 * other_init_v
            - m1 * (other_init_v - self_init_v) * damping)
            / combined_mass;
        let other_diff_v = other_end_v - other_init_v;
        let other_sum_v_and_normal_direction =
            graph_math::add(other.velocity, normalized_direction);

        let new_self = Ball {
            position: [
                pos_n_dir_sum[0] * -correction_scaler,
                pos_n_dir_sum[1] * -correction_scaler,
            ],
            velocity: [
                sum_v_and_normal_direction[0] * self_diff_v,
                sum_v_and_normal_direction[1] * self_diff_v,
            ],
            mass: self.mass,
            radius: self.radius,
        };
        let new_other = Ball {
            position: [
                pos_n_dir_sum[0] * correction_scaler,
                pos_n_dir_sum[1] * correction_scaler,
            ],
            velocity: [
                other_sum_v_and_normal_direction[0] * other_diff_v,
                other_sum_v_and_normal_direction[1] * other_diff_v,
            ],
            mass: other.mass,
            radius: other.radius,
        };
        Some((new_self, new_other))
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

        let init_ball_states = self.bodies;
        let mut result_ball_states: [Ball; 2] = [Ball {
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            radius: 0.0,
            mass: 0.0,
        }; 2];

        for (i, b) in self.bodies.iter_mut().enumerate() {
            result_ball_states[i].position = tick(b, args);

            b.handle_boundary_colision(surface);
            // let updated_ball_position = tick(b, args);
            // let result_ball_states = updated_ball.handle_boundary_colision(surface);
            for (j, ob) in init_ball_states.iter().skip(i + 1).enumerate() {
                match b.handle_ball_colisions(ob) {
                    Some((ball, other_ball)) => {
                        result_ball_states[i] = ball;
                        result_ball_states[j] = other_ball
                    }
                    None => {
                        result_ball_states[i] = *b;
                        result_ball_states[j] = *ob;
                    }
                }
            }
        }
        self.bodies = result_ball_states;
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

    let radius = 10.0;
    let mut app = App {
        gl: GlGraphics::new(opengl),
        bodies: [
            Ball {
                position: [140.0, 200.0],
                velocity: [12.0, 1.0],
                radius,
                mass: radius_to_volume_in_l3(radius),
            },
            Ball {
                position: [100.0, 100.0],
                velocity: [6.0, 5.0],
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
