#![allow(dead_code)]


use num_traits::real::Real;

use crate::geometry::vector::Tup;


pub struct Ray<T: Real> {
    origin: Tup<T>,
    direction: Tup<T>,
}

impl<T: Real> Ray<T> {
    pub fn new(origin: Tup<T>, direction: Tup<T>) -> Self {
        Self { origin, direction }
    }
}
