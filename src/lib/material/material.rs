#![allow(dead_code, unused_variables)]

use std::ops::{Add, Mul};

use num_traits::Pow;

use crate::{
    colour::colour::Colour,
    geometry::vector::{Operations, Tup, Vector},
    light::light::PointLight,
};

use super::pattern::Pattern;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub colour: Colour,
}

pub struct MaterialBuilder {
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    colour: Colour,
}

impl Default for MaterialBuilder {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            colour: Colour::new(1.0, 1.0, 1.0),
        }
    }
}

impl MaterialBuilder {
    pub fn build(self) -> Material {
        Material {
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
            colour: self.colour,
        }
    }

    pub fn with_ambient(mut self, ambient: f64) -> MaterialBuilder {
        self.ambient = ambient;
        self
    }
    pub fn with_diffuse(mut self, diffuse: f64) -> MaterialBuilder {
        self.diffuse = diffuse;
        self
    }
    pub fn with_specular(mut self, specular: f64) -> MaterialBuilder {
        self.specular = specular;
        self
    }
    pub fn with_shininess(mut self, shininess: f64) -> MaterialBuilder {
        self.shininess = shininess;
        self
    }
    pub fn with_colour(mut self, colour: Colour) -> MaterialBuilder {
        self.colour = colour;
        self
    }
}

impl Material {
    pub fn builder() -> MaterialBuilder {
        MaterialBuilder::default()
    }
    pub fn new(ambient: f64, diffuse: f64, specular: f64, shininess: f64, colour: Colour) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
            colour,
        }
    }

    pub fn with_colour(colour: Colour) -> Self {
        Self {
            colour,
            ..Default::default()
        }
    }

    // phong shading model
    pub fn lighting(
        &self,
        illum_point: Tup,
        light: &PointLight,
        eye_vec: Tup,
        norm_vec: Tup,
        in_shadow: bool
    ) -> Colour {
        if in_shadow {
            return Colour::new(0.1, 0.1, 0.1);
        };
        let effective_colour = self.colour.mul(light.intensity);
        let light_v = light.position.sub(illum_point).norm();
        let ambient = effective_colour.mul(self.ambient);

        let light_dot_normal = light_v.dot(norm_vec);

        let mut diffuse = Colour::black();
        let mut specular = Colour::black();
        if light_dot_normal >= 0.0 {
            diffuse = effective_colour.mul(self.diffuse).mul(light_dot_normal);
            let reflect_v = light_v.neg().reflect(norm_vec);
            let reflect_dot_eye = reflect_v.dot(eye_vec);
            if reflect_dot_eye <= 0.0 {
                specular = Colour::black();
            } else {
                let factor = reflect_dot_eye.pow(self.shininess);
                specular = light.intensity.mul(self.specular).mul(factor);
            }
        }

        ambient.add(diffuse).add(specular)
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            colour: Colour::new(1.0, 1.0, 1.0),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        colour::colour::Colour,
        geometry::vector::{point, vector},
        light::light::PointLight,
        utils::test::ApproxEq,
    };

    use super::Material;

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));

        let sut = m.lighting(position, &light, eye_v, normal_v, false);
        sut.approx_eq(Colour::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_offset_by_45_between_light_and_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_v = vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));

        let sut = m.lighting(position, &light, eye_v, normal_v, false);
        sut.approx_eq(Colour::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_light_offset_by_45() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));

        let sut = m.lighting(position, &light, eye_v, normal_v, false);
        sut.approx_eq(Colour::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_v = vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));

        let sut = m.lighting(position, &light, eye_v, normal_v, false);
        sut.approx_eq(Colour::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_behind_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, 10.0), Colour::new(1.0, 1.0, 1.0));

        let sut = m.lighting(position, &light, eye_v, normal_v, false);
        sut.approx_eq(Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn shadow_cast() {
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let position = point(0.0, 0.0, 0.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Colour::white());
        let in_shadow = true;
        let material = Material::default();
        let result = material.lighting(position, &light, eye_v, normal_v, in_shadow);
        result.approx_eq(Colour::new(0.1, 0.1, 0.1));
    }

    // #[test]
    // fn shadow_cast() {
    //     let eye_v = vector(0.0, 0.0, -1.0);
    //     let normal_v = vector(0.0, 0.0, -1.0);
    //     let position = point(0.0, 0.0, 0.0);
    //     let light = PointLight::new(point(0.0, 0.0, -10.0), Colour::white());
    //     let in_shadow = true;
    //     let material = Material::default();
    //     let result = material.lighting(position, &light, eye_v, normal_v, in_shadow);
    //     result.approx_eq(Colour::new(0.1, 0.1, 0.1));
    // }
}
