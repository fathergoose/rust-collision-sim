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
    simulation: Simulation
    bodies: Vec<Ball>,
}

#[derive(Clone)]
struct Ball {
    position: Vec2d,
    velocity: Vec2d,
    radius: f64,
    mass: f64,
}

struct Wall {
    line: graph_math::Matrix2d
    
}


// NOTE: I'm sure there is a method in the graphics library for checking if two polygons are overlapping
trait Body {
    fn render_coordinates(&self) -> [f64; 2];
    fn is_coliding_with<T: Body>(&self, other_body: &T) -> bool;
}

impl Body for Ball {
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
}
fn handle_ball_colision(ball: &Ball, other: &Ball) {
    let damping = 1.0;
    let diff = graph_math::sub(ball.position, other.position);
    let diff_len = graph_math::square_len(diff).sqrt();
    let center_seperation_len = ball.radius + other.radius;
    if diff_len == 0.0 || diff_len > center_seperation_len {
        return;
    }
    let scale = 1.0 / diff_len;
    let normalized_direction = diff.map(|d| d * scale);
    let correction_scaler = (center_seperation_len - diff_len) / 2.0;
    let pos_n_dir_sum = graph_math::add(ball.position, normalized_direction);
    ball.position = [
        pos_n_dir_sum[0] * -correction_scaler,
        pos_n_dir_sum[1] * -correction_scaler,
    ];

    let ball_init_v = graph_math::dot(ball.velocity, normalized_direction);
    let other_init_v = graph_math::dot(other.velocity, normalized_direction);

    let m1 = ball.mass;
    let m2 = other.mass;
    let combined_mass = m1 + m2;

    let ball_end_v = (m1 * ball_init_v + m2 * other_init_v
        - m2 * (ball_init_v - other_init_v) * damping)
        / combined_mass;
    let ball_diff_v = ball_end_v - ball_init_v;
    let sum_v_and_normal_direction = graph_math::add(ball.velocity, normalized_direction);
    ball.velocity = [
        sum_v_and_normal_direction[0] * ball_diff_v,
        sum_v_and_normal_direction[1] * ball_diff_v,
    ];

    let other_end_v = (m1 * ball_init_v + m2 * other_init_v
        - m1 * (other_init_v - ball_init_v) * damping)
        / combined_mass;
    let other_diff_v = other_end_v - other_init_v;
    let other_sum_v_and_normal_direction = graph_math::add(other.velocity, normalized_direction);
    other.velocity = [
        other_sum_v_and_normal_direction[0] * other_diff_v,
        other_sum_v_and_normal_direction[1] * other_diff_v,
    ]
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

        let mut init_ball_states = self.bodies.clone();

        // I think about one array turning into another array of the same number
        // of elements as a "map"

        let mut i = 0;
        self.bodies = <std::vec::Vec<Ball> as Clone>::clone(&self.bodies)
            .into_iter()
            .map(|b| {
                i += 1;
                let other_bodies = &self.bodies[i..];
                other_bodies.iter().fold(b, |b1, _b2| b1)
            })
            .collect();
        // for (i, b) in self.bodies.iter_mut().enumerate() {
        //     b.position[0] += args.dt * b.velocity[0];
        //     b.position[1] += args.dt * b.velocity[1];
        //     b.handle_boundary_colision(surface);
        //     for ob in init_ball_states.iter_mut().skip(i + 1) {
        //         b.handle_ball_colision(ob);
        //     }
        // }
    }
}

fn radius_to_volume_in_l3(radius: f64) -> f64 {
    (4.0 / 3.0) * PI * radius.powf(3.0)
}

fn main() {
    // let opengl = OpenGL::V2_1;
    let opengl = OpenGL::V3_2;

    // Create Glutin window.
    let mut window: Window = WindowSettings::new("bouncing-balls", [X_MAX, Y_MAX])
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
