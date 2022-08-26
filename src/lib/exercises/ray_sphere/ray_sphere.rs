#![allow(unused_imports, unused_variables, dead_code, unused_mut)]

use crate::{
    canvas::canvas::Canvas,
    exercises::shared::shared::{set_pixel, Coord, save_canvas},
    geometry::vector::{point, vector},
    matrix::matrix::Matrix,
    ray::{
        intersection::Intersection,
        ray::{Hit, Ray},
    },
    shapes::sphere::Sphere,
};

pub fn render_sphere() {
    let sphere = Sphere::with_transform(Matrix::scaling(10.0, 10.0, 10.0));
    let (width, height) = (100, 100);

    let mut canvas = Canvas::new(width, height);

    let rays: Vec<Ray> = (0..width)
        .flat_map(move |x| {
            (0..height)
                .map(move |y| Ray::new(point(x as f64, y as f64, -5.0), vector(0.0, 0.0, 1.0)))
        })
        .collect();

    for ray in rays {
        let intersections: Option<Vec<Intersection>> = ray.intersect(&sphere);

        let hit = intersections.hit();

        if let Some(hit) = hit {
            let p = ray.position(hit.at);
            set_pixel(Coord { x: p.0, y: p.1 }, &mut canvas);
        }
    }
    save_canvas("sphere_test", &canvas);

}

#[cfg(test)]
mod tests {
    use super::render_sphere;

    #[test]
    fn test() {
        render_sphere()
    }
}
