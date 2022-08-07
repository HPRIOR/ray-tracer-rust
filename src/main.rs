use module_lib::{
    canvas::canvas::Canvas,
    colour::colour::Colour,
    projectile_test::projectile::{tick, Env, Projectile}, geometry::vector::{Vector, Operations},
};

fn main() {
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
    canvas.save("/Users/harryprior/Code/ray-tracer/resources/test.ppm");

}
