#![allow(dead_code)]

use crate::shapes::sphere::Sphere;

#[derive(PartialEq, Debug)]
pub enum Object<'a> {
    Sphere(&'a Sphere),
}



#[derive(PartialEq, Debug)]
pub struct Intersection<'a> {
    pub at: f64,
    pub object: Object<'a>,
}

impl<'a> Intersection<'a> {
    pub fn new(at: f64, object: Object<'a>) -> Self {
        Self { at, object }
    }
}
