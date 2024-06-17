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
