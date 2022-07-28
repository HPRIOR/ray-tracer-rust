use crate::geometry::vector::{Point, Vector};

#[derive(Clone, Copy, Debug)]
pub struct Projectile {
    pub position: Point<f32>,
    pub velocity: Vector<f32>,
}

#[derive(Clone, Copy)]
pub struct Env {
    pub gravity: Vector<f32>,
    pub wind: Vector<f32>,
}

/*
 * The position is increased by the velocity each time
 * The velocity is redueced due to wind and gravity until
 * should expect an arch
 * */
pub fn tick(env: Env, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    return Projectile { position, velocity };
}
