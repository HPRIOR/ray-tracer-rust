use crate::{
    canvas::canvas::Canvas,
    colour::colour::Colour,
    exercises::shared::shared::{save_canvas, set_pixel_with_colour, Coord},
    geometry::vector::{point, vector, Operations, Vector},
    light::light::PointLight,
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Hit, Ray, TIntersection},
    shapes::{shape::Shape, sphere::Sphere},
};
use rayon::prelude::*;

pub fn render_sphere() {
    let sphere: Box<dyn Shape> = Sphere::as_trait_with_attr(
        Matrix::scaling(400.0, 400.0, 500.0).translate(500.0, 500.0, 0.0),
        Material::with_colour(Colour::new(0.5, 0.2, 1.0)),
    );
    let light = PointLight::new(point(2000.0, -2000.0, 3000.0), Colour::white());
    let (width, height) = (1000, 1000);

    let mut canvas = Canvas::new(width, height);

    let rays: Vec<Ray> = (0..width)
        .flat_map(move |x| {
            (0..height).map(move |y| {
                Ray::new(
                    point(x as f64, y as f64, -5.0),
                    vector(0.0, 0.0, 1.0).norm(),
                )
            })
        })
        .collect();

    let hit_coords: Vec<(Option<Colour>, Coord)> = rays
        .par_iter()
        .filter_map(|ray| {
            let intersections: Vec<Box<dyn TIntersection>> = ray.intersect(&sphere);
            let hit = intersections.hit();
            if let Some(hit) = hit {
                let p = ray.position(hit.at());
                let sphere = hit.object();
                let normal = sphere.normal_at(p);
                let eye = ray.direction.neg();
                let colour =
                    normal.map(|normal| sphere.material().lighting(p, &light, eye, normal));
                Some((colour, Coord { x: p.0, y: p.1 }))
            } else {
                None
            }
        })
        .collect();

    hit_coords.into_iter().for_each(|(colour_opt, coord)| {
        if let Some(colour) = colour_opt {
            set_pixel_with_colour(coord, colour, &mut canvas);
        };
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
