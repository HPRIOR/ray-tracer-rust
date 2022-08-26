use crate::{
    canvas::canvas::Canvas,
    exercises::shared::shared::{save_canvas, set_pixel, Coord},
    geometry::vector::{point, vector},
    matrix::matrix::Matrix,
    ray::{
        intersection::Intersection,
        ray::{Hit, Ray},
    },
    shapes::sphere::Sphere,
};
use rayon::prelude::*;

pub fn render_sphere() {
    let sphere =
        Sphere::with_transform(Matrix::scaling(400.0, 400.0, 400.0).translate(500.0, 500.0, 0.0));
    let (width, height) = (1000, 1000);

    let mut canvas = Canvas::new(width, height);

    let rays: Vec<Ray> = (0..width)
        .flat_map(move |x| {
            (0..height)
                .map(move |y| Ray::new(point(x as f64, y as f64, -5.0), vector(0.0, 0.0, 1.0)))
        })
        .collect();


    let coords: Vec<Coord> = rays
        .par_iter()
        .filter_map(|ray| {
            let intersections: Option<Vec<Intersection>> = ray.intersect(&sphere);
            let hit = intersections.hit();
            if let Some(hit) = hit {
                let p = ray.position(hit.at);
                Some(Coord { x: p.0, y: p.1 })
            } else {
                None
            }
        })
        .collect();

    coords.into_iter().for_each(|coord| {
        set_pixel(coord, &mut canvas);
    });

    save_canvas("sphere_test", &canvas);
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::render_sphere;

    #[test]
    fn test() {
        // render_sphere()
    }
}
