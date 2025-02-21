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

const X_MAX: u32 = 500;
const Y_MAX: u32 = 500;
const N_BODY: usize = 2;

struct App {
    gl: GlGraphics,
    enclosure: Vec2d<f64>,
}

// let bodies = [Ball; N_BODY];

#[derive(Clone, Copy)]
struct Ball {
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    radius: f64,
    mass: f64,
}

fn handle_boundary_colision(ball: &Ball, boundries: &Vec2d) -> Ball {
    for (i, item) in boundries.iter().enumerate() {
        if ball.position[i] >= item - ball.radius || ball.position[i] <= ball.radius {
            let mut new_ball = *ball;
            new_ball.velocity[i] = -ball.velocity[i];
            return new_ball;
        }
    }
    *ball
}

fn handle_ball_colisions(ball_a: &Ball, ball_b: &Ball) -> (Ball, Ball) {
    let damping = 1.0;
    let diff = gmath::sub(ball_a.position, ball_b.position);
    let diff_len = gmath::square_len(diff).sqrt();
    let center_seperation_len = ball_a.radius + ball_b.radius;
    if diff_len == 0.0 || diff_len > center_seperation_len {
        return (ball_a, ball_b);
    }
    let scale = 1.0 / diff_len;
    let normalized_direction = diff.map(|d| d * scale);
    let correction_scaler = (center_seperation_len - diff_len) / 2.0;
    let pos_n_dir_sum = gmath::add(ball_a.position, normalized_direction);

    let ball_a_init_v = gmath::dot(ball_a.velocity, normalized_direction);
    let ball_b_init_v = gmath::dot(ball_b.velocity, normalized_direction);

    let combined_mass = ball_a.mass + ball_b.mass;

    let ball_a_end_v = (ball_b.mass * ball_a_init_v + ball_b.mass * ball_a_init_v
        - m2 * (ball_a_init_v - other_init_v) * damping)
        / combined_mass;
    let ball_a_diff_v = ball_a_end_v - ball_a_init_v;
    let sum_v_and_normal_direction = gmath::add(ball_a.velocity, normalized_direction);

    let ball_b_final_v = (m1 * ball_a_init_v + m2 * other_init_v
        - m1 * (other_init_v - ball_a_init_v) * damping)
        / combined_mass;
    let ball_b_delta_v = ball_b_final_v - ball_b_init_v;
    let other_sum_v_and_normal_direction = gmath::add(other.velocity, normalized_direction);

    let new_ball_a = Ball {
        position: [
            pos_n_dir_sum[0] * -correction_scaler,
            pos_n_dir_sum[1] * -correction_scaler,
        ],
        velocity: [
            sum_v_and_normal_direction[0] * ball_a_diff_v,
            sum_v_and_normal_direction[1] * ball_a_diff_v,
        ],
        mass: ball_a.mass,
        radius: ball_a.radius,
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
        mass: ball_b.mass,
        radius: ball_b.radius,
    };
    (new_ball_a, new_other)
}
// Refactor out all of the mutable self methods
impl Ball {
    fn get_render_coordinates(&self) -> [f64; 2] {
        [
            self.position[0] - self.radius,
            self.position[1] - self.radius,
        ]
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const BG: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
        const FG: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        self.gl.draw(args.viewport(), |ctx, glg| {
            graphics::clear(BG, glg);

            for p in self.bodies.iter() {
                let square = graphics::rectangle::square(0.0, 0.0, p.radius * 2.0);
                let transform =
                    graphics::Transformed::trans_pos(ctx.transform, p.render_coordinates());
                graphics::ellipse(FG, square, transform, glg);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let bodies = &mut self.bodies;
        let enclosure = &self.enclosure;

        let init_ball_states = self.bodies;
        let mut result_ball_states: [Ball; 2] = [Ball {
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            radius: 0.0,
            mass: 0.0,
        }; 2];

        for (i, b) in self.bodies.iter().enumerate() {
            tick(b, args);
            b.handle_boundary_colision(enclosure);
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
                velocity: [1.0, 1.0],
                radius,
                mass: radius_to_volume_in_l3(radius),
            },
            Ball {
                position: [100.0, 100.0],
                velocity: [0.60, 0.50],
                radius,
                mass: radius_to_volume_in_l3(radius),
            },
        ],
        enclosure: [1000.0, 1000.0],
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
