#![allow(dead_code)]
use std::f64::consts::PI;

use module_lib::{
    canvas::canvas::Canvas,
    colour::colour::Colour,
    geometry::vector::{point, Operations, Vector},
    matrix::matrix::{Axis, Matrix},
    projectile_test::projectile::{tick, Env, Projectile},
};

fn create_projectile_canvas(file_name: &str) {
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

fn create_clock() {
    let canvas_height = 500;
    let canvas_width = 500;

    let mut canvas = Canvas::new(canvas_width, canvas_height);
    let center = Matrix::ident()
        .translate(250.0, 250.0, 0.0)
        .mul_tup(point(0.0, 0.0, 0.0));

    let length = 100.0;
    let center_coord = Coord {
        x: center.0,
        y: center.1,
    };
    set_pixel(center_coord, &mut canvas);

    (0..12).map(|i| i as f64 * 30.0 + 15.0).for_each(|degree| {
        set_pixel(get_point(degrees_to_radians(degree), length), &mut canvas);
    });

    canvas.save(
        format!(
            "/home/harry/Code/ray-tracer-rust/resources/{}.ppm",
            "test_clock"
        )
        .as_str(),
    )
}

struct Coord {
    x: f64,
    y: f64,
}

fn get_point(radians: f64, length: f64) -> Coord {
    let coords = Matrix::ident()
        .translate(length, length, 0.0)
        .rotate(Axis::Z, radians)
        .translate(250.0, 250.0, 0.0)
        .mul_tup(point(0.0, 0.0, 0.0));
    Coord {
        x: coords.0,
        y: coords.1,
    }
}

fn set_pixel(coord: Coord, canvas: &mut Canvas) {
    canvas.set_pixel(
        coord.x as usize,
        coord.y as usize,
        Colour::new(1.0, 1.0, 1.0),
    );
}

fn degrees_to_radians(degrees: f64) -> f64 {
    (PI / 180.0) * degrees
}

fn main() {
    // create_projectile_canvas("projectile");
    create_clock();
}
