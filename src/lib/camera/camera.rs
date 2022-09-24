#![allow(unused)]
use crate::{
    canvas::canvas::Canvas,
    colour::colour::Colour,
    geometry::vector::{point, Operations, Vector},
    matrix::matrix::{Axis, Matrix},
    ray::ray::Ray,
    world::world::World,
};

use rayon::prelude::*;

pub struct Camera {
    h_size: usize,
    v_size: usize,
    fov: f64,
    half_width: f64,
    half_height: f64,
    pub transform: Matrix,
    px_size: f64,
}

impl Camera {
    pub fn new(h_size: usize, v_size: usize, fov: f64) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = h_size as f64 / v_size as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / h_size as f64;

        Self {
            h_size,
            v_size,
            fov,
            transform: Matrix::ident(),
            px_size: pixel_size,
            half_width,
            half_height,
        }
    }

    fn ray_for_pixel(&self, x: f64, y: f64) -> Option<Ray> {
        // offset from edge of canvas to pixel's center
        let x_offset = (x + 0.5) * self.px_size;
        let y_offset = (y + 0.5) * self.px_size;

        // untransformed coords of the pixel in world space
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let maybe_px = self
            .transform
            .inverse()
            .map(|m| m.mul_tup(point(world_x, world_y, -1.0)));

        let maybe_orig = self
            .transform
            .inverse()
            .map(|m| m.mul_tup(point(0.0, 0.0, 0.0)));

        // unwraps maybes to calculate the direction, which is used to form the ray
        maybe_px
            .and_then(|px| maybe_orig.map(|orig| px.sub(orig).norm()))
            .and_then(|dir| maybe_orig.map(|orig| Ray::new(orig, dir)))
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.h_size, self.v_size);
        let colours: Vec<Option<(usize, usize, Colour)>> = (0..self.v_size)
            .into_par_iter()
            .flat_map(|y| {
                (0..self.h_size)
                    .into_par_iter()
                    .map(|x| {
                        let ray = self.ray_for_pixel(x as f64, y as f64);
                        let result: Option<(usize, usize, Colour)> =
                            ray.map(|r| world.color_at(&r)).map(|c| (x, y, c));
                        result
                    })
                    .collect::<Vec<Option<(usize, usize, Colour)>>>()
            })
            .collect();
        colours.into_iter().flatten().for_each(|(x, y, c)| {
            canvas.set_pixel(x, y, c);
        });
        canvas
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        colour::colour::Colour,
        geometry::vector::{point, vector},
        matrix::matrix::{Axis, Matrix},
        utils::test::ApproxEq,
        world::world::World,
    };

    use super::Camera;

    #[test]
    fn default_constructor_has_corrector_fields() {
        let sut = Camera::new(160, 120, PI / 2.0);
        assert_eq!(sut.transform, Matrix::ident())
    }

    #[test]
    fn pixel_size_is_correct_for_horizontal_canvas() {
        let sut = Camera::new(200, 125, PI / 2.0);
        sut.px_size.approx_eq(0.01);
    }
    #[test]
    fn pixel_size_is_correct_for_vertical_canvas() {
        let sut = Camera::new(125, 200, PI / 2.0);
        sut.px_size.approx_eq(0.01);
    }

    #[test]
    fn construct_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let ray = c.ray_for_pixel(100.0, 50.0).unwrap();
        assert_eq!(ray.origin, point(0.0, 0.0, 0.0));
        ray.origin.approx_eq(point(0.0, 0.0, 0.0));
        ray.direction.approx_eq(vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn construct_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let ray = c.ray_for_pixel(0.0, 0.0).unwrap();
        assert_eq!(ray.origin, point(0.0, 0.0, 0.0));
        ray.direction.approx_eq(vector(0.66519, 0.33259, -0.66851))
    }

    #[test]
    fn construct_ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Matrix::ident()
            .translate(0.0, -2.0, 5.0)
            .rotate(Axis::Y, PI / 4.0);

        let ray = c.ray_for_pixel(100.0, 50.0).unwrap();
        assert_eq!(ray.origin, point(0.0, 2.0, -5.0));
        ray.direction
            .approx_eq(vector(2.0_f64.sqrt() / 2.0, 0.0, -(2.0_f64.sqrt() / 2.0)))
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.transform = Matrix::view_transform(from, to, up);
        let image = c.render(&w);
        let px = image.get_pixel(5, 5).unwrap();
        px.approx_eq(Colour::new(0.38066, 0.47583, 0.2855));
    }
}
