#![allow(dead_code)]
use crate::{geometry::vector::Tup, colour::colour::Colour};

pub struct PointLight{
    pub position: Tup,
    pub intensity: Colour
}

impl PointLight{
    pub fn new(position: Tup, intensity: Colour) -> Self{
        Self {
            position, intensity
        }
    }
}
