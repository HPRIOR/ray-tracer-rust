#![allow(dead_code, unused_variables)]

use uuid::Uuid;

pub struct Sphere {
    id: Uuid,
}

impl Sphere {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}


