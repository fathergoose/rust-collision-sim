use graphics::math::{add, dot, square_len, sub, Vec2d};
use opengl_graphics::GlGraphics;

const N_BODY: usize = 2;

pub struct App {
    pub gl: GlGraphics,
    pub bodies: [Ball; N_BODY],
}

#[derive(Clone, Copy)]
pub struct Ball {
    pub position: Vec2d<f64>,
    pub velocity: Vec2d<f64>,
    pub radius: f64,
    pub mass: f64,
}

impl Ball {
    pub fn render_coordinates(&self) -> [f64; 2] {
        [
            self.position[0] - self.radius,
            self.position[1] - self.radius,
        ]
    }
    pub fn handle_boundary_colision(&self, boundries: Vec2d) -> Ball {
        for (i, item) in boundries.iter().enumerate() {
            if self.position[i] >= item - self.radius || self.position[i] <= self.radius {
                let mut new_ball = *self;
                new_ball.velocity[i] = -self.velocity[i];
                return new_ball;
            }
        }
        *self
    }
    pub fn handle_ball_colisions(&self, other: &Ball) -> Option<(Ball, Ball)> {
        let damping = 1.0;
        let diff = sub(self.position, other.position);
        let diff_len = square_len(diff).sqrt();
        let center_seperation_len = self.radius + other.radius;
        if diff_len == 0.0 || diff_len > center_seperation_len {
            return None;
        }
        let scale = 1.0 / diff_len;
        let normalized_direction = diff.map(|d| d * scale);
        let correction_scaler = (center_seperation_len - diff_len) / 2.0;
        let pos_n_dir_sum = add(self.position, normalized_direction);

        let self_init_v = dot(self.velocity, normalized_direction);
        let other_init_v = dot(other.velocity, normalized_direction);

        let m1 = self.mass;
        let m2 = other.mass;
        let combined_mass = m1 + m2;

        let self_end_v = (m1 * self_init_v + m2 * other_init_v
            - m2 * (self_init_v - other_init_v) * damping)
            / combined_mass;
        let self_diff_v = self_end_v - self_init_v;
        let sum_v_and_normal_direction = add(self.velocity, normalized_direction);

        let other_end_v = (m1 * self_init_v + m2 * other_init_v
            - m1 * (other_init_v - self_init_v) * damping)
            / combined_mass;
        let other_diff_v = other_end_v - other_init_v;
        let other_sum_v_and_normal_direction = add(other.velocity, normalized_direction);

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
