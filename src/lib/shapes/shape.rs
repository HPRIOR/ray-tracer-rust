use crate::geometry::vector::Tup;

pub trait Normal{
    fn normal_at(&self, point: Tup) -> Option<Tup>;
}

