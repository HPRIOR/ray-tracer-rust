use crate::{
    canvas::canvas::Canvas,
    colour::colour::Colour,
    geometry::vector::{Operations, Vector},
};

#[derive(Clone, Copy, Debug)]
pub struct Projectile {
    pub position: (f64, f64, f64, f64),
    pub velocity: (f64, f64, f64, f64),
}

#[derive(Clone, Copy)]
pub struct Env {
    pub gravity: (f64, f64, f64, f64),
    pub wind: (f64, f64, f64, f64),
}

/*
 * The position is increased by the velocity each time
 * The velocity is reduced due to wind and gravity until
 * should expect an arch
 * */
pub fn tick(env: Env, proj: Projectile) -> Projectile {
    let position = proj.position.add(proj.velocity);
    let velocity = proj.velocity.add(env.gravity).add(env.wind);
    return Projectile { position, velocity };
}

pub fn create_projectile_canvas(file_name: &str) {
    let canvas_height = 500;
    let canvas_width = 1000;
    let mut canvas = Canvas::new(canvas_width, canvas_height);

    let mut projectile = Projectile {
        position: (0.0, 1.0, 0.0, 1.0),
        velocity: (1.0, 1.8, 0.0, 0.0).norm().mul(11.0),
    };
    let env = Env {
        gravity: (0.0, -0.1, 0.0, 0.0),
        wind: (-0.01, 0.0, 0.0, 0.0),
    };

    // get 'inverted' position to make 0,0 the bottom left of the canvas
    let mut proj_canv_position = canvas_height as i32 - projectile.position.y() as i32;
    loop {
        if projectile.position.y() <= 0.0 {
            break;
        }
        projectile = tick(env, projectile);
        canvas.set_pixel(
            projectile.position.x() as usize,
            proj_canv_position as usize,
            Colour::new(1.0, 1.0, 1.0),
        );
        proj_canv_position = canvas_height as i32 - projectile.position.y() as i32;
        println!("{:?}", projectile);
    }

    println!("saving canvas");
    canvas.save(
        format!(
            "/home/harry/Code/ray-tracer-rust/resources/{}.ppm",
            file_name
        )
        .as_str(),
    );
}
