use std::f64::consts::PI;

use crate::{canvas::canvas::Canvas, colour::colour::Colour};

pub struct Coord {
    pub x: f64,
    pub y: f64,
}

pub fn set_pixel(coord: Coord, canvas: &mut Canvas) {
    canvas.set_pixel(
        coord.x as usize,
        coord.y as usize,
        Colour::new(1.0, 1.0, 1.0),
    );
}

pub fn save_canvas(name: &str, canvas: &Canvas) -> (){
    canvas.save(
        format!(
            "/home/harry/Code/ray-tracer-rust/resources/{}.ppm",
            name
        )
        .as_str(),
    )
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (PI / 180.0) * degrees
}
