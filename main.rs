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
    bodies: [Ball; N_BODY],
    enclosure: Vec2d<f64>,
}

#[derive(Clone, Copy)]
struct Ball {
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    radius: f64,
    mass: f64,
}

fn handle_boundary_colision(ball: &Ball, boundries: &Vec2d) -> Vec2d {
    for (i, item) in boundries.iter().enumerate() {
        if ball.position[i] >= item - ball.radius || ball.position[i] <= ball.radius {
            let mut new_ball = *ball;
            new_ball.velocity[i] = -ball.velocity[i];
            return new_ball.velocity;
        }
    }
    ball.velocity
}

fn handle_ball_colisions(ball_a: &Ball, ball_b: &Ball) -> [Ball; 2] {
    let damping = 1.0;
    let diff = gmath::sub(ball_a.position, ball_b.position);
    let diff_len = gmath::square_len(diff).sqrt();
    let center_seperation_len = ball_a.radius + ball_b.radius;
    if diff_len == 0.0 || diff_len > center_seperation_len {
        return [ball_a.clone(), ball_b.clone()];
    }
    let scale = 1.0 / diff_len;
    let normalized_direction = diff.map(|d| d * scale);
    let correction_scaler = (center_seperation_len - diff_len) / 2.0;
    let pos_n_dir_sum = gmath::add(ball_a.position, normalized_direction);

    let ball_a_init_v = gmath::dot(ball_a.velocity, normalized_direction);
    let ball_b_init_v = gmath::dot(ball_b.velocity, normalized_direction);

    let combined_mass = ball_a.mass + ball_b.mass;

    let ball_a_final_v = (ball_b.mass * ball_a_init_v + ball_b.mass * ball_a_init_v
        - ball_b.mass * (ball_a_init_v - ball_b_init_v) * damping)
        / combined_mass;
    let ball_a_delta_v = ball_a_final_v - ball_a_init_v;
    let ball_a_sum_v_and_normal_direction = gmath::add(ball_a.velocity, normalized_direction);

    let ball_b_final_v = (ball_a.mass * ball_a_init_v + ball_b.mass * ball_b_init_v
        - ball_a.mass * (ball_b_init_v - ball_a_init_v) * damping)
        / combined_mass;
    let ball_b_delta_v = ball_b_final_v - ball_b_init_v;
    let ball_b_sum_v_and_normal_direction = gmath::add(ball_b.velocity, normalized_direction);

    let new_ball_a = Ball {
        position: [
            pos_n_dir_sum[0] * -correction_scaler,
            pos_n_dir_sum[1] * -correction_scaler,
        ],
        velocity: [
            ball_a_sum_v_and_normal_direction[0] * ball_a_delta_v,
            ball_a_sum_v_and_normal_direction[1] * ball_a_delta_v,
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
            ball_b_sum_v_and_normal_direction[0] * ball_b_delta_v,
            ball_b_sum_v_and_normal_direction[1] * ball_b_delta_v,
        ],
        mass: ball_b.mass,
        radius: ball_b.radius,
    };
    [new_ball_a, new_other]
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
                    graphics::Transformed::trans_pos(ctx.transform, p.get_render_coordinates());
                graphics::ellipse(FG, square, transform, glg);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let bodies = &mut self.bodies;
        let enclosure = &self.enclosure;

        let balls = self.bodies;
        let mut result_ball_states: [Ball; 2] = [Ball {
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            radius: 0.0,
            mass: 0.0,
        }; 2];

        for (outer_index, outer_ball) in self.bodies.iter_mut().enumerate() {
            outer_ball.position = next_position(outer_ball, args);
            outer_ball.velocity = handle_boundary_colision(outer_ball, enclosure);

            // NOTE: Am I handling the "edge" cases of my list correctly here?
            for (j, inner_ball) in balls.iter().skip(outer_index + 1).enumerate() {
                let inner_index = j + 1;
                let results = handle_ball_colisions(outer_ball, inner_ball);
                result_ball_states[outer_index] = results[0];
                result_ball_states[inner_index] = results[1];
            }
        }
        self.bodies = result_ball_states;
    }
}

fn next_position(b: &Ball, args: &UpdateArgs) -> Vec2d<f64> {
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
                position: [14.0, 20.0],
                velocity: [100.0, 100.0],
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
