#![allow(unused)]
use std::f64::consts::PI;

use crate::{
    camera::camera::Camera,
    colour::colour::Colour,
    exercises::shared::shared::save_canvas,
    geometry::vector::{point, vector},
    light::light::PointLight,
    material::{
        material::Material,
        pattern::{Checker, Gradient, Ring, Stripe},
    },
    matrix::matrix::{Axis, Matrix},
    shapes::{plane::Plane, shape::TShapeBuilder, sphere::Sphere},
    world::world::World,
};

pub fn render_world(size: usize) {
    let bg_colour = Colour::new(0.5, 0.5, 0.5);

    let floor = Plane::builder()
        .with_material(
            Material::builder()
                .with_pattern(Box::new(Stripe::new(
                    Colour::new(0.5, 0.5, 0.1),
                    Colour::new(0.1, 0.6, 0.9),
                    Matrix::ident(),
                )))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflectivity(0.5)
                .build(),
        )
        .build_trait();

    let l_wall = Plane::builder()
        .with_transform(
            Matrix::ident()
                .rotate(Axis::X, PI / 2.0)
                .rotate(Axis::Y, -PI / 4.0)
                .translate(0.0, 0.0, 5.0),
        )
        .with_material(Material::builder().with_colour(bg_colour).build())
        .build_trait();

    let r_wall = Plane::builder()
        .with_transform(
            Matrix::ident()
                .rotate(Axis::X, PI / 2.0)
                .rotate(Axis::Y, PI / 4.0)
                .translate(0.0, 0.0, 5.0),
        )
        .with_material(Material::builder().with_colour(bg_colour).build())
        .build_trait();

    let middle = Sphere::builder()
        .with_transform(Matrix::ident().translate(0.33, 0.9, 0.0))
        .with_material(
            Material::builder()
                .with_colour(Colour::new(0.1, 1.0, 0.5))
                .with_diffuse(0.6)
                .with_specular(0.3)
                .with_reflectivity(0.2)
                .build(),
        )
        .build_trait();

    let right = Sphere::builder()
        .with_transform(
            Matrix::ident()
                .scale(0.5, 0.5, 0.5)
                .translate(0.5, 0.5, -0.5),
        )
        .with_material(
            Material::builder()
                .with_colour(Colour::new(0.5, 1.0, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .build(),
        )
        .build_trait();

    let left = Sphere::builder()
        .with_transform(
            Matrix::ident()
                .scale(0.4, 0.4, 0.4)
                .translate(-0.33, 0.33, -0.75),
        )
        .with_material(
            Material::builder()
                .with_colour(Colour::new(1.0, 0.8, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .build(),
        )
        .build_trait();

    let world = World::new(
        vec![floor, r_wall, l_wall, middle],
        PointLight::new(point(-10.0, 10.0, -10.0), Colour::white()),
    );

    let mut camera = Camera::new(size, size, PI / 3.0);
    camera.transform = Matrix::view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    save_canvas("world_ex_hq", &canvas)
}

#[cfg(test)]
mod tests {
    use crate::exercises::world_ex::world_ex::render_world;

    #[test]
    fn run() {
        // render_world(3000);
    }
}
