#![allow(dead_code)]
use crate::{colour::colour::Colour, geometry::vector::{Tup, point}};

#[derive(Clone)]
pub struct PointLight {
    pub position: Tup,
    pub intensity: Colour,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            position: point(-10.0, 10.0, -10.0),
            intensity: Colour::white(),
        }
    }
}

impl PointLight {
    pub fn new(position: Tup, intensity: Colour) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
