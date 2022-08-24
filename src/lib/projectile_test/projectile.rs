use crate::geometry::vector::Operations;


#[derive(Clone, Copy, Debug)]
pub struct Projectile {
    pub position: (f64, f64, f64, f64),
    pub velocity: (f64,f64,f64,f64),
}

#[derive(Clone, Copy)]
pub struct Env {
    pub gravity: (f64, f64,f64,f64),
    pub wind: (f64,f64,f64,f64),
}

/*
 * The position is increased by the velocity each time
 * The velocity is redueced due to wind and gravity until
 * should expect an arch
 * */
pub fn tick(env: Env, proj: Projectile) -> Projectile {
    let position = proj.position.add(proj.velocity);
    let velocity = proj.velocity.add(env.gravity).add(env.wind);
    return Projectile { position, velocity };
}
