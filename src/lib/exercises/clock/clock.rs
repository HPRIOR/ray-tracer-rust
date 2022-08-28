use crate::{
    canvas::canvas::Canvas,
    exercises::shared::shared::{degrees_to_radians, save_canvas, set_pixel, Coord},
    geometry::vector::point,
    matrix::matrix::{Axis, Matrix},
};

pub fn create_clock() {
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

    save_canvas("test_clock", &canvas);
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
